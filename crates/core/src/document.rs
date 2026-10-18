use std::sync::Arc;

const TILE_SIZE: usize = 256;

pub struct Document {
    title: String,
    layers: Vec<Layer>,
}

pub struct Layer {
    name: String,
    is_visible: bool,
    is_locked: bool,
    kind: LayerKind,
}

pub enum LayerKind {
    Raster(Raster),
}

struct Raster {
    width: u32,
    height: u32,
    pixels: [u8; TILE_SIZE * TILE_SIZE * 4],
}
