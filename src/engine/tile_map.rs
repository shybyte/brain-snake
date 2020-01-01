use std::fs;
use std::path::Path;

use crate::engine::common::{GridPosition};
use crate::engine::error::SimpleResult;

pub trait Tile: Sized + Copy {
    fn from_char(c: char) -> SimpleResult<Self>;
}

pub struct TileMap<T: Tile> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
    default_tile: T,
}

impl<T: Tile> TileMap<T> {
    pub fn from_file<P: AsRef<Path>>(path: P, default_tile: T) -> SimpleResult<Self> {
        let file_content: String = fs::read_to_string(path)?.parse()?;
        let data_result: SimpleResult<Vec<Vec<T>>> = file_content
            .lines()
            .map(|line| line.chars().map(T::from_char).collect())
            .collect();
        let data = data_result?;
        Ok(TileMap {
            width: data.iter().map(Vec::len).max().unwrap_or_default(),
            height: data.len(),
            data,
            default_tile,
        })
    }

    pub fn get_tile(&self, pos: GridPosition) -> T {
        if pos.x < 0 || pos.y < 0 {
            return self.default_tile;
        }
        match &self.data.get(pos.y as usize) {
            Some(row) => *row.get(pos.x as usize).unwrap_or(&self.default_tile),
            None => self.default_tile,
        }
    }

    pub fn set_tile(&mut self, pos: GridPosition, tile: T) {
        self.data[pos.y as usize][pos.x as usize] = tile;
    }

    pub fn get_data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }
}
