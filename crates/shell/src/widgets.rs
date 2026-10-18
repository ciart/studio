use gpui::{Hsla, IntoElement, ParentElement, SharedString, Styled, div, px, svg};

use crate::theme::colors;

pub fn icon(path: &'static str, width: f32, height: f32, color: Hsla) -> impl IntoElement {
    svg()
        .path(path)
        .w(px(width))
        .h(px(height))
        .flex_none()
        .text_color(color)
}

pub fn empty_hint(text: impl Into<SharedString>) -> impl IntoElement {
    div()
        .text_color(colors().muted_foreground)
        .child(text.into())
}
