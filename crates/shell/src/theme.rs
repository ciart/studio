use std::sync::LazyLock;

use gpui::{App, Hsla, Pixels, SharedString, px, rgb, rgba, transparent_black};
use gpui_base::{ColorTokens, RadiusTokens, SemanticThemeTokens, Theme, ThemeAppearance};

pub const GAP: f32 = 1.;

static TOKENS: LazyLock<SemanticThemeTokens> = LazyLock::new(|| {
    let mut colors = ColorTokens::dark();
    colors.background = rgb(0x000000).into();
    colors.foreground = rgba(0xffffffe5).into();
    colors.surface = rgb(0x26262b).into();
    colors.surface_foreground = rgba(0xffffffe5).into();
    colors.primary = rgb(0x5089f0).into();
    colors.primary_foreground = rgba(0xffffffe5).into();
    colors.secondary = rgb(0x35353b).into();
    colors.secondary_foreground = rgba(0xffffffe5).into();
    colors.muted = rgb(0x1a1a1e).into();
    colors.muted_foreground = rgba(0xffffff99).into();
    colors.accent = rgb(0x45454b).into();
    colors.accent_foreground = rgba(0xffffffe5).into();
    colors.border = rgba(0xffffff1a).into();
    colors.input = rgba(0xffffff33).into();
    colors.ring = rgb(0x5089f0).into();

    let mut radius = RadiusTokens::default();
    radius.lg = px(10.);

    let mut tokens = SemanticThemeTokens::default();
    tokens.colors = colors;
    tokens.radius = radius;
    tokens
});

pub fn tokens() -> &'static SemanticThemeTokens {
    &TOKENS
}

pub fn colors() -> &'static ColorTokens {
    &TOKENS.colors
}

pub fn radius() -> Pixels {
    TOKENS.radius.lg
}

pub fn font() -> SharedString {
    TOKENS.typography.sans.clone()
}

pub fn font_size() -> Pixels {
    TOKENS.typography.xs.size
}

pub fn drop_target() -> Hsla {
    TOKENS.colors.primary.alpha(0.2)
}

// Panel gaps are backdrop, not dividers: Base must not paint resize handles.
pub fn install(cx: &mut App) {
    let theme = Theme::global_mut(cx);
    theme.appearance = ThemeAppearance::Dark;
    theme.tokens = TOKENS.clone();
    theme.resizable.handle = Some(transparent_black());
    theme.resizable.active_handle = Some(transparent_black());
}
