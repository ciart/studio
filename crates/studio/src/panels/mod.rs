mod canvas;
mod color_picker;
mod palette;
mod project;
mod timeline;

pub use canvas::CanvasPanel;

use gpui::{App, AppContext as _, Entity, Hsla, SharedString};
use studio_shell::{
    dock::{DockPanel, PanelContent, PanelZone},
    theme::colors,
};

pub const PROJECT: &str = "Project";
pub const CANVAS_PANEL: &str = "Canvas";
pub const TIMELINE: &str = "Timeline";
pub const COLOR_PICKER: &str = "ColorPicker";
pub const PALETTE: &str = "Palette";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PanelKind {
    Project,
    Canvas,
    Timeline,
    ColorPicker,
    Palette,
}

impl PanelKind {
    pub fn background(self) -> Hsla {
        match self {
            PanelKind::Canvas => colors().muted,
            PanelKind::Timeline => colors().secondary,
            _ => colors().surface,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            PanelKind::Project => PROJECT,
            PanelKind::Canvas => CANVAS_PANEL,
            PanelKind::Timeline => TIMELINE,
            PanelKind::ColorPicker => COLOR_PICKER,
            PanelKind::Palette => PALETTE,
        }
    }

    /// Build what this panel renders. Panels that own state get their own view
    /// entity so it survives re-renders and follows the panel between windows.
    pub fn panel(
        self,
        title: impl Into<SharedString>,
        zone: PanelZone,
        cx: &mut App,
    ) -> Entity<DockPanel> {
        let content = self.content(cx);
        DockPanel::new(self.name(), self.background(), title, zone, content, cx)
    }

    pub fn content(self, cx: &mut App) -> PanelContent {
        match self {
            PanelKind::Canvas => PanelContent::View(cx.new(CanvasPanel::new).into()),
            PanelKind::Project => PanelContent::Stateless(project::project),
            PanelKind::Timeline => PanelContent::Stateless(timeline::timeline),
            PanelKind::ColorPicker => PanelContent::Stateless(color_picker::color_picker),
            PanelKind::Palette => PanelContent::Stateless(palette::palette),
        }
    }
}
