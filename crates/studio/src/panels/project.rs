use gpui::{AnyElement, IntoElement, ParentElement, Styled, div, img, px};

use studio_shell::theme::{colors, radius};

pub fn project() -> AnyElement {
    div()
        .size_full()
        .relative()
        .child(
            img("images/sprite.png")
                .absolute()
                .left(px(17.))
                .top(px(11.))
                .size(px(64.))
                .rounded(radius())
                .border_1()
                .border_color(colors().input),
        )
        .child(
            div()
                .absolute()
                .left(px(35.))
                .top(px(80.))
                .text_color(colors().foreground)
                .child("Hello"),
        )
        .into_any_element()
}
