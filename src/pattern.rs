use std::cmp::Ordering;
// use crate::*;

#[derive(Clone, Debug)]
pub struct PatternRule {
    pub input: Pattern,
    pub output: Pattern,
    pub probability: f32,
    pub canonical_key: Option<(usize, usize)>,
}

const DEFAULT_PROBABILITY: f32 = 1.0;

impl PatternRule {
    pub fn new(input: Pattern, output: Pattern, maybe_probability: Option<f32>) -> PatternRule {
        let canonical_key = Self::calculate_canonical_key(input.width, input.height);

        PatternRule {
            input,
            output,
            probability: maybe_probability.unwrap_or(DEFAULT_PROBABILITY),
            canonical_key,
        }
    }

    pub fn calculate_canonical_key(width: usize, height: usize) -> Option<(usize, usize)> {
        if (width == 1 && height == 1) || (width != height) {
            None
        } else {
            Some((width, height))
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct RotatedSeq {
    pub data: Vec<char>,
    pub width: usize,
    pub height: usize,
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
pub const ANYTHING: char = '*';
pub const NOTHING: char = '❌';

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pattern {
    pub line: String,
    pub data: Vec<char>,
    pub width: usize,
    pub height: usize,
    pub rotations: Vec<RotatedSeq>,
    pub unique_rotations: Vec<RotatedSeq>,
    pub canonical_form: Option<RotatedSeq>,
    pub has_wildcards: bool,
}

impl Pattern {
    pub fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(PATTERN_DELIMITER).collect();
        let width = parts[0].len();
        let height = parts.len();
        let data = parts.join("").chars().collect::<Vec<char>>();

        let has_wildcards = data.iter().any(|&char| char == ANYTHING);
        let (maybe_canonical_form, rotations, unique_rotations) =
            Self::compute_canonical_form_and_rotations(&data, width, height, has_wildcards);

        Pattern {
            line: line.to_string(),
            data,
            width,
            height,
            rotations,
            unique_rotations,
            canonical_form: maybe_canonical_form,
            has_wildcards,
        }
    }

    pub fn compute_canonical_form_and_rotations(
        data: &[char],
        width: usize,
        height: usize,
        has_wildcards: bool,
    ) -> (Option<RotatedSeq>, Vec<RotatedSeq>, Vec<RotatedSeq>) {
        if width == 1 && height == 1 {
            let rotation = RotatedSeq {
                data: data.to_vec(),
                width: 1,
                height: 1,
                rotation: 1,
            };

            return (None, [rotation.clone()].to_vec(), [rotation].to_vec());
        }

        let rotations = vec![
            RotatedSeq {
                data: data.to_vec(),
                width,
                height,
                rotation: 1,
            },
            RotatedSeq {
                data: Self::rotate_90(data, width, height),
                width: height,
                height: width,
                rotation: 2,
            },
            RotatedSeq {
                data: Self::rotate_180(data),
                width,
                height,
                rotation: 3,
            },
            RotatedSeq {
                data: Self::rotate_270(data, width, height),
                width: height,
                height: width,
                rotation: 4,
            },
            RotatedSeq {
                data: Self::mirror(data, width),
                width,
                height,
                rotation: -1,
            },
            RotatedSeq {
                data: Self::rotate_90(&Self::mirror(data, width), width, height),
                width: height,
                height: width,
                rotation: -2,
            },
            RotatedSeq {
                data: Self::rotate_180(&Self::mirror(data, width)),
                width,
                height,
                rotation: -3,
            },
            RotatedSeq {
                data: Self::rotate_270(&Self::mirror(data, width), width, height),
                width: height,
                height: width,
                rotation: -4,
            },
        ];

        let mut unique_rotations = Vec::new();
        for rotation in rotations.iter() {
            if !unique_rotations.iter().any(|r: &RotatedSeq| {
                r.data == rotation.data && r.width == rotation.width && r.height == rotation.height
            }) {
                unique_rotations.push(rotation.clone());
            }
        }

        if has_wildcards || width != height {
            (None, rotations, unique_rotations)
        } else {
            let canonical_form = rotations
                .iter()
                .min_by(|a, b| {
                    let data_cmp = a
                        .data
                        .iter()
                        .collect::<String>()
                        .cmp(&b.data.iter().collect::<String>());
                    if data_cmp == Ordering::Equal {
                        match (a.rotation >= 0, b.rotation >= 0) {
                            (true, false) => Ordering::Less,
                            (false, true) => Ordering::Greater,
                            _ => a.rotation.abs().cmp(&b.rotation.abs()),
                        }
                    } else {
                        data_cmp
                    }
                })
                .unwrap()
                .clone();

            (Some(canonical_form), rotations, unique_rotations)
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

    pub fn rollback_rotation(
        data: &[char],
        width: usize,
        height: usize,
        rotation: isize,
    ) -> Vec<char> {
        match rotation {
            -4 => Self::mirror(&Self::rotate_90(data, width, height), width),
            -3 => Self::mirror(&Self::rotate_180(data), width),
            -2 => Self::mirror(&Self::rotate_270(data, width, height), width),
            -1 => Self::mirror(data, width),
            1 => data.to_vec(),
            2 => Self::rotate_270(data, width, height),
            3 => Self::rotate_180(data),
            4 => Self::rotate_90(data, width, height),
            _ => unreachable!(),
        }
    }

    pub fn calculate_relative_rotation(grid_rotation: isize, pattern_rotation: isize) -> isize {
        match (grid_rotation, pattern_rotation) {
            (1, _) => pattern_rotation,
            // (1, 1) => 1,
            // (1, 2) => 2,
            // (1, 3) => 3,
            // (1, 4) => 4,
            // (1, -1) => -1,
            // (1, -2) => -2,
            // (1, -3) => -3,
            // (1, -4) => -4,
            (2, 1) => 4,
            (2, 2) => 1,
            (2, 3) => 2,
            (2, 4) => 3,
            (2, -1) => -4,
            (2, -2) => -1,
            (2, -3) => -2,
            (2, -4) => -3,

            (3, 1) => 3,
            (3, 2) => 4,
            (3, 3) => 1,
            (3, 4) => 2,
            (3, -1) => -3,
            (3, -2) => -4,
            (3, -3) => -1,
            (3, -4) => -2,

            (4, 1) => 2,
            (4, 2) => 3,
            (4, 3) => 4,
            (4, 4) => 1,
            (4, -1) => -2,
            (4, -2) => -3,
            (4, -3) => -4,
            (4, -4) => -1,

            (-1, 1) => -1,
            (-1, 2) => -4,
            (-1, 3) => -3,
            (-1, 4) => -2,
            (-1, -1) => 1,
            (-1, -2) => 4,
            (-1, -3) => 3,
            (-1, -4) => 2,

            (-2, 1) => -2,
            (-2, 2) => -1,
            (-2, 3) => -4,
            (-2, 4) => -3,
            (-2, -1) => 2,
            (-2, -2) => 1,
            (-2, -3) => 4,
            (-2, -4) => 3,

            (-3, 1) => -3,
            (-3, 2) => -2,
            (-3, 3) => -1,
            (-3, 4) => -4,
            (-3, -1) => 3,
            (-3, -2) => 2,
            (-3, -3) => 1,
            (-3, -4) => 4,

            (-4, 1) => -4,
            (-4, 2) => -3,
            (-4, 3) => -2,
            (-4, 4) => -1,
            (-4, -1) => 4,
            (-4, -2) => 3,
            (-4, -3) => 2,
            (-4, -4) => 1,

            _ => unreachable!(),
        }
    }
}
