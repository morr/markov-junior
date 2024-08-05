#![feature(stmt_expr_attributes)]
use markov_junior::*;

fn rule_to_sequence<I>(rule_or_rules: I) -> Sequence
where
    I: IntoIterator<Item = Rule>,
{
    let vec = rule_or_rules
        .into_iter()
        .map(RuleOrSequence::Rule)
        .collect::<Vec<_>>();

    Sequence { vec, steps: None }
}

#[test]
fn test_generate() {
    let mut mj = MarkovJunior::new('.', 3, 3, None);
    mj.grid = #[rustfmt::skip] vec![
        b'B', b'W', b'G',
        b'B', b'W', b'G',
        b'B', b'W', b'G'
    ];
    let sequence = rule_to_sequence(Rule {
        patterns: vec![PatternRule::new(
            Pattern::new("BW"),
            Pattern::new("WW"),
            None,
        )],
        kind: RuleKind::One,
        steps: None,
    });

    mj.apply_sequence(&sequence);

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

    let sequence = rule_to_sequence(Rule {
        patterns: vec![PatternRule::new(
            Pattern::new("WG"),
            Pattern::new("WR"),
            None,
        )],
        kind: RuleKind::One,
        steps: None,
    });
    mj.apply_sequence(&sequence);

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
    let sequence = rule_to_sequence(vec![
        Rule {
            patterns: vec![PatternRule::new(
                Pattern::new("BW"),
                Pattern::new("WW"),
                None,
            )],
            kind: RuleKind::One,
            steps: None,
        },
        Rule {
            patterns: vec![PatternRule::new(
                Pattern::new("WG"),
                Pattern::new("WR"),
                None,
            )],
            kind: RuleKind::One,
            steps: None,
        },
    ]);
    mj.apply_sequence(&sequence);

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
    let sequence = rule_to_sequence(vec![
        Rule {
            patterns: vec![PatternRule::new(
                Pattern::new("BW"),
                Pattern::new("WW"),
                None,
            )],
            kind: RuleKind::One,
            steps: None,
        },
        Rule {
            patterns: vec![PatternRule::new(
                Pattern::new("WG"),
                Pattern::new("WR"),
                None,
            )],
            kind: RuleKind::One,
            steps: None,
        },
    ]);
    mj.apply_sequence(&sequence);

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

    let sequence = rule_to_sequence(Rule {
        patterns: vec![PatternRule::new(
            Pattern::new("BW"),
            Pattern::new("WW"),
            None,
        )],
        kind: RuleKind::One,
        steps: None,
    });
    mj.apply_sequence(&sequence);

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

    let sequence = rule_to_sequence(Rule {
        patterns: vec![PatternRule::new(
            Pattern::new("WB"),
            Pattern::new("WW"),
            None,
        )],
        kind: RuleKind::One,
        steps: None,
    });
    mj.apply_sequence(&sequence);

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
    let mut mj = MarkovJunior::new('.', 2, 2, None);
    mj.grid = #[rustfmt::skip] vec![
        b'U', b'B',
        b'B', b'U',
    ];

    let sequence = rule_to_sequence(Rule {
        patterns: vec![PatternRule::new(
            Pattern::new("BU/UB"),
            Pattern::new("U*/**"),
            None,
        )],
        kind: RuleKind::One,
        steps: None,
    });
    mj.apply_sequence(&sequence);

    assert_eq!(mj.changes, 1);
    assert_eq!(
        mj.grid,
        #[rustfmt::skip] vec![
            b'U', b'B',
            b'U', b'U',
        ]
    );
}
