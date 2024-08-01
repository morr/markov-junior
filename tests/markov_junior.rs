#![feature(stmt_expr_attributes)]

use markov_junior::*;

fn set_pattern(mj: &mut MarkovJunior, line: &str) -> Pattern {
    let pattern = Pattern::new(line);
    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(pattern.clone(), pattern.clone())],
        kind: RuleKind::One,
        steps: None,
    });
    mj.precompute_canonical_forms(0);

    pattern
}

#[test]
fn test_pattern_fits_canonical() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);

    let pattern = set_pattern(&mut mj, "EF/HI");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), None);
    assert_eq!(mj.pattern_fits_canonical(1, 1, &pattern), Some(1));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern), None);

    let pattern = set_pattern(&mut mj, "AB/DE");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(1));

    let pattern_90 = set_pattern(&mut mj, "DA/EB");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_90), Some(4));

    let pattern_180 = set_pattern(&mut mj, "ED/BA");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_180), Some(3));

    let pattern_270 = set_pattern(&mut mj, "BE/AD");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_270), Some(2));

    let pattern_mirror = set_pattern(&mut mj, "BA/ED");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_mirror), Some(-1));

    let pattern_mirror_90 = set_pattern(&mut mj, "EB/DA");
    assert_eq!(
        mj.pattern_fits_canonical(0, 0, &pattern_mirror_90),
        Some(-2)
    );

    let pattern_mirror_180 = set_pattern(&mut mj, "DE/AB");
    assert_eq!(
        mj.pattern_fits_canonical(0, 0, &pattern_mirror_180),
        Some(-3)
    );

    let pattern_mirror_270 = set_pattern(&mut mj, "AD/BE");
    assert_eq!(
        mj.pattern_fits_canonical(0, 0, &pattern_mirror_270),
        Some(-4)
    );
}

#[test]
fn test_pattern_fits() {
    let mut mj = MarkovJunior::new_grid("ABCDEFGHI", 3, 3, None);

    let pattern = set_pattern(&mut mj, "AB");
    assert_eq!(mj.pattern_fits(0, 0, &pattern), Some(1));

    let pattern = set_pattern(&mut mj, "BC");
    assert_eq!(mj.pattern_fits(1, 0, &pattern), Some(1));

    let pattern = set_pattern(&mut mj, "EF");
    assert_eq!(mj.pattern_fits(1, 1, &pattern), Some(1));

    let pattern = set_pattern(&mut mj, "HI");
    assert_eq!(mj.pattern_fits(1, 2, &pattern), Some(1));

    let pattern = set_pattern(&mut mj, "FI");
    assert_eq!(mj.pattern_fits(2, 1, &pattern), Some(2));

    let pattern = set_pattern(&mut mj, "IF");
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

#[test]
fn test_generate() {
    let mut mj = MarkovJunior::new('.', 3, 3, None);
    mj.grid = #[rustfmt::skip] vec![
        b'B', b'W', b'G',
        b'B', b'W', b'G',
        b'B', b'W', b'G'
    ];
    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(Pattern::new("BW"), Pattern::new("WW"))],
        kind: RuleKind::One,
        steps: None,
    });
    // mj.print_grid();
    mj.generate();
    mj.print_grid();

    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'W', b'W', b'G',
            b'W', b'W', b'G',
            b'W', b'W', b'G'
        ]
    );
}

#[test]
fn test_generate_2() {
    let mut mj = MarkovJunior::new('.', 3, 3, None);
    mj.grid = #[rustfmt::skip] vec![
        b'B', b'W', b'G',
        b'B', b'W', b'G',
        b'B', b'W', b'G'
    ];

    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(Pattern::new("WG"), Pattern::new("WR"))],
        kind: RuleKind::One,
        steps: None,
    });
    mj.print_grid();
    mj.generate();
    mj.print_grid();

    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'B', b'W', b'R',
            b'B', b'W', b'R',
            b'B', b'W', b'R'
        ]
    );
}

#[test]
fn test_generate_3() {
    let mut mj = MarkovJunior::new('.', 3, 3, None);
    mj.grid = #[rustfmt::skip] vec![
        b'B', b'W', b'G',
        b'B', b'W', b'G',
        b'B', b'W', b'G'
    ];
    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(Pattern::new("BW"), Pattern::new("WW"))],
        kind: RuleKind::One,
        steps: None,
    });
    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(Pattern::new("WG"), Pattern::new("WR"))],
        kind: RuleKind::One,
        steps: None,
    });
    mj.generate();
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'W', b'W', b'R',
            b'W', b'W', b'R',
            b'W', b'W', b'R'
        ]
    );
}

#[test]
fn test_generate_4() {
    let mut mj = MarkovJunior::new('.', 3, 3, None);
    mj.grid = #[rustfmt::skip] vec![
        b'B', b'B', b'B',
        b'W', b'W', b'W',
        b'G', b'G', b'G'
    ];
    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(Pattern::new("BW"), Pattern::new("WW"))],
        kind: RuleKind::One,
        steps: None,
    });
    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(Pattern::new("WG"), Pattern::new("WR"))],
        kind: RuleKind::One,
        steps: None,
    });
    mj.generate();
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'W', b'W', b'W',
            b'W', b'W', b'W',
            b'R', b'R', b'R'
        ]
    );
}

#[test]
fn test_generate_5() {
    let mut mj = MarkovJunior::new('.', 3, 3, None);
    mj.grid = #[rustfmt::skip] vec![
        b'B', b'B', b'B',
        b'W', b'W', b'W',
        b'G', b'G', b'G'
    ];

    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(Pattern::new("BW"), Pattern::new("WW"))],
        kind: RuleKind::One,
        steps: None,
    });
    mj.generate();

    // Check the result
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'W', b'W', b'W',
            b'W', b'W', b'W',
            b'G', b'G', b'G',
        ]
    );
}

#[test]
fn test_generate_6() {
    let mut mj = MarkovJunior::new('.', 3, 3, None);
    mj.grid = #[rustfmt::skip] vec![
        b'B', b'B', b'B',
        b'B', b'B', b'B',
        b'B', b'B', b'W',
    ];

    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(Pattern::new("WB"), Pattern::new("WW"))],
        kind: RuleKind::One,
        steps: None,
    });
    mj.generate();

    mj.print_grid();
    // Check the result
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'W', b'W', b'W',
            b'W', b'W', b'W',
            b'W', b'W', b'W',
        ]
    );
}

#[test]
fn test_generate_7() {
    let mut mj = MarkovJunior::new('.', 4, 4, None);
    mj.grid = #[rustfmt::skip] vec![
        b'B', b'B', b'B', b'B',
        b'B', b'U', b'B', b'B',
        b'B', b'B', b'U', b'B',
        b'B', b'B', b'B', b'B'
    ];

    mj.add_rule(Rule {
        patterns: vec![PatternRule::new(Pattern::new("BU/UB"), Pattern::new("U*/**"))],
        kind: RuleKind::All,
        steps: None,
    });
    mj.generate();
    // mj.print_grid();

    // Check the result
    assert_eq!(mj.patterns_applied_counter, 1);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'B', b'B', b'B', b'B',
            b'B', b'U', b'U', b'B',
            b'B', b'B', b'U', b'B',
            b'B', b'B', b'B', b'B'
        ]
    );
}
