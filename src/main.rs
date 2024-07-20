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

struct MarkovJunior {
    grid: Vec<char>,
    width: usize,
    height: usize,
    patterns: Vec<(Pattern, Pattern, f32)>,
}

impl MarkovJunior {
    fn new(width: usize, height: usize) -> Self {
        MarkovJunior {
            grid: vec!['.'; width * height],
            width,
            height,
            patterns: Vec::new(),
        }
    }

    fn add_pattern(&mut self, input: Pattern, output: Pattern, weight: f32) {
        self.patterns.push((input.clone(), output.clone(), weight));
        self.patterns.push((input.rotate_90(), output.rotate_90(), weight));
        self.patterns.push((input.rotate_180(), output.rotate_180(), weight));
        self.patterns.push((input.rotate_270(), output.rotate_270(), weight));
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
                .map(|&(_, index)| self.patterns[index].2)
                .sum();
            let mut choice = rng.gen::<f32>() * total_weight;

            for &((x, y), index) in &valid_patterns {
                choice -= self.patterns[index].2;
                if choice <= 0.0 {
                    let output = self.patterns[index].1.clone();
                    self.apply_pattern(x, y, &output);
                    break;
                }
            }
        }
    }

    fn find_valid_patterns(&self) -> Vec<((usize, usize), usize)> {
        let mut valid_patterns = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                for (index, pattern) in self.patterns.iter().enumerate() {
                    if self.pattern_fits(x, y, &pattern.0) {
                        valid_patterns.push(((x, y), index));
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

    // Add some example patterns
    markov.add_pattern(
        Pattern::new(vec!['?', '.'], 2, 1),
        Pattern::new(vec!['#', '#'], 2, 1),
        1.0,
    );
    markov.add_pattern(
        Pattern::new(vec!['.', '?'], 2, 1),
        Pattern::new(vec!['#', '#'], 2, 1),
        1.0,
    );
    markov.add_pattern(
        Pattern::new(vec!['?', '.', '?'], 3, 1),
        Pattern::new(vec!['#', '#', '#'], 3, 1),
        0.5,
    );
    markov.add_pattern(
        Pattern::new(vec!['?', '.', '?', '.'], 2, 2),
        Pattern::new(vec!['#', '#', '#', '#'], 2, 2),
        0.3,
    );

    markov.generate(1000);
    markov.print_grid();
}
