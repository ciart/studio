#!/usr/bin/env bash

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
target_directory="${CARGO_TARGET_DIR:-"$repository_root/target"}"
if [[ "$target_directory" != /* ]]; then
    target_directory="$repository_root/$target_directory"
fi

cd "$repository_root"
mkdir -p "$target_directory"
work_directory="$(mktemp -d "$target_directory/studio-ios.XXXXXX")"
trap 'rm -rf "$work_directory"' EXIT

xcrun devicectl list devices --json-output "$work_directory/devices.json" > /dev/null
xcrun simctl list devices available -j > "$work_directory/simulators.json"
selection="$(python3 - "$work_directory/devices.json" "$work_directory/simulators.json" <<'PY'
import json
import pathlib
import re
import signal
import sys

signal.signal(signal.SIGINT, signal.SIG_DFL)
tty_in = open("/dev/tty")
tty_out = open("/dev/tty", "w")


def choose(title, options):
    for index, (label, _) in enumerate(options, 1):
        print(f"{index:2d}) {label}", file=tty_out)
    tty_out.write(f"{title} [1]: ")
    tty_out.flush()
    answer = tty_in.readline().strip() or "1"
    if not answer.isdigit() or not 1 <= int(answer) <= len(options):
        sys.exit(f"Invalid selection: {answer}")
    return options[int(answer) - 1][1]


def has_app(udid):
    applications = (
        pathlib.Path.home()
        / "Library/Developer/CoreSimulator/Devices"
        / udid
        / "data/Containers/Bundle/Application"
    )
    return any(applications.glob("*/Ciart Studio.app"))


with open(sys.argv[1]) as listing:
    devices = json.load(listing)["result"]["devices"]
devices = [
    device
    for device in devices
    if device["hardwareProperties"].get("reality") == "physical"
    and device["hardwareProperties"].get("platform") == "iOS"
    and device["connectionProperties"].get("pairingState") == "paired"
    and device["connectionProperties"].get("transportType")
]
devices.sort(
    key=lambda device: device["connectionProperties"].get("lastConnectionDate", ""),
    reverse=True,
)
options = [
    (device["deviceProperties"]["name"], ("device", device["hardwareProperties"]["udid"]))
    for device in devices
]

with open(sys.argv[2]) as listing:
    runtimes = json.load(listing)["devices"]
simulators = []
for runtime, runtime_devices in runtimes.items():
    version = re.search(r"\.iOS-(\d+)-(\d+)$", runtime)
    if not version or int(version[1]) < 26:
        continue
    for simulator in runtime_devices:
        simulators.append((simulator, (int(version[1]), int(version[2]))))
simulators.sort(key=lambda entry: (entry[0]["state"] != "Booted", entry[0]["name"], entry[1]))

others = []
for simulator, version in simulators:
    booted = simulator["state"] == "Booted"
    if booted or has_app(simulator["udid"]):
        label = f"{simulator['name']} (iOS {version[0]}.{version[1]} simulator{', booted' if booted else ''})"
        options.append((label, ("simulator", simulator["udid"])))
    else:
        others.append((simulator, version))
if not options and not others:
    sys.exit("No iOS device or simulator found")
if others:
    options.append(("Other simulators...", None))

selected = choose("Device", options)
if selected is None:
    models = sorted({simulator["name"] for simulator, _ in others})
    model = choose("Simulator", [(name, name) for name in models])
    versions = sorted(
        ((version, simulator) for simulator, version in others if simulator["name"] == model),
        key=lambda entry: entry[0],
        reverse=True,
    )
    if len(versions) == 1:
        simulator = versions[0][1]
    else:
        simulator = choose(
            "iOS", [(f"iOS {version[0]}.{version[1]}", simulator) for version, simulator in versions]
        )
    selected = ("simulator", simulator["udid"])
print("\t".join(selected))
PY
)"
IFS=$'\t' read -r kind device <<< "$selection"

if [[ "$kind" == simulator ]]; then
    rust_target="aarch64-apple-ios-sim"
    app_directory="$target_directory/ios-sim/Ciart Studio.app"
else
    rust_target="aarch64-apple-ios"
    app_directory="$target_directory/ios-device/Ciart Studio.app"
fi

IPHONEOS_DEPLOYMENT_TARGET=26.0 \
    cargo build -p studio --target "$rust_target" \
    --message-format=json-render-diagnostics "$@" > "$work_directory/build.json"

executable="$(python3 - "$work_directory/build.json" <<'PY'
import json
import sys

with open(sys.argv[1]) as output:
    executables = [
        event["executable"]
        for event in map(json.loads, output)
        if event.get("reason") == "compiler-artifact"
        and event.get("target", {}).get("name") == "studio"
        and "bin" in event.get("target", {}).get("kind", [])
        and event.get("executable")
    ]
if len(executables) != 1:
    sys.exit("Expected one studio executable in Cargo's build output")
print(executables[0])
PY
)"

