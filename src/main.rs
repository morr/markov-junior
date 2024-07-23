use rand::Rng;
use rayon::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pattern {
    data: Vec<char>,
    width: usize,
    height: usize,
}

const PATTERN_DELIMITER: char = '/';
const ANYTHING: char = '*';

impl Pattern {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(PATTERN_DELIMITER).collect();
        let width = parts[0].len();
        let height = parts.len();
        let data: Vec<char> = parts.join("").chars().collect();

        Pattern {
            data,
            width,
            height,
        }
    }

    fn rotate_90(&self) -> Self {
        let mut rotated_data = vec![' '; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                rotated_data[x * self.height + (self.height - 1 - y)] =
                    self.data[y * self.width + x];
            }
        }
        Pattern {
            data: rotated_data,
            width: self.height,
            height: self.width,
        }
    }

    fn rotate_180(&self) -> Self {
        let mut rotated_data = self.data.clone();
        rotated_data.reverse();
        Pattern {
            data: rotated_data,
            width: self.width,
            height: self.height,
        }
    }

    fn rotate_270(&self) -> Self {
        let mut rotated_data = vec![' '; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                rotated_data[(self.width - 1 - x) * self.height + y] =
                    self.data[y * self.width + x];
            }
        }
        Pattern {
            data: rotated_data,
            width: self.height,
            height: self.width,
        }
    }
}

#[derive(Clone)]
struct PatternRule {
    input: Pattern,
    output: Pattern,
    weight: f32,
}

#[derive(Clone, Copy)]
enum RuleKind {
    One,
    All,
    Parallel,
}

struct Rule {
    patterns: Vec<PatternRule>,
    kind: RuleKind,
    steps: Option<usize>,
}

impl Rule {
    fn new(kind: RuleKind, patterns: Vec<PatternRule>, steps: Option<usize>) -> Self {
        let mut rule = Rule {
            patterns: Vec::new(),
            kind,
            steps,
        };
        for pattern in patterns {
            rule.add_pattern(pattern);
        }
        rule
    }

    fn add_pattern(&mut self, pattern: PatternRule) {
        self.patterns.push(pattern.clone());
        self.patterns.push(PatternRule {
            input: pattern.input.rotate_90(),
            output: pattern.output.rotate_90(),
            weight: pattern.weight,
        });
        self.patterns.push(PatternRule {
            input: pattern.input.rotate_180(),
            output: pattern.output.rotate_180(),
            weight: pattern.weight,
        });
        self.patterns.push(PatternRule {
            input: pattern.input.rotate_270(),
            output: pattern.output.rotate_270(),
            weight: pattern.weight,
        });
    }
}

struct MarkovJunior {
    grid: Vec<u8>,
    width: usize,
    height: usize,
    rules: Vec<Rule>,
}

