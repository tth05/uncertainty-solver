use std::time::Duration;
use enigo::MouseControllable;
use screenshots::Screen;
use speedy::{Writable, Readable};
use crate::screen_reader::screen_scaled;
use crate::solver::IndexPermutation;

const DELAY: Duration = Duration::from_millis(100);

#[derive(Writable, Readable, Clone, Copy)]
pub struct MouseGridData {
    pub x_base: i32,
    pub y_base: i32,
    pub grid_offset: i32,
    pub grid_center_width: i32,
}

pub fn perform_permutations(screen: &Screen, grid_data: MouseGridData, permutations: Vec<IndexPermutation>) {
    let mut enigo = enigo::Enigo::new();
    let mut move_and_click = |x: i32, y: i32| {
        let additional_offset = screen_scaled(screen, if x > 1 { grid_data.grid_center_width - grid_data.grid_offset } else { 0 });
        enigo.mouse_move_to(
            screen_scaled(screen, grid_data.x_base) + screen_scaled(screen, x * grid_data.grid_offset) + additional_offset,
            screen_scaled(screen, grid_data.y_base) + screen_scaled(screen, y * grid_data.grid_offset),
        );
        std::thread::sleep(Duration::from_millis(80));
        enigo.mouse_click(enigo::MouseButton::Left);
    };

    for (a, b) in permutations {
        let (x1, y1) = convert_index_to_coords(a);
        let (x2, y2) = convert_index_to_coords(b);

        move_and_click(x1, y1);
        std::thread::sleep(DELAY);
        move_and_click(x2, y2);
        std::thread::sleep(DELAY);
    }
}

fn convert_index_to_coords(index: usize) -> (i32, i32) {
    ((index / 4) as i32, (index % 4) as i32)
}