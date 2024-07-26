#![feature(stmt_expr_attributes)]
use std::collections::HashMap;

use markov_junior::*;

fn create_test_grid(data: &str) -> Vec<u8> {
    data.chars().map(|c| c as u8).collect()
}

#[test]
fn test_pattern_fits_canonical() {
    let mj = MarkovJunior {
        grid: create_test_grid("ABCDEFGHI"),
        width: 3,
        height: 3,
        rules: vec![],
        canonical_forms: HashMap::new(),
    };

    let pattern = Pattern::new("AB/DE");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(1));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern), None);

    let pattern_90 = Pattern::new("DA/EB");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_90), Some(2));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_90), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_90), None);

    let pattern_180 = Pattern::new("ED/BA");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_180), Some(3));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_180), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_180), None);

    let pattern_270 = Pattern::new("BE/AD");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_270), Some(4));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_270), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_270), None);

    let pattern_mirror = Pattern::new("BA/ED");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_mirror), Some(-1));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_mirror), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_mirror), None);

    let pattern_mirror_90 = Pattern::new("EB/DA");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_mirror_90), Some(-2));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_mirror_90), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_mirror_90), None);

    let pattern_mirror_180 = Pattern::new("DE/AB");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_mirror_180), Some(-3));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_mirror_180), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_mirror_180), None);

    let pattern_mirror_270 = Pattern::new("AD/BE");
    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_mirror_270), Some(-4));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_mirror_270), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_mirror_270), None);
}

#[test]
fn test_apply_pattern() {
    let mut mj = MarkovJunior::new('.', 5, 5);
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
fn test_apply_pattern_with_anything_symbol() {
    let mut mj = MarkovJunior::new('.', 5, 5);
    let pattern = Pattern::new("A*/C*");

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
}

#[test]
fn test_apply_pattern_at_edge() {
    let mut mj = MarkovJunior::new('.', 5, 5);
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

// #[test]
// fn test_apply_pattern() {
//     let mut mj = MarkovJunior::new('.', 3, 3);
//     mj.grid = #[rustfmt::skip] vec![
//         b'B', b'W', b'G',
//         b'B', b'W', b'G',
//         b'B', b'W', b'G'
//     ];
//
//     let pattern_rule = PatternRule::new(Pattern::new("BW"), Pattern::new("WW"));
//
//     let rule = Rule::new(RuleKind::All, vec![pattern_rule], None);
//     mj.add_rule(rule);
//
//     // Apply the rule
//     mj.generate();
//
//     // Check the result
//     assert_eq!(
//         mj.grid,
//         #[rustfmt::skip] vec![
//             b'W', b'W', b'G',
//             b'W', b'W', b'G',
//             b'W', b'W', b'G'
//         ]
//     );
// }
//
// #[test]
// fn test_apply_pattern_rotated() {
//     let mut mj = MarkovJunior::new('.', 3, 3);
//     mj.grid = #[rustfmt::skip] vec![
//         b'B', b'B', b'B',
//         b'W', b'W', b'W',
//         b'G', b'G', b'G'
//     ];
//
//     let pattern_rule = PatternRule::new(Pattern::new("BW"), Pattern::new("WW"));
//
//     let rule = Rule::new(RuleKind::All, vec![pattern_rule], None);
//     mj.add_rule(rule);
//
//     // Apply the rule
//     mj.generate();
//
//     // Check the result
//     assert_eq!(
//         mj.grid,
//         #[rustfmt::skip] vec![
//             b'W', b'W', b'W',
//             b'W', b'W', b'W',
//             b'G', b'G', b'G',
//         ]
//     );
// }
