use raylib::prelude::*;

use crate::engine::common::{Direction, GridCoordinate, GridPosition, Position};
use crate::engine::utils::get_cursor_key;
use crate::tiles::{MyMap, MyTile};

const VELOCITY: f32 = 5.0;

#[derive(Clone)]
pub struct SnakeSegment {
    start: GridPosition,
    pos: Position,
}

pub struct Snake {
    segments: Vec<SnakeSegment>,
    head_target_pos: Option<GridPosition>,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            segments: vec![SnakeSegment {
                start: GridPosition { x: 1, y: 1 },
                pos: Position { x: 1.0, y: 1.0 },
            }],
            head_target_pos: None,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, tile_map: &mut MyMap) {
        if self.head_target_pos == None {
            if let Some(ref nd) = get_cursor_key(rl) {
                let head = &mut self.segments[0];
                let x_grid = head.pos.x as GridCoordinate;
                let y_grid = head.pos.y as GridCoordinate;
                let new_head_target_pos = match nd {
                    Direction::Left => GridPosition {
                        x: x_grid - 1,
                        y: y_grid,
                    },
                    Direction::Right => GridPosition {
                        x: x_grid + 1,
                        y: y_grid,
                    },
                    Direction::Up => GridPosition {
                        x: x_grid,
                        y: y_grid - 1,
                    },
                    Direction::Down => GridPosition {
                        x: x_grid,
                        y: y_grid + 1,
                    },
                };

                let head_pos_tile = tile_map.get_tile(new_head_target_pos);
                if head_pos_tile != MyTile::WALL && !self.bites_itself(new_head_target_pos) {
                    self.head_target_pos = Some(new_head_target_pos);

                    for segment in self.segments.iter_mut() {
                        segment.start = GridPosition {
                            x: segment.pos.x.round() as GridCoordinate,
                            y: segment.pos.y.round() as GridCoordinate,
                        };
                    }

                    if head_pos_tile == MyTile::FOOD {
                        tile_map.set_tile(new_head_target_pos, MyTile::SPACE);
                        self.segments.push(self.segments.last().unwrap().clone());
                    }
                }
            };
        }

        let mut movement_finished = false;
        if let Some(head_target_pos) = self.head_target_pos {
            let frame_time = rl.get_frame_time();
            let frame_velocity = VELOCITY * frame_time;
            let head = &mut self.segments[0];
            let dx = head_target_pos.x - head.start.x;
            let new_pos_x = head.pos.x + (dx) as f32 * frame_velocity;
            let dy = head_target_pos.y - head.start.y;
            let new_pos_y = head.pos.y + (dy) as f32 * frame_velocity;
            head.pos.x = new_pos_x;
            head.pos.y = new_pos_y;

            movement_finished = dx == -1 && new_pos_x <= head_target_pos.x as f32
                || dx == 1 && new_pos_x >= head_target_pos.x as f32
                || dy == -1 && new_pos_y <= head_target_pos.y as f32
                || dy == 1 && new_pos_y >= head_target_pos.y as f32;

            let mut target_pos = head.start;
            for segment in self.segments.iter_mut().skip(1) {
                segment.pos.x += (target_pos.x - segment.start.x) as f32 * frame_velocity;
                segment.pos.y += (target_pos.y - segment.start.y) as f32 * frame_velocity;
                target_pos = segment.start;
            }
        }

        if movement_finished {
            self.head_target_pos = None;
            for segment in self.segments.iter_mut() {
                segment.pos.x = segment.pos.x.round();
                segment.pos.y = segment.pos.y.round();
            }
        }
    }

    fn bites_itself(&self, pos: GridPosition) -> bool {
        if self.segments.len() == 1 {
            return false;
        }
        self.segments[1..(self.segments.len() - 1).max(2)]
            .iter()
            .any(|segment| {
                segment.pos.x.round() as GridCoordinate == pos.x
                    && segment.pos.y.round() as GridCoordinate == pos.y
            })
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, grid_size: f32) {
        for (i, segment) in self.segments.iter().enumerate() {
            let color = if i == 0 { Color::RED } else { Color::ORANGE };
            d.draw_circle(
                ((segment.pos.x + 0.5) * grid_size) as i32,
                ((segment.pos.y + 0.5) * grid_size) as i32,
                grid_size / 2 as f32 - 2.0,
                color,
            );
        }
    }
}
