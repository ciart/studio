use euclid::{Box2D, Point2D, Vector2D};

pub struct DocumentSpace;

pub type DocumentVector = Vector2D<i32, DocumentSpace>;

pub struct LayerSpace;

pub type LayerBox = Box2D<i32, LayerSpace>;

pub struct TileGridSpace;

pub type TileGridPoint = Point2D<i32, TileGridSpace>;