generated_app="$(dirname "$executable")/Ciart Studio.app"
bundle_id="$(plutil -extract CFBundleIdentifier raw "$generated_app/Info.plist")"

rm -rf "$app_directory"
mkdir -p "$(dirname "$app_directory")"
cp -R "$generated_app" "$app_directory"
cp "$executable" "$app_directory/studio"

if [[ "$kind" == simulator ]]; then
    codesign --force --sign - "$app_directory"
    xcrun simctl boot "$device" 2>/dev/null || true
    xcrun simctl bootstatus "$device" -b
    open -a Simulator 2>/dev/null || true
    xcrun simctl install "$device" "$app_directory"
    xcrun simctl terminate "$device" "$bundle_id" 2>/dev/null || true
    xcrun simctl launch "$device" "$bundle_id"
    printf 'Installed and launched %s on %s\n' "$app_directory" "$device"
    exit 0
fi

security find-identity -v -p codesigning > "$work_directory/identities.txt"
signing="$(python3 - "$bundle_id" "$device" "$work_directory/identities.txt" <<'PY'
import datetime
import hashlib
import pathlib
import plistlib
import re
import subprocess
import sys

bundle_id, device, identities_path = sys.argv[1:]
identities = set(re.findall(r"\b([0-9A-F]{40})\b", pathlib.Path(identities_path).read_text()))
now = datetime.datetime.now(datetime.timezone.utc)
candidates = []
for directory in (
    pathlib.Path.home() / "Library/Developer/Xcode/UserData/Provisioning Profiles",
    pathlib.Path.home() / "Library/MobileDevice/Provisioning Profiles",
):
    for path in directory.glob("*.mobileprovision"):
        decoded = subprocess.run(
            ["security", "cms", "-D", "-i", str(path)], capture_output=True
        )
        if decoded.returncode != 0:
            continue
        profile = plistlib.loads(decoded.stdout)
        team = profile["TeamIdentifier"][0]
        expiration = profile["ExpirationDate"].replace(tzinfo=datetime.timezone.utc)
        if profile["Entitlements"].get("application-identifier") != f"{team}.{bundle_id}":
            continue
        if expiration <= now or device not in profile.get("ProvisionedDevices", []):
            continue
        for certificate in profile["DeveloperCertificates"]:
            fingerprint = hashlib.sha1(certificate).hexdigest().upper()
            if fingerprint in identities:
                candidates.append((expiration, path, fingerprint))
if not candidates:
    sys.exit(
        f"No provisioning profile for {bundle_id} with device {device} and a signing identity in the keychain"
    )
_, path, fingerprint = max(candidates)
print(path)
print(fingerprint)
PY
)"
profile="$(sed -n 1p <<< "$signing")"
identity="$(sed -n 2p <<< "$signing")"

security cms -D -i "$profile" > "$work_directory/profile.plist"
plutil -extract Entitlements xml1 -o "$work_directory/entitlements.plist" "$work_directory/profile.plist"

cp "$profile" "$app_directory/embedded.mobileprovision"
codesign --force --sign "$identity" --entitlements "$work_directory/entitlements.plist" "$app_directory"

xcrun devicectl device install app --device "$device" "$app_directory"
xcrun devicectl device process launch --device "$device" --terminate-existing "$bundle_id"
printf 'Installed and launched %s on %s\n' "$app_directory" "$device"
