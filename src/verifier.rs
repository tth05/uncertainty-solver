pub fn get_balance_checks(mode: usize) -> Vec<Vec<(usize, usize)>> {
    if !(1usize..=5).contains(&mode) {
        panic!("Invalid mode")
    }

    match mode {
        //Case 1
        1 => vec![vec![(0, 15), (1, 14), (2, 13), (3, 12), (4, 11), (5, 10), (6, 9), (7, 8)]],
        // Case 2
        2 => vec![
            vec![(0, 7), (4, 3), (1, 6), (5, 2)],
            vec![(8, 15), (12, 11), (9, 14), (13, 10)],
        ],
        // Case 3
        3 => vec![
            vec![(0, 3), (4, 9), (8, 5), (12, 1)],
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
            vec![(0, 15), (1, 14), (2, 13), (3, 12), (4, 11), (5, 10), (6, 9), (7, 8)],
            vec![(2, 7), (6, 3)],
            vec![(10, 15), (14, 11)],
        ],
        _ => unreachable!()
    }
}

pub fn check_if_valid(state: &[usize; 16], checks: &[Vec<(usize, usize)>]) -> bool {
    checks.iter().all(|check| {
        let mut inequality = 0.0;
        for (first, last) in check.iter().cloned() {
            inequality += (state[first] as f32 - state[last] as f32).abs();
        }

        inequality < (check.len() << 7) as f32
    })
}
