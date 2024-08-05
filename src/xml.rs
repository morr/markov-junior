use crate::*;

pub fn parse_xml(xml: &str, seed: Option<u64>) -> MarkovJunior {
    let doc = roxmltree::Document::parse(xml).unwrap();
    let root = doc.root_element();

    let width = root.attribute("width").unwrap().parse().unwrap();
    let height = root.attribute("height").unwrap().parse().unwrap();
    let initial_fill = root.attribute("fill").unwrap().chars().next().unwrap();

    let mut markov = MarkovJunior::new(initial_fill, width, height, seed);

    for rule_node in root.children().filter(|n| n.is_element()) {
        let rule_kind = match rule_node.tag_name().name() {
            "one" => RuleKind::One,
            "all" => RuleKind::All,
            "prl" => RuleKind::Parallel,
            _ => continue,
        };

        let steps = rule_node.attribute("steps").and_then(|s| s.parse().ok());

        let patterns = if rule_node.has_attribute("in") && rule_node.has_attribute("out") {
            vec![PatternRule::new(
                Pattern::new(rule_node.attribute("in").unwrap()),
                Pattern::new(rule_node.attribute("out").unwrap()),
                rule_node
                    .attribute("p")
                    .map(|str| str.parse::<f32>().unwrap()),
            )]
        } else {
            rule_node
                .children()
                .filter(|n| n.is_element() && n.tag_name().name() == "rule")
                .map(|n| {
                    PatternRule::new(
                        Pattern::new(n.attribute("in").unwrap()),
                        Pattern::new(n.attribute("out").unwrap()),
                        n.attribute("p").map(|str| str.parse::<f32>().unwrap()),
                    )
                })
                .collect()
        };

        let rule = Rule::new(rule_kind, patterns, steps);
        markov.add_rule(rule);
    }

    markov
}
