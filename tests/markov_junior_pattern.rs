#![feature(stmt_expr_attributes)]
use markov_junior::*;

fn precompute_pattern(mj: &mut MarkovJunior, line: &str) -> Pattern {
    let pattern = Pattern::new(line);
    let rule = Rule {
        patterns: vec![PatternRule::new(pattern.clone(), pattern.clone(), None)],
        kind: RuleKind::One,
        steps: None,
    };
    mj.precompute_canonical_forms(&rule);

    pattern
}

#[test]
fn test_pattern_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);
    let pattern = precompute_pattern(&mut mj, "EF/HI");

    assert_eq!(mj.pattern_fits(0, 0, &pattern), None);
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), None);
    assert_eq!(mj.pattern_fits(1, 1, &pattern), Some(1));
    assert_eq!(mj.pattern_fits_canonical(1, 1, &pattern), Some(1));
    assert_eq!(mj.pattern_fits(1, 0, &pattern), None);
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern), None);
    assert_eq!(mj.pattern_fits(0, 1, &pattern), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern), None);
}

#[test]
fn test_pattern_0_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);
    let pattern = precompute_pattern(&mut mj, "AB/DE");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(1));
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(1));
}

#[test]
fn test_pattern_90_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);
    let pattern = precompute_pattern(&mut mj, "DA/EB");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(4));
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(4));
}

#[test]
fn test_pattern_180_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);
    let pattern = precompute_pattern(&mut mj, "ED/BA");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(3));
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(3));
}

#[test]
fn test_pattern_270_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);
    let pattern = precompute_pattern(&mut mj, "BE/AD");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(2));
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(2));
}

#[test]
fn test_pattern_mirror_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);
    let pattern = precompute_pattern(&mut mj, "BA/ED");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(-1));
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(-1));
}

#[test]
fn test_pattern_mirror_90_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);
    let pattern = precompute_pattern(&mut mj, "EB/DA");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(-2));
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(-2));
}

#[test]
fn test_pattern_mirror_180_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);
    let pattern = precompute_pattern(&mut mj, "DE/AB");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(-3));
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(-3));
}

#[test]
fn test_pattern_mirror_270_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);
    let pattern = precompute_pattern(&mut mj, "AD/BE");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(-4));
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(-4));
}

#[test]
fn test_pattern_fits_canonical_2() {
    let line = "AB/CD";
    let original_pattern = Pattern::new(line);

    for RotatedSeq { data, .. } in original_pattern.rotations.iter() {
        let pattern_line: String = vec![data[0], data[1], '/', data[2], data[3]]
            .into_iter()
            .collect();

        let mut mj = MarkovJunior::new_grid("ABCD", 2, 2, None);
        let pattern = precompute_pattern(&mut mj, &pattern_line);

        assert_ne!(mj.pattern_fits(0, 0, &pattern), None);
        assert_eq!(
            mj.pattern_fits(0, 0, &pattern),
            mj.pattern_fits_canonical(0, 0, &pattern)
        );

        let grid_data: String = data.iter().collect();

        let mut mj = MarkovJunior::new_grid(&grid_data, 2, 2, None);
        let pattern = precompute_pattern(&mut mj, line);

        assert_ne!(mj.pattern_fits(0, 0, &pattern), None);
        assert_eq!(
            mj.pattern_fits(0, 0, &pattern),
            mj.pattern_fits_canonical(0, 0, &pattern)
        );

        for RotatedSeq { data: data2, .. } in original_pattern.rotations.iter() {
            let grid_data2: String = data2.iter().collect();

            let mut mj = MarkovJunior::new_grid(&grid_data2, 2, 2, None);
            let pattern = precompute_pattern(&mut mj, &pattern_line);

            assert_ne!(mj.pattern_fits(0, 0, &pattern), None);
            assert_eq!(
                mj.pattern_fits(0, 0, &pattern),
                mj.pattern_fits_canonical(0, 0, &pattern)
            );
        }
    }
}

// #[test]
// fn test_pattern_fits_canonical_4() {
//     let mut mj = MarkovJunior::new_grid("UBBU", 2, 2, None);
//     let pattern = set_pattern(&mut mj, "BU/UB");
//     assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(2));
//     assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(2));
// }

#[test]
fn test_pattern_fits() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);

    let pattern = precompute_pattern(&mut mj, "AB");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(1));

    let pattern = precompute_pattern(&mut mj, "BC");
    assert_eq!(mj.pattern_fits(1, 0, &pattern), Some(1));

    let pattern = precompute_pattern(&mut mj, "EF");
    assert_eq!(mj.pattern_fits(1, 1, &pattern), Some(1));

    let pattern = precompute_pattern(&mut mj, "HI");
    assert_eq!(mj.pattern_fits(1, 2, &pattern), Some(1));

    let pattern = precompute_pattern(&mut mj, "FI");
    assert_eq!(mj.pattern_fits(2, 1, &pattern), Some(2));

    let pattern = precompute_pattern(&mut mj, "IF");
    assert_eq!(mj.pattern_fits(2, 1, &pattern), Some(4));
}

