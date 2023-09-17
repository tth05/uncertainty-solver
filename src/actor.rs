use std::time::Duration;
use enigo::MouseControllable;
use crate::InputData;
use crate::screen_reader::{ingame_scaled, screen_scaled};
use crate::solver::IndexPermutation;

const DELAY: Duration = Duration::from_millis(100);

const MOUSE_GRID_CENTER_WIDTH_BASE_SCALE: i32 = 650;

pub fn perform_permutations(screen: &Screen, input_data: &InputData, permutations: Vec<IndexPermutation>) {
    let mut enigo = enigo::Enigo::new();
    let mut move_and_click = |x: i32, y: i32| {
        let additional_offset = if x > 1 { ingame_scaled(input_data, MOUSE_GRID_CENTER_WIDTH_BASE_SCALE) - input_data.mouse_grid_offset } else { 0 };
        enigo.mouse_move_to(
            screen.display_info.x + screen_scaled(input_data, input_data.mouse_grid_x_base + x * input_data.mouse_grid_offset + additional_offset),
            screen.display_info.y + screen_scaled(input_data, input_data.mouse_grid_y_base + y * input_data.mouse_grid_offset),
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