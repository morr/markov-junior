use crate::*;

pub fn parse_xml(xml: &str) -> MarkovJunior {
    let doc = roxmltree::Document::parse(xml).unwrap();
    let root = doc.root_element();

    let width = root.attribute("width").unwrap().parse().unwrap();
    let height = root.attribute("height").unwrap().parse().unwrap();
    let initial_value = root.attribute("value").unwrap().chars().next().unwrap();

    let mut markov = MarkovJunior::new(initial_value, width, height);

    for node in root.children().filter(|n| n.is_element()) {
        let rule_kind = match node.tag_name().name() {
            "one" => RuleKind::One,
            "all" => RuleKind::All,
            "prl" => RuleKind::Parallel,
            _ => continue,
        };

        let steps = node.attribute("steps").and_then(|s| s.parse().ok());

        let patterns = if node.has_attribute("in") && node.has_attribute("out") {
            vec![PatternRule::new(
                Pattern::new(node.attribute("in").unwrap()),
                Pattern::new(node.attribute("out").unwrap()),
            )]
        } else {
            node.children()
                .filter(|n| n.is_element() && n.tag_name().name() == "rule")
                .map(|n| {
                    PatternRule::new(
                        Pattern::new(n.attribute("in").unwrap()),
                        Pattern::new(n.attribute("out").unwrap()),
                    )
                })
                .collect()
        };

        let rule = Rule::new(rule_kind, patterns, steps);
        markov.add_rule(rule);
    }

    markov
}
