use crate::verifier;
use rand::distributions::{Distribution, Uniform};

const ITERATIONS: usize = 100000000;

pub type IndexPermutation = (usize, usize);

pub fn solve(
    screen_state: [usize; 16],
    mode: usize,
    unsolve: bool,
) -> Result<Vec<IndexPermutation>, String> {
    println!("Solving for mode {}", unsolve);
    let mut permutations = Vec::with_capacity(15);
    let mut best_permutations = vec![(0, 0); 16];

    let mut total_iterations = 0;
    for _ in 0..5000 {
        let uniform_range = Uniform::from(0usize..16);
        let mut rand = rand::thread_rng();

        let checks = verifier::get_balance_checks(mode);
        let mut solved_state = screen_state;

        let mut found = false;
        for _ in 0..ITERATIONS {
            total_iterations += 1;

            let index1 = uniform_range.sample(&mut rand);
            let index2 = uniform_range.sample(&mut rand);

            // Swap the value at the index with the random value
            solved_state.swap(index1, index2);

            // Check if the state is valid
            let done = if !unsolve {
                verifier::check_if_solved(&solved_state, &checks)
            } else {
                verifier::check_if_unsolved(&solved_state, &checks)
            };

            if done {
                found = true;
                break;
            }
        }

        if !found {
            return Err(format!(
                "Failed to find a valid state after {:?} iterations",
                ITERATIONS
            ));
        }

        // Compute permutations to get from original state to state
        permutations.clear();
        let mut screen_state_copy = screen_state;
        let mut i = 0;
        while i < screen_state.len() {
            let old_val = screen_state_copy[i];
            let new_index = solved_state
                .iter()
                .position(|&new_val| new_val == old_val)
                .unwrap();
            if i != new_index {
                permutations.push((i, new_index));
                screen_state_copy.swap(i, new_index);
            } else {
                i += 1;
            }
        }

        if permutations.len() < best_permutations.len() {
            std::mem::swap(&mut permutations, &mut best_permutations);
        }

        // Good enough, although 1 is the minimum
        if best_permutations.len() <= 2 {
            break;
        }
    }

    println!(
        "Found a close and valid state after {} iterations",
        total_iterations
    );
    println!(
        "Requires {} permutations: {:?}",
        best_permutations.len(),
        best_permutations
    );

    Ok(best_permutations)
}
