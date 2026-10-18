mod chip;
mod detach;
mod drag;
mod skin;
mod util;

pub use detach::apply_drop;
pub use skin::DockSkin;

use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use gpui::{
    AnyElement, AnyView, AnyWindowHandle, App, Context, Empty, Entity, EventEmitter, FocusHandle,
    Focusable, Hsla, IntoElement, ParentElement, Render, SharedString, Styled, WeakEntity, Window,
    div, prelude::*,
};
use gpui_base::dock::{DockArea, NodeId, Panel, PanelEvent, PanelId, PanelView};

use crate::theme::{colors, font, font_size, radius};

pub enum PanelContent {
    /// Stateless panels are element trees rebuilt on every render.
    Stateless(fn() -> AnyElement),
    /// Stateful panels live in their own view entity.
    View(AnyView),
}

impl PanelContent {
    pub fn body(&self) -> AnyElement {
        match self {
            PanelContent::Stateless(build) => build(),
            PanelContent::View(view) => view.clone().into_any_element(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PanelZone {
    Canvas,
    Dock,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum AreaKind {
    Main,
    Detached(PanelZone),
}

impl AreaKind {
    pub(crate) fn is_detached(self) -> bool {
        matches!(self, AreaKind::Detached(_))
    }
}

pub struct DockPanel {
    pub(crate) name: &'static str,
    pub(crate) background: Hsla,
    pub(crate) title: SharedString,
    pub(crate) zone: PanelZone,
    content: PanelContent,
    focus_handle: FocusHandle,
}

impl DockPanel {
    pub fn new(
        name: &'static str,
        background: Hsla,
        title: impl Into<SharedString>,
        zone: PanelZone,
        content: PanelContent,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            name,
            background,
            title: title.into(),
            zone,
            content,
            focus_handle: cx.focus_handle(),
        })
    }
}

impl Panel for DockPanel {
    fn panel_name(&self) -> &'static str {
        self.name
    }
}

impl EventEmitter<PanelEvent> for DockPanel {}

impl Focusable for DockPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DockPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.content.body()
    }
}

pub(crate) struct PanelDrag {
    pub(crate) panel: PanelId,
    pub(crate) source: Option<NodeId>,
    pub(crate) source_area: WeakEntity<DockArea>,
    pub(crate) source_window: AnyWindowHandle,
    pub(crate) view: Arc<dyn PanelView>,
    pub(crate) detached: RefCell<Option<(AnyWindowHandle, WeakEntity<DockArea>)>>,
    pub(crate) landed: Cell<bool>,
}

#[cfg_attr(target_os = "windows", allow(dead_code))]
pub(crate) struct DragPreview {
    pub(crate) title: SharedString,
}

impl Render for DragPreview {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(colors().surface)
            .font_family(font())
            .text_size(font_size())
            .text_color(colors().foreground)
            .border_1()
            .border_color(colors().border)
            .rounded(radius())
            .child(self.title.clone())
    }
}

pub(crate) struct HiddenPreview;

impl Render for HiddenPreview {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

pub(crate) const TAB_HEIGHT: f32 = 30.;
pub(crate) const TITLEBAR_HEIGHT: f32 = 38.;
/// Lead a detached titlebar keeps clear for the macOS traffic lights. Every
/// other platform parks its window controls on the right, so the strip there
/// starts flush against the window edge.
pub(crate) const TRAFFIC_LIGHT_PAD: f32 = if cfg!(target_os = "macos") { 78. } else { 0. };
