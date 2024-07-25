use rand::Rng;

// #[cfg(feature = "parallel")]
// use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct PatternRule {
    pub input: Pattern,
    pub output: Pattern,
    pub weight: f32,
}

impl PatternRule {
    pub fn new(input: Pattern, output: Pattern) -> PatternRule {
        PatternRule {
            input,
            output,
            weight: 1.0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pattern {
    data: Vec<char>,
    width: usize,
    height: usize,
    canonical_form: Vec<char>,
    rotation: usize, // 0, 1, 2, or 3 representing 0°, 90°, 180°, 270°
}

pub const PATTERN_DELIMITER: char = '/';
pub const ANYTHING: char = '*';

impl Pattern {
    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(PATTERN_DELIMITER).collect();
        let width = parts[0].len();
        let height = parts.len();
        let data = parts.join("").chars().collect::<Vec<char>>();
        let (canonical_form, rotation) = Self::compute_canonical_form(&data, width, height);

        Pattern {
            data,
            width,
            height,
            canonical_form,
            rotation,
        }
    }

    pub fn compute_canonical_form(data: &[char], width: usize, height: usize) -> (Vec<char>, usize) {
        let rotations = [
            (data.to_vec(), 0),
            (Self::rotate_90(data, width, height), 1),
            (Self::rotate_180(data), 2),
            (Self::rotate_270(data, width, height), 3),
        ];

        rotations
            .into_iter()
            .min_by_key(|(rotated_data, _)| rotated_data.iter().collect::<String>())
            .unwrap()
    }

    pub fn rotate_90(data: &[char], width: usize, height: usize) -> Vec<char> {
        let mut rotated_data = vec![' '; width * height];
        for y in 0..height {
            for x in 0..width {
                rotated_data[x * height + (height - 1 - y)] = data[y * width + x];
            }
        }
        rotated_data
    }

    pub fn rotate_180(data: &[char]) -> Vec<char> {
        let mut rotated_data = data.to_owned();
        rotated_data.reverse();
        rotated_data.to_vec()
    }

    pub fn rotate_270(data: &[char], width: usize, height: usize) -> Vec<char> {
        let mut rotated_data = vec![' '; width * height];
        for y in 0..height {
            for x in 0..width {
                rotated_data[(width - 1 - x) * height + y] = data[y * width + x];
            }
        }
        rotated_data
    }
}

#[derive(Clone, Copy)]
pub enum RuleKind {
    One,
    All,
    Parallel,
}

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

pub struct MarkovJunior {
    pub grid: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub rules: Vec<Rule>,
}

impl MarkovJunior {
    pub fn new(default: char, width: usize, height: usize) -> Self {
        MarkovJunior {
            grid: vec![default as u8; width * height],
            width,
            height,
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        for rule_index in 0..self.rules.len() {
            let steps = self.rules[rule_index]
                .steps
                .unwrap_or(self.width * self.height * 16);
            let kind = self.rules[rule_index].kind;

            for _step in 0..steps {
                // println!("rule_index {rule_index} step {step}");
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
        // Ensure the pattern fits within the grid boundaries
        if x + pattern.width > self.width || y + pattern.height > self.height {
            return false;
        }

        let pattern_data = &pattern.data;
        let grid_width = self.width;
        let grid = &self.grid;

        for py in 0..pattern.height {
            for px in 0..pattern.width {
                let pattern_char = pattern_data[py * pattern.width + px];
                if pattern_char != ANYTHING {
                    let grid_char = grid[(y + py) * grid_width + (x + px)] as char;
                    if pattern_char != grid_char {
                        return false;
                    }
                }
            }
        }

        true
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

    fn apply_pattern(&mut self, x: usize, y: usize, pattern: &Pattern) {
        for (i, &pattern_char) in pattern.data.iter().enumerate() {
            let px = i % pattern.width;
            let py = i / pattern.width;
            if pattern_char != ANYTHING {
                self.grid[(y + py) * self.width + (x + px)] = pattern_char as u8;
            }
        }
    }

    pub fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.grid[y * self.width + x] as char);
            }
            println!();
        }
    }
}
