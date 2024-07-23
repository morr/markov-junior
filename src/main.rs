use rand::Rng;

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
    grid: Vec<char>,
    width: usize,
    height: usize,
    rules: Vec<Rule>,
}

impl MarkovJunior {
    fn new(default: char, width: usize, height: usize) -> Self {
        MarkovJunior {
            grid: vec![default; width * height],
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
                let mut any_change = false;

                match kind {
                    RuleKind::One => {
                        if self.apply_one_rule(&mut rng, rule_index) {
                            any_change = true;
                        }
                    }
                    RuleKind::All => {
                        any_change |= self.apply_all_rule(rule_index);
                    }
                    RuleKind::Parallel => {
                        any_change |= self.apply_parallel_rule(&mut rng, rule_index);
                    }
                }

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
                self.apply_pattern(x, y, output);
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
            self.apply_pattern(x, y, output);
            applied = true;
        }

        applied
    }

    fn apply_parallel_rule(&mut self, rng: &mut impl Rng, rule_index: usize) -> bool {
        let valid_patterns = self.find_valid_patterns_for_rule(rule_index);
        let mut applied = false;
        let mut new_grid = self.grid.clone();

        for &(x, y, _, pattern_index) in &valid_patterns {
            if rng.gen_bool(0.5) {
                let output = self.rules[rule_index].patterns[pattern_index]
                    .output
                    .clone();
                Self::apply_pattern_to_grid(&mut new_grid, self.width, x, y, &output);
                applied = true;
            }
        }

        if applied {
            self.grid = new_grid;
        }

        applied
    }

    fn find_valid_patterns_for_rule(&self, rule_index: usize) -> Vec<(usize, usize, f32, usize)> {
        let mut valid_patterns = Vec::new();
        let rule = &self.rules[rule_index];

        for y in 0..self.height {
            for x in 0..self.width {
                for (pattern_index, pattern) in rule.patterns.iter().enumerate() {
                    if self.pattern_fits(x, y, &pattern.input) {
                        valid_patterns.push((x, y, pattern.weight, pattern_index));
                    }
                }
            }
        }

        valid_patterns
    }

    fn pattern_fits(&self, x: usize, y: usize, pattern: &Pattern) -> bool {
        if x + pattern.width > self.width || y + pattern.height > self.height {
            return false;
        }

        for py in 0..pattern.height {
            for px in 0..pattern.width {
                let grid_char = self.grid[(y + py) * self.width + (x + px)];
                let pattern_char = pattern.data[py * pattern.width + px];

                if pattern_char != ANYTHING && pattern_char != grid_char {
                    return false;
                }
            }
        }

        true
    }

    fn apply_pattern(&mut self, x: usize, y: usize, pattern: Pattern) {
        Self::apply_pattern_to_grid(&mut self.grid, self.width, x, y, &pattern);
    }

    fn apply_pattern_to_grid(
        grid: &mut [char],
        width: usize,
        x: usize,
        y: usize,
        pattern: &Pattern,
    ) {
        for py in 0..pattern.height {
            for px in 0..pattern.width {
                let pattern_char = pattern.data[py * pattern.width + px];
                if pattern_char != ANYTHING {
                    grid[(y + py) * width + (x + px)] = pattern_char;
                }
            }
        }
    }

    fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.grid[y * self.width + x]);
            }
            println!();
        }
    }
}

fn main() {
    // let mut markov = MarkovJunior::new('.', 20, 20);
    //
    // // Create rules with patterns and steps
    // let rule1 = Rule::new(
    //     RuleKind::One,
    //     vec![PatternRule {
    //         input: Pattern::new("*."),
    //         output: Pattern::new("##"),
    //         weight: 1.0,
    //     }],
    //     Some(5), // Apply this rule 5 times
    // );
    //
    // let rule2 = Rule::new(
    //     RuleKind::All,
    //     vec![PatternRule {
    //         input: Pattern::new(".*."),
    //         output: Pattern::new("###"),
    //         weight: 0.5,
    //     }],
    //     None, // Apply this rule until no more changes
    // );
    //
    // let rule3 = Rule::new(
    //     RuleKind::Parallel,
    //     vec![Patter nRule {
    //         input: Pattern::new("*./*."),
    //         output: Pattern::new("##/##"),
    //         weight: 0.3,
    //     }],
    //     Some(10), // Apply this rule 10 times
    // );
    //
    // // Add rules to MarkovJunior
    // markov.add_rule(rule1);
    // markov.add_rule(rule2);
    // markov.add_rule(rule3);

    let mut markov = MarkovJunior::new('B', 20, 20);

    markov.add_rule(Rule::new(
        RuleKind::One,
        vec![PatternRule {
            input: Pattern::new("B"),
            output: Pattern::new("W"),
            weight: 1.0,
        }],
        Some(1),
    ));
    markov.add_rule(Rule::new(
        RuleKind::One,
        vec![PatternRule {
            input: Pattern::new("B"),
            output: Pattern::new("R"),
            weight: 1.0,
        }],
        Some(1),
    ));
    markov.add_rule(Rule::new(
        RuleKind::One,
        vec![PatternRule {
            input: Pattern::new("RB"),
            output: Pattern::new("RR"),
            weight: 1.0,
        }, PatternRule {
            input: Pattern::new("WB"),
            output: Pattern::new("WW"),
            weight: 1.0,
        }],
        None,
    ));

    markov.generate();
    markov.print_grid();
}
