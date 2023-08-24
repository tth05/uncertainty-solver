use rand::distributions::{Distribution, Uniform};
use crate::verifier;

const ITERATIONS: usize = 100000000;

pub type IndexPermutation = (usize, usize);

pub fn solve(mut original_state: [usize; 16], mode: usize) -> Result<Vec<IndexPermutation>, String> {
    let uniform_range = Uniform::from(0usize..16);
    let mut rand = rand::thread_rng();

    let checks = verifier::get_balance_checks(mode);
    let mut state = original_state;

    let mut found = false;
    for i in 0..ITERATIONS {
        let index1 = uniform_range.sample(&mut rand);
        let index2 = uniform_range.sample(&mut rand);

        // Swap the value at the index with the random value
        state.swap(index1, index2);

        // Check if the state is valid
        if verifier::check_if_valid(&state, &checks) {
            println!("Found a valid state after {} iterations {:?}", i, state);
            found = true;
            break;
        }
    }

    if !found {
        return Err(format!("Failed to find a valid state after {:?} iterations", ITERATIONS));
    }

    // Compute permutations to get from original state to state
    let mut permutations = Vec::new();
    let mut i = 0;
    loop {
        if i >= original_state.len() {
            break;
        }

        let old_val = original_state[i];
        let new_index = state.iter().position(|&new_val| new_val == old_val).unwrap();
        if i != new_index {
            permutations.push((i, new_index));
            original_state.swap(i, new_index);
        } else {
            i += 1;
        }
    }

    Ok(permutations)
}
