use screenshots::Screen;
use speedy::{Readable, Writable};

const SAMPLE_COUNT: usize = 100;

#[derive(Writable, Readable, Clone, Copy)]
pub struct GridData {
    pub x_base: i32,
    pub y_base: i32,
    pub grid_offset: i32,
}

pub fn read_values_from_screen(screen: &Screen, grid_data: GridData) -> [usize; 16] {
    let mut arr = [0usize; 16];

    let mut i = 0;
    // NOTE: The returned image is scaled by the scale factor
    let image = screen.capture_area(
        screen_scaled(screen, grid_data.x_base),
        screen_scaled(screen, grid_data.y_base),
        screen_scaled(screen, 3 * grid_data.grid_offset) as u32,
        screen_scaled(screen, 3 * grid_data.grid_offset) as u32,
    ).unwrap();

    let image_size = image.width() as usize;
    let grid_offset = image_size / 3 - 1;
    for x in 0..4 {
        for y in 0..4 {
            let pixel_offset = (y * grid_offset * image_size + x * grid_offset) * 4;
            let [_, g, b, _] = image.rgba()[pixel_offset..pixel_offset + 4] else { unreachable!() };
            // let gray = g as usize + b as usize;
            arr[i] = ((b as f64 * 999f64) / 255f64) as usize;
            i += 1;
        }
    }

    arr
}

pub fn read_values_sampled_from_screen(screen: &Screen, grid_data: GridData) -> [usize; 16] {
    let mut arr = [0usize; 16];

    for _ in 0..SAMPLE_COUNT {
        let mut i = 0;
        // NOTE: The returned image is scaled by the scale factor
        let image = screen.capture_area(
            screen_scaled(screen, grid_data.x_base),
            screen_scaled(screen, grid_data.y_base),
            screen_scaled(screen, 3 * grid_data.grid_offset) as u32,
            screen_scaled(screen, 3 * grid_data.grid_offset) as u32,
        ).unwrap();

        let image_size = image.width() as usize;
        let grid_offset = image_size / 3 - 1;
        for x in 0..4 {
            for y in 0..4 {
                let pixel_offset = (y * grid_offset * image_size + x * grid_offset) * 4;
                let [_, g, b, _] = image.rgba()[pixel_offset..pixel_offset + 4] else { unreachable!() };
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

pub fn screen_scaled(screen: &Screen, val: i32) -> i32 {
    (val as f32 * 1f32 / screen.display_info.scale_factor) as i32
}
