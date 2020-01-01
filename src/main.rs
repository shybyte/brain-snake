use raylib::prelude::*;

use crate::engine::error::SimpleResult;
use crate::engine::tile_map::TileMap;
use crate::snake::Snake;
use crate::tiles::MyTile;

mod engine;
mod snake;
mod tiles;

const GRID_SIZE: f32 = 32.0;

fn main() -> SimpleResult<()> {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Brain Snake")
        .build();

    rl.set_target_fps(60);

    let mut tile_map = TileMap::from_file("data/levels/level1.txt", MyTile::WALL)?;
    let mut snake = Snake::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        snake.update(&d, &mut tile_map);

        draw_background(&mut d, tile_map.get_width(), tile_map.get_height());

        let tile_size_factor = 0.8;
        for (y, row) in tile_map.get_data().iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                d.draw_rectangle(
                    (x as f32 * GRID_SIZE + (GRID_SIZE * (1.0 - tile_size_factor) / 2.0)) as i32,
                    (y as f32 * GRID_SIZE + (GRID_SIZE * (1.0 - tile_size_factor) / 2.0)) as i32,
                    (GRID_SIZE * tile_size_factor) as i32,
                    (GRID_SIZE * tile_size_factor) as i32,
                    match tile {
                        MyTile::SPACE => {
                            if x + y & 1 == 1 {
                                Color::DARKGRAY
                            } else {
                                Color::GRAY
                            }
                        }
                        MyTile::WALL => Color::BLACK,
                        MyTile::FOOD => Color::GREEN,
                        MyTile::SNAKE => Color::RED,
                    },
                )
            }
        }

        snake.render(&mut d, GRID_SIZE);

        //        d.draw_text("Hello, world!!", 12, 12, 20, Color::BLACK);
        d.draw_fps(10, 10)
    }

    Ok(())
}

fn draw_background(d: &mut RaylibDrawHandle, width: usize, height: usize) {
    d.clear_background(Color::BLACK);
    for x in 0..width {
        for y in 0..height {
            let color = if x + y & 1 == 1 {
                Color::DARKGRAY
            } else {
                Color::GRAY
            };
            d.draw_rectangle(
                (x as f32 * GRID_SIZE) as i32,
                (y as f32 * GRID_SIZE) as i32,
                GRID_SIZE as i32,
                GRID_SIZE as i32,
                color,
            )
        }
    }
}