impl MarkovJunior {
    fn new(default: char, width: usize, height: usize) -> Self {
        MarkovJunior {
            grid: vec![default as u8; width * height],
            width,
            height,
            rules: Vec::new(),
        }
    }

    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        for rule_index in 0..self.rules.len() {
            let steps = self.rules[rule_index].steps.unwrap_or(usize::MAX);
            let kind = self.rules[rule_index].kind;

            for _ in 0..steps {
                let any_change = match kind {
                    RuleKind::One => self.apply_one_rule(&mut rng, rule_index),
                    RuleKind::All => self.apply_all_rule(rule_index),
                    RuleKind::Parallel => self.apply_parallel_rule(&mut rng, rule_index),
                };

                if !any_change {
                    break;
                }
            }
        }
    }

    fn apply_one_rule(&mut self, rng: &mut impl Rng, rule_index: usize) -> bool {
        let valid_patterns = self.find_valid_patterns_for_rule(rule_index);

        if valid_patterns.is_empty() {
            return false;
        }

        let total_weight: f32 = valid_patterns.iter().map(|&(_, _, weight, _)| weight).sum();
        let mut choice = rng.gen::<f32>() * total_weight;

        for &(x, y, weight, pattern_index) in &valid_patterns {
            choice -= weight;
            if choice <= 0.0 {
                let output = self.rules[rule_index].patterns[pattern_index]
                    .output
                    .clone();
                self.apply_pattern(x, y, &output);
                return true;
            }
        }

        false
    }

    fn apply_all_rule(&mut self, rule_index: usize) -> bool {
        let valid_patterns = self.find_valid_patterns_for_rule(rule_index);
        let mut applied = false;

        for &(x, y, _, pattern_index) in &valid_patterns {
            let output = self.rules[rule_index].patterns[pattern_index]
                .output
                .clone();
            self.apply_pattern(x, y, &output);
            applied = true;
        }

        applied
    }

    fn apply_parallel_rule(&mut self, rng: &mut impl Rng, rule_index: usize) -> bool {
        let valid_patterns = self.find_valid_patterns_for_rule(rule_index);
        let mut applied = false;
        let mut changes = Vec::new();

        for &(x, y, _, pattern_index) in &valid_patterns {
            if rng.gen_bool(0.5) {
                let output = self.rules[rule_index].patterns[pattern_index]
                    .output
                    .clone();
                changes.push((x, y, output));
                applied = true;
            }
        }

        for (x, y, output) in changes {
            self.apply_pattern(x, y, &output);
        }

        applied
    }

    fn find_valid_patterns_for_rule(&self, rule_index: usize) -> Vec<(usize, usize, f32, usize)> {
        let rule = &self.rules[rule_index];
        (0..self.height)
            .into_par_iter()
            .flat_map_iter(|y| {
                (0..self.width).flat_map(move |x| {
                    rule.patterns
                        .iter()
                        .enumerate()
                        .filter_map(move |(pattern_index, pattern)| {
                            if self.pattern_fits(x, y, &pattern.input) {
                                Some((x, y, pattern.weight, pattern_index))
                            } else {
                                None
                            }
                        })
                })
            })
            .collect()
    }

    fn pattern_fits(&self, x: usize, y: usize, pattern: &Pattern) -> bool {
        if x + pattern.width > self.width || y + pattern.height > self.height {
            return false;
        }

        let result = pattern.data.iter().enumerate().all(|(i, &pattern_char)| {
            let px = i % pattern.width;
            let py = i / pattern.width;
            let grid_char = self.grid[(y + py) * self.width + (x + px)] as char;
            pattern_char == ANYTHING || pattern_char == grid_char
        });

        result
    }

    fn apply_pattern(&mut self, x: usize, y: usize, pattern: &Pattern) {
        for (i, &pattern_char) in pattern.data.iter().enumerate() {
            let px = i % pattern.width;
            let py = i / pattern.width;
            if pattern_char != ANYTHING {
                self.grid[(y + py) * self.width + (x + px)] = pattern_char as u8;
            }
        }
    }

    fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.grid[y * self.width + x] as char);
            }
            println!();
        }
    }
}

fn main() {
    let xml = r#"
    <sequence value="B" width="50" height="50">
      <one in="B" out="W" steps="1"/>
      <one in="B" out="R" steps="1"/>
      <one>
        <rule in="RB" out="RR"/>
        <rule in="WB" out="WW"/>
      </one>
      <all in="RW" out="UU"/>
      <all>
        <rule in="W" out="B"/>
        <rule in="R" out="B"/>
      </all>
      <all in="UB" out="UU" steps="1"/>
      <all in="BU/UB" out="U*/**"/>
      <all in="UB" out="*G"/>
      <one in="B" out="E" steps="13"/>
      <one>
        <rule in="EB" out="*E"/>
        <rule in="GB" out="*G"/>
      </one>
    </sequence>
    "#;

    let mut markov = parse_xml(xml);

    markov.generate();
    markov.print_grid();
}

fn parse_xml(xml: &str) -> MarkovJunior {
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
            vec![PatternRule {
                input: Pattern::new(node.attribute("in").unwrap()),
                output: Pattern::new(node.attribute("out").unwrap()),
                weight: 1.0,
            }]
        } else {
            node.children()
                .filter(|n| n.is_element() && n.tag_name().name() == "rule")
                .map(|n| PatternRule {
                    input: Pattern::new(n.attribute("in").unwrap()),
                    output: Pattern::new(n.attribute("out").unwrap()),
                    weight: 1.0,
                })
                .collect()
        };

        let rule = Rule::new(rule_kind, patterns, steps);
        markov.add_rule(rule);
    }

    markov
}
