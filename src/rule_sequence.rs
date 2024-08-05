use crate::*;

#[derive(Debug)]
pub struct Sequence {
    pub vec: Vec<RuleOrSequence>
}

#[derive(Debug)]
pub enum RuleOrSequence {
    Rule(Rule),
    Sequence(Sequence),
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

// it is implemented for tests so rule can be converted into iterator
impl IntoIterator for Rule {
    type Item = Rule;
    type IntoIter = std::vec::IntoIter<Rule>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
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
