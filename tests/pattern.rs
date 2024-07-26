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
