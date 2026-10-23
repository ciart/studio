use std::collections::HashMap;

use crate::space::{DocumentVector, LayerBox, TileGridPoint};

const RASTER_TILE_SIZE: usize = 256;

pub struct Raster {
    position: DocumentVector,
    bounds: LayerBox,
    tiles: HashMap<TileGridPoint, Tile>,
}

impl Raster {
    
}

pub struct Tile {
    pixels: Box<[u8; RASTER_TILE_SIZE * RASTER_TILE_SIZE * 4]>
}
