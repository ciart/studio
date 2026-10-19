use std::sync::LazyLock;

use gpui::{App, Hsla, Pixels, SharedString, px, rgb, rgba, transparent_black};
use gpui_base::{
    ColorTokens, RadiusTokens, ResizableTheme, SemanticThemeTokens, Theme, ThemeAppearance,
};

pub const GAP: f32 = 1.;

static TOKENS: LazyLock<SemanticThemeTokens> = LazyLock::new(|| SemanticThemeTokens {
    colors: ColorTokens {
        background: rgb(0x000000).into(),
        foreground: rgba(0xffffffe5).into(),
        surface: rgb(0x26262b).into(),
        surface_foreground: rgba(0xffffffe5).into(),
        primary: rgb(0x5089f0).into(),
        primary_foreground: rgba(0xffffffe5).into(),
        secondary: rgb(0x35353b).into(),
        secondary_foreground: rgba(0xffffffe5).into(),
        muted: rgb(0x1a1a1e).into(),
        muted_foreground: rgba(0xffffff99).into(),
        accent: rgb(0x45454b).into(),
        accent_foreground: rgba(0xffffffe5).into(),
        border: rgba(0xffffff1a).into(),
        input: rgba(0xffffff33).into(),
        ring: rgb(0x5089f0).into(),
        ..ColorTokens::dark()
    },
    radius: RadiusTokens {
        lg: px(10.),
        ..Default::default()
    },
    ..Default::default()
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
    theme.resizable = ResizableTheme {
        handle: Some(transparent_black()),
        active_handle: Some(transparent_black()),
    };
}