#[test]
fn test_apply_canonical_pattern() {
    let mut mj = MarkovJunior::new('.', 5, 5, None);
    let pattern = Pattern::new("AB/CD");

    mj.apply_pattern(1, 1, &pattern, 1);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'A', b'B', b'.', b'.',
            b'.', b'C', b'D', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    mj.apply_pattern(1, 1, &pattern, 2);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'C', b'A', b'.', b'.',
            b'.', b'D', b'B', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    mj.apply_pattern(1, 1, &pattern, 3);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'D', b'C', b'.', b'.',
            b'.', b'B', b'A', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    mj.apply_pattern(1, 1, &pattern, 4);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'B', b'D', b'.', b'.',
            b'.', b'A', b'C', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    mj.apply_pattern(1, 1, &pattern, -1);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'B', b'A', b'.', b'.',
            b'.', b'D', b'C', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    mj.apply_pattern(1, 1, &pattern, -2);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'D', b'B', b'.', b'.',
            b'.', b'C', b'A', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    mj.apply_pattern(1, 1, &pattern, -3);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'C', b'D', b'.', b'.',
            b'.', b'A', b'B', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    mj.apply_pattern(1, 1, &pattern, -4);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'A', b'C', b'.', b'.',
            b'.', b'B', b'D', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );
}

#[test]
fn test_apply_wildcard_square_pattern() {
    let pattern = Pattern::new("A*/C*");

    let mut mj = MarkovJunior::new('.', 5, 5, None);
    mj.apply_pattern(1, 1, &pattern, 1);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'A', b'.', b'.', b'.',
            b'.', b'C', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    let mut mj = MarkovJunior::new('.', 5, 5, None);
    mj.apply_pattern(1, 1, &pattern, 2);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'C', b'A', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    let mut mj = MarkovJunior::new('.', 5, 5, None);
    mj.apply_pattern(1, 1, &pattern, 3);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'C', b'.', b'.',
            b'.', b'.', b'A', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );

    let mut mj = MarkovJunior::new('.', 5, 5, None);
    mj.apply_pattern(1, 1, &pattern, 4);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'A', b'C', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
        ]
    );
}

#[test]
fn test_apply_horizontal_pattern() {
    let pattern = Pattern::new("AB");

    let mut mj = MarkovJunior::new('.', 4, 4, None);
    mj.apply_pattern(1, 1, &pattern, 1);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.',
            b'.', b'A', b'B', b'.',
            b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.',
        ]
    );

    let mut mj = MarkovJunior::new('.', 4, 4, None);
    mj.apply_pattern(1, 1, &pattern, 2);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.',
            b'.', b'A', b'.', b'.',
            b'.', b'B', b'.', b'.',
            b'.', b'.', b'.', b'.',
        ]
    );

    let mut mj = MarkovJunior::new('.', 4, 4, None);
    mj.apply_pattern(1, 1, &pattern, 3);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.',
            b'.', b'B', b'A', b'.',
            b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.',
        ]
    );

    let mut mj = MarkovJunior::new('.', 4, 4, None);
    mj.apply_pattern(1, 1, &pattern, 4);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.',
            b'.', b'B', b'.', b'.',
            b'.', b'A', b'.', b'.',
            b'.', b'.', b'.', b'.',
        ]
    );
}

#[test]
fn test_apply_vertical_pattern() {
    let pattern = Pattern::new("A/B");

    let mut mj = MarkovJunior::new('.', 4, 4, None);
    mj.apply_pattern(1, 1, &pattern, 1);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.',
            b'.', b'A', b'.', b'.',
            b'.', b'B', b'.', b'.',
            b'.', b'.', b'.', b'.',
        ]
    );

    let mut mj = MarkovJunior::new('.', 4, 4, None);
    mj.apply_pattern(1, 1, &pattern, 2);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.',
            b'.', b'B', b'A', b'.',
            b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.',
        ]
    );

    let mut mj = MarkovJunior::new('.', 4, 4, None);
    mj.apply_pattern(1, 1, &pattern, 3);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.',
            b'.', b'B', b'.', b'.',
            b'.', b'A', b'.', b'.',
            b'.', b'.', b'.', b'.',
        ]
    );

    let mut mj = MarkovJunior::new('.', 4, 4, None);
    mj.apply_pattern(1, 1, &pattern, 4);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.',
            b'.', b'A', b'B', b'.',
            b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.',
        ]
    );
}

#[test]
fn test_apply_pattern_at_edge() {
    let mut mj = MarkovJunior::new('.', 5, 5, None);
    let pattern = Pattern::new("AB/CD");

    mj.apply_pattern(3, 3, &pattern, 1);

    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.',
            b'.', b'.', b'.', b'A', b'B',
            b'.', b'.', b'.', b'C', b'D',
        ]
    );
}
