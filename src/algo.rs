use crate::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::{collections::HashMap, ops::Range};

// #[cfg(feature = "parallel")]
// use rayon::prelude::*;

pub struct MarkovJunior {
    pub grid: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub rules: Vec<Rule>,
    pub canonical_forms: HashMap<(usize, usize), Vec<RotatedSeq>>,
    pub patterns_applied_counter: usize,
    pub rng: ChaCha8Rng,
    pub seed: u64,
}

impl MarkovJunior {
    pub fn new(default: char, width: usize, height: usize, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| rand::thread_rng().gen());

        MarkovJunior {
            grid: vec![default as u8; width * height],
            width,
            height,
            rules: Vec::new(),
            canonical_forms: HashMap::new(),
            patterns_applied_counter: 0,
            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
        }
    }

    pub fn new_grid(data: &str, width: usize, height: usize, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| rand::thread_rng().gen());

        MarkovJunior {
            grid: data.chars().map(|c| c as u8).collect(),
            width,
            height,
            rules: Vec::new(),
            canonical_forms: HashMap::new(),
            patterns_applied_counter: 0,
            rng: ChaCha8Rng::seed_from_u64(seed),
            seed,
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn generate(&mut self) {
        for rule_index in 0..self.rules.len() {
            let rule = &self.rules[rule_index];
            let steps = rule.steps.unwrap_or(self.width * self.height * 16);
            let kind = rule.kind;

            let current_patterns_applied_counter = self.patterns_applied_counter;
            self.precompute_canonical_forms(rule_index);
            let mut cache = self.compute_cache(rule_index, &(0..self.width), &(0..self.height));

            for _step in 0..steps {
                let any_change = match kind {
                    RuleKind::One => self.apply_one_rule(rule_index, &mut cache),
                    RuleKind::All => self.apply_all_rule(rule_index, &mut cache),
                    RuleKind::Parallel => self.apply_parallel_rule(rule_index, &mut cache),
                };

                if !any_change {
                    break;
                }
            }
            println!(" rule: {rule_index} patterns_applied: {}", self.patterns_applied_counter - current_patterns_applied_counter);
            // self.print_grid();
        }
    }

    pub fn pattern_fits_canonical(&self, x: usize, y: usize, pattern: &Pattern) -> Option<isize> {
        // ensure pattern definitely fits within the grid boundaries
        if x + pattern.width > self.width || y + pattern.height > self.height {
            return None;
        }

        let canonical_key =
            PatternRule::calculate_canonical_key(pattern.width, pattern.height).unwrap();
        let precalculated_forms = self
            .canonical_forms
            .get(&canonical_key)
            .expect("Canonical form should be precalculated for this key");

        let index = y * self.width + x;
        let grid_canonical_form = &precalculated_forms[index];
        let pattern_canonical_form = &pattern.canonical_form.as_ref().unwrap();

        if grid_canonical_form.data == pattern_canonical_form.data {
            Some(pattern_canonical_form.rotation)
        } else {
            None
        }
    }

    pub fn pattern_fits(&self, x: usize, y: usize, pattern: &Pattern) -> Option<isize> {
        let grid_width = self.width;
        let grid = &self.grid;

        'rotated_seq: for rotated_seq in pattern.unique_rotations.iter() {
            let pattern_data = &rotated_seq.data;

            // ensure pattern definitely fits within the grid boundaries
            if x + rotated_seq.width > self.width || y + rotated_seq.height > self.height {
                continue;
            }

            for py in 0..rotated_seq.height {
                for px in 0..rotated_seq.width {
                    let pattern_char = pattern_data[py * rotated_seq.width + px];
                    if pattern_char != ANYTHING {
                        let grid_char = grid[(y + py) * grid_width + (x + px)] as char;
                        if pattern_char != grid_char {
                            continue 'rotated_seq;
                        }
                    }
                }
            }

            return Some(rotated_seq.rotation);
        }

        None
    }

    fn apply_one_rule(
        &mut self,
        rule_index: usize,
        cache: &mut HashMap<(usize, usize), Vec<PatternMatch>>,
    ) -> bool {
        let total_weight: f32 = cache.iter().fold(0.0, |memo, (_, pattern_matches)| {
            pattern_matches
                .iter()
                .fold(memo, |memo2, pattern_match| memo2 + pattern_match.weight)
        });

        if total_weight == 0.0 {
            return false;
        }

        let mut choice = self.rng.gen::<f32>() * total_weight;
        let mut selected_change = None;

        for pattern_match in Self::cache_iter(cache) {
            choice -= pattern_match.weight;

            if choice <= 0.0 {
                let pattern_rule = &self.rules[rule_index].patterns[pattern_match.pattern_index];
                let pattern = pattern_rule.output.clone();
                let is_canonical_key = pattern_rule.canonical_key.is_some();

                selected_change = Some((
                    pattern_match.x,
                    pattern_match.y,
                    pattern,
                    pattern_match.rotation,
                    is_canonical_key,
                ));
                break;
            }
        }

        if let Some((x, y, pattern, rotation, is_canonical_key)) = selected_change {
            self.apply_pattern(x, y, &pattern, rotation);

            let size = std::cmp::max(pattern.width, pattern.height);
            let x_range = Self::x_range(x, size, self.width);
            let y_range = Self::x_range(y, size, self.height);
            if is_canonical_key {
                self.update_canonical_forms(&x_range, &y_range, rule_index);
            }
            cache.extend(self.compute_cache(rule_index, &x_range, &y_range));

            return true;
        }

        false
    }

    fn apply_all_rule(
        &mut self,
        rule_index: usize,
        cache: &mut HashMap<(usize, usize), Vec<PatternMatch>>,
    ) -> bool {
        let mut applied = false;
        let mut changes = Vec::new();

        for pattern_match in Self::cache_iter(cache) {
            let pattern_rule = &self.rules[rule_index].patterns[pattern_match.pattern_index];
            let pattern = pattern_rule.output.clone();
            let is_canonical_key = pattern_rule.canonical_key.is_some();

            self.apply_pattern(
                pattern_match.x,
                pattern_match.y,
                &pattern,
                pattern_match.rotation,
            );
            changes.push((
                pattern_match.x,
                pattern_match.y,
                pattern.width,
                pattern.height,
                is_canonical_key,
            ));
            applied = true;
        }

        for (x, y, pattern_width, pattern_height, is_canonical_key) in changes {
            let x_range = Self::x_range(x, pattern_width, self.width);
            let y_range = Self::x_range(y, pattern_height, self.height);
            if is_canonical_key {
                self.update_canonical_forms(&x_range, &x_range, rule_index);
            }
            cache.extend(self.compute_cache(rule_index, &x_range, &y_range));
        }

        applied
    }

    fn apply_parallel_rule(
        &mut self,
        rule_index: usize,
        cache: &mut HashMap<(usize, usize), Vec<PatternMatch>>,
    ) -> bool {
        let mut applied = false;
        let mut changes = Vec::new();

        for pattern_match in Self::cache_iter(cache) {
            let pattern_rule = &self.rules[rule_index].patterns[pattern_match.pattern_index];
            let output = pattern_rule.output.clone();

            changes.push((
                pattern_match.x,
                pattern_match.y,
                output,
                pattern_match.rotation,
                pattern_rule.canonical_key.is_some(),
            ));
            applied = true;
        }

        for (x, y, pattern, rotation, is_canonical_key) in changes {
            self.apply_pattern(x, y, &pattern, rotation);

            let x_range = Self::x_range(x, pattern.width, self.width);
            let y_range = Self::x_range(y, pattern.height, self.height);
            if is_canonical_key {
                self.update_canonical_forms(&x_range, &x_range, rule_index);
            }
            cache.extend(self.compute_cache(rule_index, &x_range, &y_range));
        }

        applied
    }

    pub fn apply_pattern(&mut self, x: usize, y: usize, pattern: &Pattern, rotation: isize) {
        self.patterns_applied_counter += 1;

        let rotated_seq = pattern
            .rotations
            .iter()
            .find(|&rotated_seq| rotated_seq.rotation == rotation)
            .unwrap();

        for (i, &pattern_char) in rotated_seq.data.iter().enumerate() {
            let px = i % rotated_seq.width;
            let py = i / rotated_seq.width;
            if pattern_char != ANYTHING {
                let index = (y + py) * self.width + (x + px);
                self.grid[index] = pattern_char as u8;
            }
        }
    }

    fn update_canonical_forms(
        &mut self,
        x_range: &Range<usize>,
        y_range: &Range<usize>,
        rule_index: usize,
    ) {
        for pattern_index in 0..self.rules[rule_index].patterns.len() {
            let pattern_rule = &self.rules[rule_index].patterns[pattern_index];

            let Some(canonical_key) = pattern_rule.canonical_key else {
                continue;
            };
            if !self.canonical_forms.contains_key(&canonical_key) {
                unreachable!();
            }

            for dy in y_range.clone() {
                for dx in x_range.clone() {
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
                }
            }
        }
    }

    pub fn precompute_canonical_forms(&mut self, rule_index: usize) {
        for pattern_rule in self.rules[rule_index].patterns.iter() {
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

    pub fn compute_cache(
        &self,
        rule_index: usize,
        x_range: &Range<usize>,
        y_range: &Range<usize>,
    ) -> HashMap<(usize, usize), Vec<PatternMatch>> {
        y_range
            .clone()
            .flat_map(|y| x_range.clone().map(move |x| (x, y)))
            .map(|(x, y)| {
                let valid_patterns = self.rules[rule_index]
                    .patterns
                    .iter()
                    .enumerate()
                    .filter_map(|(pattern_index, pattern_rule)| {
                        let maybe_pattern_match = if pattern_rule.input.canonical_form.is_some() {
                            self.pattern_fits_canonical(x, y, &pattern_rule.input)
                        } else {
                            self.pattern_fits(x, y, &pattern_rule.input)
                        };

                        maybe_pattern_match.map(|rotation| PatternMatch {
                            x,
                            y,
                            weight: pattern_rule.weight,
                            pattern_index,
                            rotation,
                        })
                    })
                    .collect::<Vec<_>>();

                ((x, y), valid_patterns)
            })
            .collect()
    }

    pub fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.grid[y * self.width + x] as char);
            }
            println!();
        }
    }

    // fn cached_patterns(cache: &HashMap<(usize, usize), Vec<PatternMatch>>) -> Vec<&PatternMatch> {
    //     cache
    //         .iter()
    //         .flat_map(|(_k, pattern_matches)| pattern_matches)
    //         .collect()
    // }

    fn cache_iter(
        cache: &HashMap<(usize, usize), Vec<PatternMatch>>,
    ) -> impl Iterator<Item = &PatternMatch> {
        cache
            .values()
            .flat_map(|pattern_matches| pattern_matches.iter())
    }

    fn x_range(x: usize, size: usize, grid_size: usize) -> Range<usize> {
        let from_x = x.saturating_sub(size - 1);
        let to_x = std::cmp::min(x + size, grid_size);

        from_x..to_x
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
        Pattern::compute_canonical_form_and_rotations(&data, pattern_width, pattern_height, false)
            .0
            .unwrap()
    }
}
