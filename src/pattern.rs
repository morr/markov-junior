use std::cmp::Ordering;
// use crate::*;

#[derive(Clone, Debug)]
pub struct PatternRule {
    pub input: Pattern,
    pub output: Pattern,
    pub weight: f32,
    pub canonical_key: Option<(usize, usize)>,
}

impl PatternRule {
    pub fn new(input: Pattern, output: Pattern) -> PatternRule {
        let canonical_key = Self::calculate_canonical_key(input.width, input.height);

        PatternRule {
            input,
            output,
            weight: 1.0,
            canonical_key,
        }
    }

    pub fn calculate_canonical_key(width: usize, height: usize) -> Option<(usize, usize)> {
        if width == 1 && height == 1 {
            None
        } else {
            let size = std::cmp::max(width, height);
            Some((size, size))
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CanonicalForm {
    pub data: Vec<char>,
    pub rotation: isize, // 1, 2, 3, or 4 representing 0°, 90°, 180°, 270°, -1, -2, -3, or -4 representing mirrored 0°, 90°, 180°, 270°
}

#[derive(Debug)]
pub struct PatternMatch {
    pub x: usize,
    pub y: usize,
    pub weight: f32,
    pub pattern_index: usize,
    pub rotation: isize,
}

pub const PATTERN_DELIMITER: char = '/';
// pub const ANYTHING: char = '*';
pub const ANYTHING_INPUT: char = '*';
pub const ANYTHING: char = u8::MAX as char; // the value is set to make it to be sorted to the bottom
pub const NOTHING: char = '❌';

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pattern {
    pub data: Vec<char>,
    pub width: usize,
    pub height: usize,
    pub canonical_form: CanonicalForm,
    pub canonical_form_2: Option<CanonicalForm>,
}

impl Pattern {
    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(PATTERN_DELIMITER).collect();
        let original_width = parts[0].len();
        let original_height = parts.len();
        let square_size = std::cmp::max(original_width, original_height);

        let mut data = vec![ANYTHING; square_size * square_size];

        for (y, part) in parts.iter().enumerate() {
            for (x, c) in part.chars().enumerate() {
                data[y * square_size + x] = if c == ANYTHING_INPUT { ANYTHING } else { c };
            }
        }

        let canonical_form = Self::compute_canonical_form(&data, square_size, square_size);
        let canonical_form_2 = if original_width != original_height {
            Some(Self::compute_canonical_form_mirrored(
                &canonical_form.data,
                square_size,
                square_size,
            ))
        } else {
            None
        };

        Pattern {
            data,
            width: square_size,
            height: square_size,
            canonical_form,
            canonical_form_2,
        }
    }

    pub fn compute_canonical_form(data: &[char], width: usize, height: usize) -> CanonicalForm {
        let rotations = [
            CanonicalForm {
                data: data.to_vec(),
                rotation: 1,
            },
            CanonicalForm {
                data: Self::rotate_90(data, width, height),
                rotation: 2,
            },
            CanonicalForm {
                data: Self::rotate_180(data),
                rotation: 3,
            },
            CanonicalForm {
                data: Self::rotate_270(data, width, height),
                rotation: 4,
            },
            CanonicalForm {
                data: Self::mirror(data, width),
                rotation: -1,
            },
            CanonicalForm {
                data: Self::rotate_90(&Self::mirror(data, width), width, height),
                rotation: -2,
            },
            CanonicalForm {
                data: Self::rotate_180(&Self::mirror(data, width)),
                rotation: -3,
            },
            CanonicalForm {
                data: Self::rotate_270(&Self::mirror(data, width), width, height),
                rotation: -4,
            },
        ];

        rotations
            .into_iter()
            .min_by(|a, b| {
                let data_cmp = a
                    .data
                    .iter()
                    .collect::<String>()
                    .cmp(&b.data.iter().collect::<String>());
                if data_cmp == Ordering::Equal {
                    a.rotation.abs().cmp(&b.rotation.abs())
                } else {
                    data_cmp
                }
            })
            .unwrap()
    }

    pub fn compute_canonical_form_mirrored(
        data: &[char],
        width: usize,
        _height: usize,
    ) -> CanonicalForm {
        CanonicalForm {
            data: Self::mirror(&Self::rotate_90(data, width, width), width),
            rotation: 0,
        }
    }

    pub fn mirror(data: &[char], width: usize) -> Vec<char> {
        let mut mirrored = Vec::with_capacity(data.len());
        for chunk in data.chunks(width) {
            mirrored.extend(chunk.iter().rev());
        }
        mirrored
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

    pub fn apply_rotation(
        data: &[char],
        width: usize,
        height: usize,
        rotation: isize,
    ) -> Vec<char> {
        match rotation.abs() {
            1 => {
                if rotation > 0 {
                    data.to_vec()
                } else {
                    Self::mirror(data, width)
                }
            }
            2 => {
                let data = if rotation > 0 {
                    data.to_vec()
                } else {
                    Self::mirror(data, width)
                };
                Self::rotate_90(&data, width, height)
            }
            3 => {
                let data = if rotation > 0 {
                    data.to_vec()
                } else {
                    Self::mirror(data, width)
                };
                Self::rotate_180(&data)
            }
            4 => {
                let data = if rotation > 0 {
                    data.to_vec()
                } else {
                    Self::mirror(data, width)
                };
                Self::rotate_270(&data, width, height)
            }
            _ => unreachable!(),
        }
    }
}
