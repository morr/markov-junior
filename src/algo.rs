use std::collections::HashMap;

use rand::Rng;

// #[cfg(feature = "parallel")]
// use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct PatternRule {
    pub input: Pattern,
    pub output: Pattern,
    pub weight: f32,
    pub canonical_key: Option<(usize, usize)>,
}

impl PatternRule {
    pub fn new(input: Pattern, output: Pattern) -> PatternRule {
        let canonical_key = (
            std::cmp::max(input.width, input.height),
            std::cmp::min(input.width, input.height),
        );

        PatternRule {
            input,
            output,
            weight: 1.0,
            canonical_key: if canonical_key.0 > 1 {
                Some(canonical_key)
            } else {
                None
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pattern {
    pub data: Vec<char>,
    pub width: usize,
    pub height: usize,
    pub canonical_form: CanonicalForm,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CanonicalForm {
    pub data: Vec<char>,
    pub rotation: usize, // 0, 1, 2, or 3 representing 0°, 90°, 180°, 270°
}

pub const PATTERN_DELIMITER: char = '/';
pub const ANYTHING: char = '*';

impl Pattern {
    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(PATTERN_DELIMITER).collect();
        let width = parts[0].len();
        let height = parts.len();
        let data = parts.join("").chars().collect::<Vec<char>>();
        let canonical_form = Self::compute_canonical_form(&data, width, height);

        Pattern {
            data,
            width,
            height,
            canonical_form,
        }
    }

    pub fn compute_canonical_form(data: &[char], width: usize, height: usize) -> CanonicalForm {
        let rotations = [
            CanonicalForm {
                data: data.to_vec(),
                rotation: 0,
            },
            CanonicalForm {
                data: Self::rotate_90(data, width, height),
                rotation: 1,
            },
            CanonicalForm {
                data: Self::rotate_180(data),
                rotation: 2,
            },
            CanonicalForm {
                data: Self::rotate_270(data, width, height),
                rotation: 3,
            },
        ];

        rotations
            .into_iter()
            .min_by_key(|CanonicalForm { data, .. }| data.iter().collect::<String>())
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

pub struct PatternMatch {
    pub x: usize,
    pub y: usize,
    pub weight: f32,
    pub pattern_index: usize,
    pub rotation: usize,
}

pub struct MarkovJunior {
    pub grid: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub rules: Vec<Rule>,
    pub canonical_forms: HashMap<(usize, usize), Vec<CanonicalForm>>,
}

impl MarkovJunior {
    pub fn new(default: char, width: usize, height: usize) -> Self {
        MarkovJunior {
            grid: vec![default as u8; width * height],
            width,
            height,
            rules: Vec::new(),
            canonical_forms: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        for rule_index in 0..self.rules.len() {
            let rule = &self.rules[rule_index];
            let steps = rule.steps.unwrap_or(self.width * self.height * 16);
            let kind = rule.kind;

            self.calculate_canonical_forms(rule_index);

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

    fn calculate_canonical_forms(&mut self, rule_index: usize) {
        for pattern_index in 0..self.rules[rule_index].patterns.len() {
            let pattern_rule = &self.rules[rule_index].patterns[pattern_index];

            let Some(canonical_key) = pattern_rule.canonical_key else {
                continue;
            };

            if self.canonical_forms.contains_key(&canonical_key) {
                continue;
            }

            let mut canonical_key_forms: Vec<CanonicalForm> =
                Vec::with_capacity(self.width * self.height);

            for y in 0..self.height {
                for x in 0..self.width {
                    // let index = y * self.width + x;

                    canonical_key_forms.push(Self::compute_cell_canonical_form(
                        &self.grid,
                        self.width,
                        self.height,
                        x,
                        y,
                        canonical_key.0,
                        canonical_key.1,
                    ));
                }
            }

            self.canonical_forms
                .insert(canonical_key, canonical_key_forms);
        }
    }

    fn match_patterns_for_rule(&self, rule_index: usize) -> Vec<PatternMatch> {
        let mut valid_patterns = Vec::new();
        let rule = &self.rules[rule_index];

        for y in 0..self.height {
            for x in 0..self.width {
                for (pattern_index, pattern_rule) in rule.patterns.iter().enumerate() {
                    if let Some(rotation) = self.pattern_fits_canonical(x, y, &pattern_rule.input) {
                        valid_patterns.push(PatternMatch {
                            x,
                            y,
                            weight: pattern_rule.weight,
                            pattern_index,
                            rotation,
                        });
                    }
                }
            }
        }

        valid_patterns
    }

    fn pattern_fits_canonical(&self, x: usize, y: usize, pattern: &Pattern) -> Option<usize> {
        if x + pattern.width > self.width || y + pattern.height > self.height {
            return None;
        }

        let canonical_form = Self::compute_cell_canonical_form(
            &self.grid,
            self.width,
            self.height,
            x,
            y,
            pattern.width,
            pattern.height,
        );

        if canonical_form.data == pattern.canonical_form.data {
            Some((canonical_form.rotation + 4 - pattern.canonical_form.rotation) % 4)
        } else {
            None
        }
    }

    // fn pattern_fits(&self, x: usize, y: usize, pattern: &Pattern) -> bool {
    //     // Ensure the pattern fits within the grid boundaries
    //     if x + pattern.width > self.width || y + pattern.height > self.height {
    //         return false;
    //     }
    //
    //     let pattern_data = &pattern.data;
    //     let grid_width = self.width;
    //     let grid = &self.grid;
    //
    //     for py in 0..pattern.height {
    //         for px in 0..pattern.width {
    //             let pattern_char = pattern_data[py * pattern.width + px];
    //             if pattern_char != ANYTHING {
    //                 let grid_char = grid[(y + py) * grid_width + (x + px)] as char;
    //                 if pattern_char != grid_char {
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //
    //     true
    // }

    fn apply_one_rule(&mut self, rng: &mut impl Rng, rule_index: usize) -> bool {
        let valid_patterns = self.match_patterns_for_rule(rule_index);

        if valid_patterns.is_empty() {
            return false;
        }

        let total_weight: f32 = valid_patterns
            .iter()
            .map(|pattern_match| pattern_match.weight)
            .sum();
        let mut choice = rng.gen::<f32>() * total_weight;

        for &PatternMatch {
            x,
            y,
            weight,
            pattern_index,
            rotation,
        } in &valid_patterns
        {
            choice -= weight;

            if choice <= 0.0 {
                let pattern_rule = &self.rules[rule_index].patterns[pattern_index];
                let output = pattern_rule.output.clone();

                self.apply_pattern(x, y, &output, rotation);
                return true;
            }
        }

        false
    }

    fn apply_all_rule(&mut self, rule_index: usize) -> bool {
        let valid_patterns = self.match_patterns_for_rule(rule_index);
        let mut applied = false;

        for &PatternMatch {
            x,
            y,
            pattern_index,
            rotation,
            ..
        } in &valid_patterns
        {
            let pattern_rule = &self.rules[rule_index].patterns[pattern_index];
            let output = pattern_rule.output.clone();

            self.apply_pattern(x, y, &output, rotation);
            applied = true;
        }

        applied
    }

    fn apply_parallel_rule(&mut self, rng: &mut impl Rng, rule_index: usize) -> bool {
        let valid_patterns = self.match_patterns_for_rule(rule_index);
        let mut applied = false;
        let mut changes = Vec::new();

        for &PatternMatch {
            x,
            y,
            pattern_index,
            rotation,
            ..
        } in &valid_patterns
        {
            if rng.gen_bool(0.5) {
                let pattern_rule = &self.rules[rule_index].patterns[pattern_index];
                let output = pattern_rule.output.clone();

                changes.push((x, y, output, rotation));
                applied = true;
            }
        }

        for (x, y, pattern, rotation) in changes {
            self.apply_pattern(x, y, &pattern, rotation);
        }

        applied
    }

    // fn apply_pattern(&mut self, x: usize, y: usize, pattern: &Pattern) {
    //     for (i, &pattern_char) in pattern.data.iter().enumerate() {
    //         let px = i % pattern.width;
    //         let py = i / pattern.width;
    //         if pattern_char != ANYTHING {
    //             self.grid[(y + py) * self.width + (x + px)] = pattern_char as u8;
    //         }
    //     }
    // }

    fn apply_pattern(&mut self, x: usize, y: usize, pattern: &Pattern, rotation: usize) {
        let rotated_output = match rotation {
            0 => pattern.data.clone(),
            1 => Pattern::rotate_270(&pattern.data, pattern.width, pattern.height),
            2 => Pattern::rotate_180(&pattern.data),
            3 => Pattern::rotate_90(&pattern.data, pattern.width, pattern.height),
            _ => unreachable!(),
        };

        let width = match rotation {
            0 | 2 => pattern.width,
            1 | 3 => pattern.height,
            _ => unreachable!(),
        };

        for (i, &pattern_char) in rotated_output.iter().enumerate() {
            let px = i % width;
            let py = i / width;
            if pattern_char != ANYTHING {
                let index = (y + py) * self.width + (x + px);
                self.grid[index] = pattern_char as u8;
                // self.update_canonical_form(x + px, y + py);
            }
        }
    }

    fn compute_cell_canonical_form(
        grid: &[u8],
        width: usize,
        height: usize,
        x: usize,
        y: usize,
        pattern_width: usize,
        pattern_height: usize,
    ) -> CanonicalForm {
        let mut data = Vec::with_capacity(pattern_width * pattern_height);
        for py in 0..pattern_height {
            for px in 0..pattern_width {
                let gx = x + px;
                let gy = y + py;
                if gx < width && gy < height {
                    data.push(grid[gy * width + gx] as char);
                } else {
                    data.push(' ');
                }
            }
        }
        Pattern::compute_canonical_form(&data, pattern_width, pattern_height)
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
