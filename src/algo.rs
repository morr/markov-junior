use crate::*;
use rand::Rng;
use std::collections::HashMap;

// #[cfg(feature = "parallel")]
// use rayon::prelude::*;

pub struct MarkovJunior {
    pub grid: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub rules: Vec<Rule>,
    pub canonical_forms: HashMap<(usize, usize), Vec<RotatedSeq>>,
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
            // self.print_grid();
            // println!("canonical_forms: {:?}", self.canonical_forms);

            for _step in 0..steps {
                // println!("\nrule_index {rule_index} step {_step}");
                let any_change = match kind {
                    RuleKind::One => self.apply_one_rule(&mut rng, rule_index),
                    RuleKind::All => self.apply_all_rule(rule_index),
                    RuleKind::Parallel => self.apply_parallel_rule(&mut rng, rule_index),
                };

                // println!("any_change: {any_change}");
                // self.print_grid();
                // println!("canonical_forms: {:?}", self.canonical_forms);

                if !any_change {
                    break;
                }
            }
        }
    }

    fn match_patterns_for_rule(&self, rule_index: usize) -> Vec<PatternMatch> {
        let mut valid_patterns = Vec::new();
        let rule = &self.rules[rule_index];

        for y in 0..self.height {
            for x in 0..self.width {
                for (pattern_index, pattern_rule) in rule.patterns.iter().enumerate() {
                    if pattern_rule.input.canonical_form.is_some() {
                        if let Some(rotation) =
                            self.pattern_fits_canonical(x, y, &pattern_rule.input)
                        {
                            valid_patterns.push(PatternMatch {
                                x,
                                y,
                                weight: pattern_rule.weight,
                                pattern_index,
                                rotation,
                            });
                        }
                    } else if let Some(rotation) = self.pattern_fits(x, y, &pattern_rule.input) {
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

    pub fn pattern_fits_canonical(&self, x: usize, y: usize, pattern: &Pattern) -> Option<isize> {
        // ensure pattern definitely fits within the grid boundaries
        if x + pattern.width > self.width || y + pattern.height > self.height {
            return None;
        }

        match PatternRule::calculate_canonical_key(pattern.width, pattern.height) {
            None => {
                // For 1x1 patterns, perform a direct comparison
                let grid_char = self.grid[y * self.width + x] as char;
                if grid_char == pattern.data[0] || pattern.data[0] == ANYTHING {
                    Some(1)
                } else {
                    None
                }
            }
            Some(key) => {
                let precalculated_forms = self
                    .canonical_forms
                    .get(&key)
                    .expect("Canonical form should be precalculated for this key");

                let index = y * self.width + x;
                let grid_canonical_form = &precalculated_forms[index];
                let pattern_canonical_form = &pattern.canonical_form.as_ref().unwrap();

                // println!("\npattern: {:?}", pattern);
                // println!("grid_canonical_form: {:?}", grid_canonical_form);

                if self.compare_canonical_forms(
                    &grid_canonical_form.data,
                    &pattern_canonical_form.data,
                ) {
                    Some(pattern_canonical_form.rotation)
                } else {
                    None
                }
            }
        }
    }

    fn pattern_fits(&self, x: usize, y: usize, pattern: &Pattern) -> bool {
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

    fn compare_canonical_forms(&self, grid_form: &[char], pattern_form: &[char]) -> bool {
        debug_assert_eq!(
            grid_form.len(),
            pattern_form.len(),
            "Canonical forms should have the same length"
        );

        grid_form
            .iter()
            .zip(pattern_form.iter())
            .all(|(&grid_char, &pattern_char)| {
                pattern_char == ANYTHING || grid_char == pattern_char
            })
    }

    fn apply_one_rule(&mut self, rng: &mut impl Rng, rule_index: usize) -> bool {
        let valid_patterns = self.match_patterns_for_rule(rule_index);
        // println!("valid_patterns: {:?}", valid_patterns);

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
                let pattern = pattern_rule.output.clone();

                // println!("apply_pattern({x},{y},{:?},{rotation})", pattern);
                self.apply_pattern(x, y, &pattern, rotation);
                // self.print_grid();
                self.update_canonical_forms(
                    x,
                    y,
                    std::cmp::max(pattern.width, pattern.height),
                    rule_index,
                );

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
            let pattern = pattern_rule.output.clone();

            self.apply_pattern(x, y, &pattern, rotation);
            self.update_canonical_forms(
                x,
                y,
                std::cmp::max(pattern.width, pattern.height),
                rule_index,
            );

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
            self.update_canonical_forms(
                x,
                y,
                std::cmp::max(pattern.width, pattern.height),
                rule_index,
            );
        }

        applied
    }

    pub fn apply_pattern(&mut self, x: usize, y: usize, pattern: &Pattern, rotation: isize) {
        let rotated_output = Pattern::rollback_rotation(
            &pattern.canonical_form.data,
            pattern.width,
            pattern.height,
            rotation,
        );
        let width = match rotation.abs() {
            1 | 3 => pattern.width,
            2 | 4 => pattern.height,
            _ => unreachable!(),
        };

        // println!("rotated_output: {:?}, rotation:{rotation}", rotated_output);

        for (i, &pattern_char) in rotated_output.iter().enumerate() {
            let px = i % width;
            let py = i / width;

            if pattern_char != ANYTHING {
                let index = (y + py) * self.width + (x + px);
                self.grid[index] = pattern_char as u8;
            }
        }
    }

    fn update_canonical_forms(&mut self, x: usize, y: usize, size: usize, rule_index: usize) {
        let from_x = x.saturating_sub(size - 1);
        let to_x = std::cmp::min(x + size - 1, self.width - 1);

        let from_y = y.saturating_sub(size - 1);
        let to_y = std::cmp::min(y + size - 1, self.height - 1);

        // println!(
        //     "update_canonical_forms x:{x}, y:{y}, size:{size}, rule_index:{rule_index}, x_range:{:?}, y_range:{:?}, width:{}, height:{}",
        //     from_x..=to_x,
        //     from_y..=to_y,
        //     self.width,
        //     self.height,
        // );

        for pattern_index in 0..self.rules[rule_index].patterns.len() {
            let pattern_rule = &self.rules[rule_index].patterns[pattern_index];

            let Some(canonical_key) = pattern_rule.canonical_key else {
                continue;
            };
            if !self.canonical_forms.contains_key(&canonical_key) {
                unreachable!();
            }

            for dy in from_y..=to_y {
                for dx in from_x..=to_x {
                    let index = dy * self.width + dx;

                    self.canonical_forms.get_mut(&canonical_key).unwrap()[index] =
                        Self::compute_cell_canonical_form(
                            &self.grid,
                            self.width,
                            self.height,
                            dx,
                            dy,
                            canonical_key.0,
                            canonical_key.1,
                        );
                    // println!(
                    //     "update_canonical_form {:?}/{:?}: {:?}",
                    //     canonical_key,
                    //     index,
                    //     self.canonical_forms.get(&canonical_key).unwrap()[index]
                    // );
                }
            }
        }
    }

    pub fn calculate_canonical_forms(&mut self, rule_index: usize) {
        // println!("calculate_canonical_forms");
        for pattern_index in 0..self.rules[rule_index].patterns.len() {
            let pattern_rule = &self.rules[rule_index].patterns[pattern_index];

            let Some(canonical_key) = pattern_rule.canonical_key else {
                continue;
            };
            if self.canonical_forms.contains_key(&canonical_key) {
                continue;
            }

            let mut canonical_key_forms: Vec<RotatedSeq> =
                Vec::with_capacity(self.width * self.height);

            for y in 0..self.height {
                for x in 0..self.width {
                    // println!("{x}/{y}");
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

    pub fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.grid[y * self.width + x] as char);
            }
            println!();
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
    ) -> RotatedSeq {
        let mut data = Vec::with_capacity(pattern_width * pattern_height);
        for py in 0..pattern_height {
            for px in 0..pattern_width {
                let gx = x + px;
                let gy = y + py;
                if gx < width && gy < height {
                    data.push(grid[gy * width + gx] as char);
                } else {
                    data.push(NOTHING);
                }
            }
        }
        Pattern::compute_canonical_form_and_rotations(&data, pattern_width, pattern_height).0
    }
}
