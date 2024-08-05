use crate::*;
use roxmltree::Node;

fn parse_rule_or_sequence(node: &Node) -> RuleOrSequence {
    match node.tag_name().name() {
        "sequence" => RuleOrSequence::Sequence(parse_sequence(node)),
        _ => RuleOrSequence::Rule(parse_rule(node)),
    }
}

fn parse_sequence(node: &Node) -> Sequence {
    let rules = node
        .children()
        .filter(|n| n.is_element())
        .map(|n| parse_rule_or_sequence(&n))
        .collect();
    // let steps = node.attribute("steps").and_then(|s| s.parse().ok());
    // Sequence::new(rules, steps)
    Sequence { vec: rules }
}

fn parse_rule(node: &Node) -> Rule {
    let rule_kind = match node.tag_name().name() {
        "one" => RuleKind::One,
        "all" => RuleKind::All,
        "prl" => RuleKind::Parallel,
        _ => panic!("Unknown rule kind: {}", node.tag_name().name()),
    };

    let steps = node.attribute("steps").and_then(|s| s.parse().ok());

    let patterns = if node.has_attribute("in") && node.has_attribute("out") {
        vec![PatternRule::new(
            Pattern::new(node.attribute("in").unwrap()),
            Pattern::new(node.attribute("out").unwrap()),
            node.attribute("p").and_then(|s| s.parse().ok()),
        )]
    } else {
        node.children()
            .filter(|n| n.is_element() && n.tag_name().name() == "rule")
            .map(|n| {
                PatternRule::new(
                    Pattern::new(n.attribute("in").unwrap()),
                    Pattern::new(n.attribute("out").unwrap()),
                    n.attribute("p").and_then(|s| s.parse().ok()),
                )
            })
            .collect()
    };

    Rule::new(rule_kind, patterns, steps)
}

pub fn parse_xml(xml: &str, seed: Option<u64>) -> (MarkovJunior, Sequence) {
    let doc = roxmltree::Document::parse(xml).unwrap();
    let root = doc.root_element();

    let width = root.attribute("width").unwrap().parse().unwrap();
    let height = root.attribute("height").unwrap().parse().unwrap();
    let initial_fill = root.attribute("fill").unwrap().chars().next().unwrap();

    (MarkovJunior::new(initial_fill, width, height, seed), parse_sequence(&root))
}
