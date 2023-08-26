use image::RgbaImage;
use crate::InputData;
use screenshots::{Screen};

const SAMPLE_COUNT: usize = 150;

const GRID_X_OFFSET_BASE_SCALE: i32 = 210;
const GRID_Y_OFFSET_BASE_SCALE: i32 = 115;
const GRID_OFFSET_BASE_SCALE: i32 = 75;

pub fn read_values_from_screen(screen: &Screen, input_data: &InputData) -> [usize; 16] {
    let mut arr = [0usize; 16];

    let mut i = 0;
    let image = capture_grid_image(screen, input_data);

    let image_size = image.width();
    let grid_offset = image_size / 3 - 1;
    for x in 0..4 {
        for y in 0..4 {
            let b = image.get_pixel(x * grid_offset, y * grid_offset).0[2];
            // let gray = g as usize + b as usize;
            arr[i] = ((b as f64 * 999f64) / 255f64) as usize;
            i += 1;
        }
    }

    arr
}

pub fn read_values_sampled_from_screen(screen: &Screen, input_data: &InputData) -> [usize; 16] {
    let mut arr = [0usize; 16];

    for _ in 0..SAMPLE_COUNT {
        let mut i = 0;
        let image = capture_grid_image(screen, input_data);

        let image_size = image.width();
        let grid_offset = image_size / 3 - 1;
        for x in 0..4 {
            for y in 0..4 {
                let [_, g, b, _] = image.get_pixel(x * grid_offset, y * grid_offset).0;
                let gray = (0.4 * g as f32 + 0.6 * b as f32) as u8;
                arr[i] += usize::from(gray >= 10);
                i += 1;
            }
        }
    }

    for i in arr.iter_mut() {
        *i = ((*i as f64 / SAMPLE_COUNT as f64) * 1000f64) as usize;
    }

    arr
}

/// NOTE: The returned image is scaled by the scale factor
fn capture_grid_image(screen: &Screen, input_data: &InputData) -> RgbaImage {
    screen
        .capture_area(
            screen_scaled(
                input_data,
                input_data.mouse_grid_x_base + ingame_scaled(input_data, GRID_X_OFFSET_BASE_SCALE),
            ),
            screen_scaled(
                input_data,
                input_data.mouse_grid_y_base + ingame_scaled(input_data, GRID_Y_OFFSET_BASE_SCALE),
            ),
            screen_ingame_scaled(input_data, 3 * GRID_OFFSET_BASE_SCALE) as u32,
            screen_ingame_scaled(input_data, 3 * GRID_OFFSET_BASE_SCALE) as u32,
        )
        .unwrap()
}

pub fn screen_scaled(input_data: &InputData, val: i32) -> i32 {
    (val as f64 * input_data.screen_scale) as i32
}

pub fn ingame_scaled(input_data: &InputData, val: i32) -> i32 {
    (val as f64 * input_data.ingame_scale) as i32
}

pub fn screen_ingame_scaled(input_data: &InputData, val: i32) -> i32 {
    screen_scaled(input_data, ingame_scaled(input_data, val))
}
