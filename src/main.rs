use rand::Rng;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pattern {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn new(data: Vec<char>, width: usize, height: usize) -> Self {
        Pattern { data, width, height }
    }

    fn rotate_90(&self) -> Self {
        let mut rotated_data = vec![' '; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                rotated_data[x * self.height + (self.height - 1 - y)] = self.data[y * self.width + x];
            }
        }
        Pattern::new(rotated_data, self.height, self.width)
    }

    fn rotate_180(&self) -> Self {
        let mut rotated_data = self.data.clone();
        rotated_data.reverse();
        Pattern::new(rotated_data, self.width, self.height)
    }

    fn rotate_270(&self) -> Self {
        let mut rotated_data = vec![' '; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                rotated_data[(self.width - 1 - x) * self.height + y] = self.data[y * self.width + x];
            }
        }
        Pattern::new(rotated_data, self.height, self.width)
    }
}

struct Rule {
    patterns: Vec<(Pattern, Pattern, f32)>,
}

impl Rule {
    fn new() -> Self {
        Rule { patterns: Vec::new() }
    }

    fn add_pattern(&mut self, input: Pattern, output: Pattern, weight: f32) {
        self.patterns.push((input.clone(), output.clone(), weight));
        self.patterns.push((input.rotate_90(), output.rotate_90(), weight));
        self.patterns.push((input.rotate_180(), output.rotate_180(), weight));
        self.patterns.push((input.rotate_270(), output.rotate_270(), weight));
    }
}

struct MarkovJunior {
    grid: Vec<char>,
    width: usize,
    height: usize,
    rules: Vec<Rule>,
}

impl MarkovJunior {
    fn new(width: usize, height: usize) -> Self {
        MarkovJunior {
            grid: vec!['.'; width * height],
            width,
            height,
            rules: Vec::new(),
        }
    }

    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn generate(&mut self, iterations: usize) {
        let mut rng = rand::thread_rng();

        for _ in 0..iterations {
            let valid_patterns = self.find_valid_patterns();

            if valid_patterns.is_empty() {
                break;
            }

            let total_weight: f32 = valid_patterns
                .iter()
                .map(|&(_, _, weight, _, _)| weight)
                .sum();
            let mut choice = rng.gen::<f32>() * total_weight;

            for &(x, y, weight, rule_index, pattern_index) in &valid_patterns {
                choice -= weight;
                if choice <= 0.0 {
                    let output = self.rules[rule_index].patterns[pattern_index].1.clone();
                    self.apply_pattern(x, y, &output);
                    break;
                }
            }
        }
    }

    fn find_valid_patterns(&self) -> Vec<(usize, usize, f32, usize, usize)> {
        let mut valid_patterns = Vec::new();
        for (rule_index, rule) in self.rules.iter().enumerate() {
            for y in 0..self.height {
                for x in 0..self.width {
                    for (pattern_index, (input, _, weight)) in rule.patterns.iter().enumerate() {
                        if self.pattern_fits(x, y, input) {
                            valid_patterns.push((x, y, *weight, rule_index, pattern_index));
                        }
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

                if pattern_char != '?' && pattern_char != grid_char {
                    return false;
                }
            }
        }

        true
    }

    fn apply_pattern(&mut self, x: usize, y: usize, pattern: &Pattern) {
        for py in 0..pattern.height {
            for px in 0..pattern.width {
                let pattern_char = pattern.data[py * pattern.width + px];
                if pattern_char != '?' {
                    self.grid[(y + py) * self.width + (x + px)] = pattern_char;
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
    let mut markov = MarkovJunior::new(20, 20);

    // Create rules and add patterns
    let mut rule1 = Rule::new();
    rule1.add_pattern(
        Pattern::new(vec!['?', '.'], 2, 1),
        Pattern::new(vec!['#', '#'], 2, 1),
        1.0,
    );
    rule1.add_pattern(
        Pattern::new(vec!['.', '?'], 2, 1),
        Pattern::new(vec!['#', '#'], 2, 1),
        1.0,
    );

    let mut rule2 = Rule::new();
    rule2.add_pattern(
        Pattern::new(vec!['?', '.', '?'], 3, 1),
        Pattern::new(vec!['#', '#', '#'], 3, 1),
        0.5,
    );

    let mut rule3 = Rule::new();
    rule3.add_pattern(
        Pattern::new(vec!['?', '.', '?', '.'], 2, 2),
        Pattern::new(vec!['#', '#', '#', '#'], 2, 2),
        0.3,
    );

    // Add rules to MarkovJunior
    markov.add_rule(rule1);
    markov.add_rule(rule2);
    markov.add_rule(rule3);

    markov.generate(1000);
    markov.print_grid();
}
