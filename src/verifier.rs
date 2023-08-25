pub fn get_balance_checks(mode: usize) -> Vec<Vec<(usize, usize)>> {
    if !(1usize..=5).contains(&mode) {
        panic!("Invalid mode")
    }

    match mode {
        //Case 1
        1 => vec![vec![
            (0, 15),
            (1, 14),
            (2, 13),
            (3, 12),
            (4, 11),
            (5, 10),
            (6, 9),
            (7, 8),
        ]],
        // Case 2
        2 => vec![
            vec![(0, 7), (4, 3), (1, 6), (5, 2)],
            vec![(8, 15), (12, 11), (9, 14), (13, 10)],
        ],
        // Case 3
        3 => vec![
            vec![(0, 13), (4, 9), (8, 5), (12, 1)],
            vec![(0, 7), (4, 3), (1, 6), (5, 2)],
            vec![(8, 15), (12, 11), (9, 14), (13, 10)],
            vec![(2, 15), (6, 11), (10, 7), (14, 3)],
        ],
        // Case 4
        4 => vec![
            vec![(0, 5), (4, 1)],
            vec![(8, 13), (12, 9)],
            vec![(2, 7), (6, 3)],
            vec![(10, 15), (14, 11)],
        ],
        // Case 5
        5 => vec![
            vec![(0, 5), (4, 1)],
            vec![(8, 13), (12, 9)],
            vec![
                (0, 15),
                (1, 14),
                (2, 13),
                (3, 12),
                (4, 11),
                (5, 10),
                (6, 9),
                (7, 8),
            ],
            vec![(2, 7), (6, 3)],
            vec![(10, 15), (14, 11)],
        ],
        _ => unreachable!(),
    }
}

pub fn check_if_solved(state: &[usize; 16], checks: &[Vec<(usize, usize)>]) -> (bool, usize) {
    checks
        .iter()
        .map(|check| check_inequality(state, check, false))
        .fold((true, 0), |(a_solved, a_ineq), (b_solved, b_ineq)| {
            (a_solved && b_solved, a_ineq + b_ineq)
        })
}

pub fn check_if_unsolved(state: &[usize; 16], checks: &[Vec<(usize, usize)>]) -> (bool, usize) {
    checks
        .iter()
        .map(|check| check_inequality(state, check, true))
        .fold((false, 0), |(a_solved, a_ineq), (b_solved, b_ineq)| {
            (!a_solved && !b_solved, a_ineq + b_ineq)
        })
}

fn check_inequality(
    state: &[usize; 16],
    check: &[(usize, usize)],
    threshold_dir: bool,
) -> (bool, usize) {
    let mut inequality = 0;
    for (first, last) in check.iter().cloned() {
        inequality += (state[first] as i64 - state[last] as i64).abs();
    }

    // NOTE: We shift by 7 (original code), then shift by 1 because the `check` slice contains
    // pairs and not single values. Then some smaller factor is subtracted to account for the
    // inaccuracy of the screen reader (Tighter threshold prevents finding solutions which are too
    // close to an unsolved state)
    let threshold = (check.len() << 8) as i64
        + (check.len() << 4) as i64 * (if threshold_dir { 1 } else { -1 });
    (inequality < threshold, inequality as usize)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_inequality() {
        let state = [
            66, 946, 646, 126, 146, 753, 760, 506, 793, 80, 706, 600, 400, 893, 393, 813,
        ];
        let checks = get_balance_checks(3);
        assert!(!check_if_solved(&state, &checks).0);

        let state = [
            910, 860, 100, 740, 780, 90, 400, 250, 830, 760, 890, 130, 620, 870, 990, 530,
        ];
        let checks = get_balance_checks(3);
        assert!(!check_if_solved(&state, &checks).0);

        let state = [
            571, 460, 821, 570, 200, 590, 251, 250, 820, 10, 610, 280, 880, 730, 300, 320,
        ];
        let checks = get_balance_checks(3);
        assert!(!check_if_unsolved(&state, &checks).0);
    }
}
