use std::io::{Read, Write};
use crate::actor::MouseGridData;
use crate::screen_reader::GridData;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input};
use screenshots::Screen;
use speedy::{Readable, Writable};
use std::time::Instant;
use dialoguer::console::Term;

mod actor;
mod screen_reader;
mod solver;
mod verifier;

const X_BASE: i32 = 1700;
const Y_BASE: i32 = 760;
const GRID_OFFSET: i32 = 70;

const MOUSE_X_BASE: i32 = 1500;
const MOUSE_Y_BASE: i32 = 650;
const MOUSE_GRID_OFFSET: i32 = 110;
const MOUSE_GRID_CENTER_WIDTH: i32 = 650;

fn main() {
    let theme = ColorfulTheme::default();

    let use_resolver_x = Input::<bool>::with_theme(&theme)
        .with_prompt("Use Resolver X")
        .interact_text_on(&Term::stdout())
        .unwrap();

    let mode = Input::<usize>::with_theme(&theme)
        .with_prompt("Mode")
        .validate_with(|val: &usize| {
            if (1..=5).contains(val) {
                Ok(())
            } else {
                Err("Invalid mode")
            }
        })
        .interact_text()
        .unwrap();

    let screen = {
        let screens = Screen::all().unwrap();
        let screen_id = Input::<usize>::with_theme(&theme)
            .with_prompt("Screen ID")
            .validate_with(|val: &usize| {
                if (0..screens.len()).contains(val) {
                    Ok(())
                } else {
                    Err("Invalid screen ID")
                }
            })
            .interact_text()
            .unwrap();

        screens[screen_id]
    };

    let verify_width = |val: &u32| {
        if (0..(screen.display_info.width as f32 * screen.display_info.scale_factor) as u32)
            .contains(val)
        {
            Ok(())
        } else {
            Err("Not in range of screen width")
        }
    };
    let verify_height = |val: &u32| {
        if (0..(screen.display_info.height as f32 * screen.display_info.scale_factor) as u32)
            .contains(val)
        {
            Ok(())
        } else {
            Err("Not in range of screen height")
        }
    };

    let (screen_grid_data, mouse_grid_data) = read_input_state()
        .or_else(|| {
            let screen_grid_data = {
                let x_base = Input::<u32>::with_theme(&theme)
                    .with_prompt("Value Grid X Base")
                    .validate_with(verify_width)
                    .interact_text()
                    .unwrap() as i32;
                let y_base = Input::<u32>::with_theme(&theme)
                    .with_prompt("Value Grid Y Base")
                    .validate_with(verify_height)
                    .interact_text()
                    .unwrap() as i32;
                let grid_offset = Input::<u32>::with_theme(&theme)
                    .with_prompt("Value Grid Offset")
                    .validate_with(verify_width)
                    .interact_text()
                    .unwrap() as i32;

                screen_reader::GridData {
                    x_base,
                    y_base,
                    grid_offset,
                }
            };

            let mouse_grid_data = {
                let x_base = Input::<u32>::with_theme(&theme)
                    .with_prompt("Mouse Grid X Base")
                    .validate_with(verify_width)
                    .interact_text()
                    .unwrap() as i32;
                let y_base = Input::<u32>::with_theme(&theme)
                    .with_prompt("Mouse Grid Y Base")
                    .validate_with(verify_height)
                    .interact_text()
                    .unwrap() as i32;
                let grid_offset = Input::<u32>::with_theme(&theme)
                    .with_prompt("Mouse Grid Offset")
                    .validate_with(verify_width)
                    .interact_text()
                    .unwrap() as i32;
                let grid_center_width = Input::<u32>::with_theme(&theme)
                    .with_prompt("Mouse Grid Center Width")
                    .validate_with(verify_width)
                    .interact_text()
                    .unwrap() as i32;

                actor::MouseGridData {
                    x_base,
                    y_base,
                    grid_offset,
                    grid_center_width,
                }
            };

            Some((screen_grid_data, mouse_grid_data))
        })
        .unwrap();

    write_input_state(&screen_grid_data, &mouse_grid_data);

    println!("Use Ctrl+H to start solver");
    inputbot::KeybdKey::HKey.bind(move || {
        if !inputbot::KeybdKey::LControlKey.is_pressed() {
            return;
        }

        let t = Instant::now();
        let mut arr = if use_resolver_x {
            screen_reader::read_values_from_screen(&screen, screen_grid_data)
        } else {
            screen_reader::read_values_sampled_from_screen(&screen, screen_grid_data)
        };
        println!("Read values from screen: {:?}, took {:?}", arr, t.elapsed());

        ensure_no_duplicates(&mut arr);

        let t = Instant::now();
        let result = solver::solve(arr, mode);
        println!("Solver took {:?}", t.elapsed());
        match result {
            Ok(permutations) => actor::perform_permutations(&screen, mouse_grid_data, permutations),
            Err(e) => println!("{}", e),
        };
    });

    inputbot::handle_input_events();
}

fn write_input_state(screen_grid_data: &GridData, mouse_grid_data: &MouseGridData) {
    let mut buf = Vec::new();
    buf.extend(screen_grid_data.write_to_vec().unwrap());
    buf.extend(mouse_grid_data.write_to_vec().unwrap());
    std::fs::write(".uncertainty-solver-input", buf)
        .expect("Failed to write input state to .uncertainty-solver-input");
}

fn read_input_state() -> Option<(GridData, MouseGridData)> {
    let buf = match std::fs::read(".uncertainty-solver-input") {
        Ok(buf) => buf,
        Err(_) => return None,
    };

    let (screen_grid_data, mouse_grid_data) = match Readable::read_from_buffer(&buf) {
        Ok(b) => b,
        Err(_) => return None,
    };
    Some((screen_grid_data, mouse_grid_data))
}

fn ensure_no_duplicates(arr: &mut [usize; 16]) {
    for i in 0..16 {
        while arr.iter().filter(|&n| *n == arr[i]).count() > 1 {
            arr[i] += 1;
        }
    }
}
