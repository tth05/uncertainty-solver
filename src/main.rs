#![feature(let_chains)]

use crate::actor::convert_index_to_coords;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use screenshots::Screen;
use speedy::{Readable, Writable};
use std::ops::Add;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

mod actor;
mod screen_reader;
mod solver;
mod verifier;

const MOUSE_GRID_OFFSET_BASE_SCALE: i32 = 110;

fn main() {
    if let Some(arg) = std::env::args().nth(1) && arg == "-r" {
        std::fs::remove_file(".uncertainty-solver-input").unwrap_or(());
    }

    let input_state = read_input_state();
    let theme = ColorfulTheme::default();

    let use_resolver_x = Confirm::with_theme(&theme)
        .with_prompt("Use Resolver X")
        .show_default(input_state.is_some())
        .default(
            input_state
                .as_ref()
                .map(|s| s.use_resolver_x)
                .unwrap_or(false),
        )
        .interact()
        .unwrap();

    let mode = Select::with_theme(&theme)
        .with_prompt("Mode")
        .items(&[
            "1: One lamp",
            "2: Two lamps",
            "3: Four lamps in centers",
            "4: Four lamps in corners",
            "5: Five lamps",
        ])
        .default(input_state.as_ref().map(|s| s.mode - 1).unwrap_or(0))
        .interact()
        .unwrap()
        + 1;

    let (screen_id, screen) = {
        let screens = Screen::all().unwrap();
        let screen_id = Select::with_theme(&theme)
            .with_prompt("Screen")
            .items(
                &screens
                    .iter()
                    .enumerate()
                    .map(|(i, screen)| {
                        format!(
                            "{}: {}x{} ({}x scale){}",
                            i,
                            screen.display_info.width,
                            screen.display_info.height,
                            screen.display_info.scale_factor,
                            if screen.display_info.is_primary {
                                " (primary)"
                            } else {
                                ""
                            }
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .default(input_state.as_ref().map(|s| s.screen_id).unwrap_or(0))
            .interact()
            .unwrap();

        (screen_id, screens[screen_id])
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

    let number_input =
        |prompt: &str, validator: &dyn Fn(&u32) -> Result<(), &'static str>| -> i32 {
            Input::<u32>::with_theme(&theme)
                .with_prompt(prompt)
                .validate_with(validator)
                .interact_text()
                .unwrap() as i32
        };

    let input_data = input_state
        // Always use new input state for these values
        .map(|state| {
                InputData {
                    use_resolver_x,
                    mode,
                    screen_id,
                    ..state
                }
        })
        .or_else(|| {
            let mouse_grid_x_base = number_input("Mouse Grid X Base", &verify_width);
            let mouse_grid_y_base = number_input("Mouse Grid Y Base", &verify_height);
            let mouse_grid_offset = number_input("Mouse Grid Offset", &verify_width);
            Some(
                InputData {
                    use_resolver_x,
                    mode,
                    screen_id,
                    mouse_grid_x_base,
                    mouse_grid_y_base,
                    mouse_grid_offset,
                    screen_scale: 1f64 / screen.display_info.scale_factor as f64,
                    ingame_scale: mouse_grid_offset as f64 / MOUSE_GRID_OFFSET_BASE_SCALE as f64
                },
            )
        })
        .unwrap();

    write_input_state(&input_data);

    println!("Use Ctrl+H to start solver");
    inputbot::KeybdKey::HKey.bind({
        let is_running = AtomicBool::new(false);

        move || {
            let shift = inputbot::KeybdKey::LShiftKey.is_pressed();
            if !inputbot::KeybdKey::LControlKey.is_pressed() || is_running.load(Ordering::Relaxed) {
                return;
            }

            is_running.store(true, Ordering::Relaxed);

            let t = Instant::now();
            let mut arr = if use_resolver_x {
                screen_reader::read_values_from_screen(&screen, &input_data)
            } else {
                screen_reader::read_values_sampled_from_screen(&screen, &input_data)
            };

            ensure_no_duplicates(&mut arr);
            println!("Read values from screen: {:?}, took {:?}", arr, t.elapsed());

            let t = Instant::now();
            let result = solver::solve(arr, mode, shift);
            println!("Solver took {:?}", t.elapsed());
            match result {
                Ok(permutations) => {
                    println!("Found a solution with {} permutations", permutations.len());
                    pretty_print_permutations(&permutations, 6);

                        actor::perform_permutations(&screen, &input_data, permutations);
                }
                Err(e) => println!("{}", e),
            };

            is_running.store(false, Ordering::Relaxed);
        }
    });

    inputbot::handle_input_events();
}

fn pretty_print_permutations(permutations: &[solver::IndexPermutation], grids_per_line: usize) {
    let mut lines = vec![String::new(); 4];
    permutations
        .chunks(grids_per_line)
        .enumerate()
        .for_each(|(chunk_index, chunk)| {
            for (index, perm) in chunk.iter().enumerate() {
                let (x1, y1) = convert_index_to_coords(perm.0);
                let (x2, y2) = convert_index_to_coords(perm.1);

                for y in 0..4 {
                    for x in 0..4 {
                        lines[y as usize].push(if x == x1 && y == y1 {
                            'A'
                        } else if x == x2 && y == y2 {
                            'B'
                        } else {
                            '~'
                        })
                    }

                    let print_arrow = y == 1
                        && (index != chunk.len() - 1
                            || chunk_index != ((permutations.len() - 1) / grids_per_line));
                    lines[y as usize].push_str(if print_arrow { " \u{2192} " } else { "   " });
                }
            }

            lines.iter_mut().for_each(|line| {
                println!("{}", line);
                line.clear();
            });

            println!();
        })
}

#[derive(Writable, Readable, Default)]
pub struct InputData {
    use_resolver_x: bool,
    mode: usize,
    screen_id: usize,
    mouse_grid_x_base: i32,
    mouse_grid_y_base: i32,
    mouse_grid_offset: i32,
    screen_scale: f64,
    ingame_scale: f64
}

fn write_input_state(
    input_data: &InputData,
) {
    let mut buf = Vec::new();
    buf.extend(input_data.write_to_vec().unwrap());
    std::fs::write(".uncertainty-solver-input", buf)
        .expect("Failed to write input state to .uncertainty-solver-input");
}

fn read_input_state() -> Option<InputData> {
    let buf = match std::fs::read(".uncertainty-solver-input") {
        Ok(buf) => buf,
        Err(_) => return None,
    };

    Readable::read_from_buffer(&buf).ok()
}

fn ensure_no_duplicates(arr: &mut [usize; 16]) {
    for i in 0..16 {
        while arr.iter().filter(|&n| *n == arr[i]).count() > 1 {
            arr[i] += 1;
        }
    }
}
