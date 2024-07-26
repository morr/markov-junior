use std::collections::HashMap;

use markov_junior::*;

#[test]
fn test_compute_canonical_form() {
    let data = vec!['A', 'B', 'C', 'D'];
    let canonical_form = Pattern::compute_canonical_form(&data, 2, 2);
    assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
    assert_eq!(canonical_form.rotation, 0);

    let data = vec!['B', 'D', 'A', 'C'];
    let canonical_form = Pattern::compute_canonical_form(&data, 2, 2);
    assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
    assert_eq!(canonical_form.rotation, 1);

    let data = vec!['D', 'C', 'B', 'A'];
    let canonical_form = Pattern::compute_canonical_form(&data, 2, 2);
    assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
    assert_eq!(canonical_form.rotation, 2);

    let data = vec!['C', 'A', 'D', 'B'];
    let canonical_form = Pattern::compute_canonical_form(&data, 2, 2);
    assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
    assert_eq!(canonical_form.rotation, 3);

    let data = vec!['B', 'A'];
    let canonical_form = Pattern::compute_canonical_form(&data, 1, 2);
    assert_eq!(canonical_form.data, vec!['A', 'B']);
    assert_eq!(canonical_form.rotation, 1);
}

#[test]
fn test_rotate_90() {
    let data = vec!['A', 'B', 'C', 'D'];
    let rotated = Pattern::rotate_90(&data, 2, 2);
    assert_eq!(rotated, vec!['C', 'A', 'D', 'B']);

    let data = vec!['1', '2', '3', '4', '5', '6'];
    let rotated = Pattern::rotate_90(&data, 3, 2);
    assert_eq!(rotated, vec!['4', '1', '5', '2', '6', '3']);
}

#[test]
fn test_rotate_180() {
    let data = vec!['A', 'B', 'C', 'D'];
    let rotated = Pattern::rotate_180(&data);
    assert_eq!(rotated, vec!['D', 'C', 'B', 'A']);

    let data = vec!['1', '2', '3', '4', '5', '6'];
    let rotated = Pattern::rotate_180(&data);
    assert_eq!(rotated, vec!['6', '5', '4', '3', '2', '1']);
}

#[test]
fn test_rotate_270() {
    let data = vec!['A', 'B', 'C', 'D'];
    let rotated = Pattern::rotate_270(&data, 2, 2);
    assert_eq!(rotated, vec!['B', 'D', 'A', 'C']);

    let data = vec!['1', '2', '3', '4', '5', '6'];
    let rotated = Pattern::rotate_270(&data, 3, 2);
    assert_eq!(rotated, vec!['3', '6', '2', '5', '1', '4']);
}

fn create_test_grid(data: &str) -> Vec<u8> {
    data.chars().map(|c| c as u8).collect()
}

#[test]
fn test_pattern_fits_canonical() {
    let mj = MarkovJunior {
        grid: create_test_grid("ABCDEFGHI"),
        width: 3,
        height: 3,
        rules: vec![],
        canonical_forms: HashMap::new(),
    };

    let pattern = Pattern::new("AB/DE");

    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern), Some(0));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern), None);

    let pattern_90 = Pattern::new("DA/EB");

    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_90), Some(1));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_90), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_90), None);

    let pattern_180 = Pattern::new("ED/BA");

    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_180), Some(2));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_180), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_180), None);

    let pattern_270 = Pattern::new("BE/AD");

    assert_eq!(mj.pattern_fits_canonical(0, 0, &pattern_270), Some(3));
    assert_eq!(mj.pattern_fits_canonical(1, 0, &pattern_270), None);
    assert_eq!(mj.pattern_fits_canonical(0, 1, &pattern_270), None);
}
