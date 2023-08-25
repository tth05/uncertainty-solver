use crate::actor::MouseGridData;
use crate::screen_reader::GridData;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use screenshots::Screen;
use speedy::{Readable, Writable};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

mod actor;
mod screen_reader;
mod solver;
mod verifier;

fn main() {
    let input_state = read_input_state();
    let theme = ColorfulTheme::default();

    let use_resolver_x = Confirm::with_theme(&theme)
        .with_prompt("Use Resolver X")
        .show_default(input_state.is_some())
        .default(
            input_state
                .as_ref()
                .map(|s| s.0.use_resolver_x)
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
        .default(input_state.as_ref().map(|s| s.0.mode - 1).unwrap_or(0))
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
            .default(input_state.as_ref().map(|s| s.0.screen_id).unwrap_or(0))
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

    let (basic_data, screen_grid_data, mouse_grid_data) = input_state
        // Always use new input state for these values
        .map(|state| {
            (
                BasicData {
                    use_resolver_x,
                    mode,
                    screen_id,
                },
                state.1,
                state.2,
            )
        })
        .or_else(|| {
            let screen_grid_data = {
                GridData {
                    x_base: number_input("Value Grid X Base", &verify_width),
                    y_base: number_input("Value Grid Y Base", &verify_height),
                    grid_offset: number_input("Value Grid Offset", &verify_width),
                }
            };

            let mouse_grid_data = {
                MouseGridData {
                    x_base: number_input("Mouse Grid X Base", &verify_width),
                    y_base: number_input("Mouse Grid Y Base", &verify_height),
                    grid_offset: number_input("Mouse Grid Offset", &verify_width),
                    grid_center_width: number_input("Mouse Grid Center Width", &verify_width),
                }
            };

            Some((
                BasicData {
                    use_resolver_x,
                    mode,
                    screen_id,
                },
                screen_grid_data,
                mouse_grid_data,
            ))
        })
        .unwrap();

    write_input_state(&basic_data, &screen_grid_data, &mouse_grid_data);

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
                screen_reader::read_values_from_screen(&screen, screen_grid_data)
            } else {
                screen_reader::read_values_sampled_from_screen(&screen, screen_grid_data)
            };

            ensure_no_duplicates(&mut arr);
            println!("Read values from screen: {:?}, took {:?}", arr, t.elapsed());

            let t = Instant::now();
            let result = solver::solve(arr, mode, shift);
            println!("Solver took {:?}", t.elapsed());
            match result {
                Ok(permutations) => {
                    actor::perform_permutations(&screen, mouse_grid_data, permutations)
                }
                Err(e) => println!("{}", e),
            };

            is_running.store(false, Ordering::Relaxed);
        }
    });

    inputbot::handle_input_events();
}

#[derive(Writable, Readable)]
struct BasicData {
    use_resolver_x: bool,
    mode: usize,
    screen_id: usize,
}

fn write_input_state(
    basic_data: &BasicData,
    screen_grid_data: &GridData,
    mouse_grid_data: &MouseGridData,
) {
    let mut buf = Vec::new();
    buf.extend(basic_data.write_to_vec().unwrap());
    buf.extend(screen_grid_data.write_to_vec().unwrap());
    buf.extend(mouse_grid_data.write_to_vec().unwrap());
    std::fs::write(".uncertainty-solver-input", buf)
        .expect("Failed to write input state to .uncertainty-solver-input");
}

fn read_input_state() -> Option<(BasicData, GridData, MouseGridData)> {
    let buf = match std::fs::read(".uncertainty-solver-input") {
        Ok(buf) => buf,
        Err(_) => return None,
    };

    let (basic_data, screen_grid_data, mouse_grid_data) = match Readable::read_from_buffer(&buf) {
        Ok(b) => b,
        Err(_) => return None,
    };
    Some((basic_data, screen_grid_data, mouse_grid_data))
}

fn ensure_no_duplicates(arr: &mut [usize; 16]) {
    for i in 0..16 {
        while arr.iter().filter(|&n| *n == arr[i]).count() > 1 {
            arr[i] += 1;
        }
    }
}
