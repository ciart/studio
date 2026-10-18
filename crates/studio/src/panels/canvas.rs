use gpui::prelude::FluentBuilder as _;
use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, Window, div, img, px, white,
};

use studio_shell::{
    theme::{colors, radius},
    widgets::icon,
};

const ZOOM_STEPS: [f32; 5] = [0.5, 1., 2., 4., 8.];

pub struct CanvasPanel {
    zoom: f32,
    playing: bool,
}

impl CanvasPanel {
    pub fn new(_: &mut Context<Self>) -> Self {
        Self {
            zoom: 1.,
            playing: false,
        }
    }

    fn toggle_play(&mut self, cx: &mut Context<Self>) {
        self.playing = !self.playing;
        cx.notify();
    }

    fn cycle_zoom(&mut self, cx: &mut Context<Self>) {
        let next = ZOOM_STEPS
            .iter()
            .position(|step| *step > self.zoom)
            .unwrap_or(0);
        self.zoom = ZOOM_STEPS[next];
        cx.notify();
    }
}

impl Render for CanvasPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sprite = px(256. * self.zoom);

        div()
            .size_full()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .size(sprite)
                    .bg(white())
                    .child(img("images/sprite.png").size(sprite)),
            )
            .child(
                div()
                    .id("canvas-play")
                    .absolute()
                    .left(px(11.))
                    .bottom(px(10.))
                    .size(px(24.))
                    .rounded(radius())
                    .border_1()
                    .border_color(colors().input)
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(self.playing, |this| this.bg(colors().primary))
                    .child(icon("icons/play_arrow.svg", 16., 16., colors().foreground))
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_play(cx))),
            )
            .child(
                div()
                    .id("canvas-zoom")
                    .absolute()
                    .right(px(9.))
                    .bottom(px(10.))
                    .w(px(56.))
                    .h(px(24.))
                    .rounded(radius())
                    .border_1()
                    .border_color(colors().input)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(colors().foreground)
                    .child(format!("{}%", (self.zoom * 100.) as i32))
                    .on_click(cx.listener(|this, _, _, cx| this.cycle_zoom(cx))),
            )
    }
}
