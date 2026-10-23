use crate::layer::raster::Raster;

pub mod raster;

pub struct Layer {
    name: String,
    is_visible: bool,
    is_locked: bool,
    kind: LayerKind,
}

pub enum LayerKind {
    Raster(Raster),
}
