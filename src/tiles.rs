use crate::engine::error::{SimpleError, SimpleResult};
use crate::engine::tile_map::{Tile, TileMap};

pub type MyMap = TileMap<MyTile>;

#[derive(Clone, Copy, PartialEq)]
pub enum MyTile {
    SPACE,
    WALL,
    FOOD,
    SNAKE,
}

impl Tile for MyTile {
    fn from_char(c: char) -> SimpleResult<Self> {
        match c {
            ' ' => Ok(MyTile::SPACE),
            '#' => Ok(MyTile::WALL),
            '1' => Ok(MyTile::FOOD),
            'S' => Ok(MyTile::SNAKE),
            _ => Err(SimpleError::new(format!("Unknown tile {}", c).as_str())),
        }
    }
}
