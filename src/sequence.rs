use crate::*;

pub struct Sequence {
    pub rules: Vec<Rule>,
}

#[derive(Clone, Copy, Debug)]
pub enum RuleKind {
    One,
    All,
    Parallel,
}

#[derive(Debug)]
pub struct Rule {
    pub patterns: Vec<PatternRule>,
    pub kind: RuleKind,
    pub steps: Option<usize>,
}

impl Rule {
    pub fn new(kind: RuleKind, patterns: Vec<PatternRule>, steps: Option<usize>) -> Self {
        let mut rule = Rule {
            patterns: Vec::new(),
            kind,
            steps,
        };
        for pattern in patterns {
            rule.patterns.push(pattern);
        }
        rule
    }
}
