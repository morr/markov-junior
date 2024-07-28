#![feature(stmt_expr_attributes)]
use markov_junior::*;

#[test]
fn test_patterns_have_no_canonical_form() {
    let pattern = Pattern::new("WG");
    assert!(pattern.canonical_form.is_none());

    let pattern = Pattern::new("W");
    assert!(pattern.canonical_form.is_none());

    let pattern = Pattern::new("WW/B*");
    assert!(pattern.canonical_form.is_none());
}

#[test]
fn test_patterns_canonical_form() {
    let pattern = Pattern::new("WW/BB");
    let canonical_form = pattern.canonical_form.unwrap();

    assert_eq!(
        canonical_form.data,
        #[rustfmt::skip] vec![
            'B', 'B',
            'W', 'W',
        ]
    );
    assert_eq!(canonical_form.rotation, 3);
}

#[test]
fn test_rollback_rotation() {
    let pattern = Pattern::new("WG/WG");
    let canonical_form = pattern.canonical_form.unwrap();

    assert_eq!(
        Pattern::rollback_rotation(
            &canonical_form.data,
            pattern.width,
            pattern.height,
            canonical_form.rotation
        ),
        pattern.data
    );

    for rotated_data in pattern.rotations {
        println!("{}", rotated_data.rotation);
        assert_eq!(
            Pattern::rollback_rotation(
                &rotated_data.data,
                pattern.width,
                pattern.height,
                rotated_data.rotation
            ),
            pattern.data
        );
    }
}

#[test]
fn test_non_square_pattern_2x1() {
    let pattern = Pattern::new("AB");
    assert_eq!(pattern.width, 2);
    assert_eq!(pattern.height, 1);
    assert_eq!(pattern.data, vec!['A', 'B']);
    assert_eq!(pattern.rotations.len(), 8);
    assert_eq!(pattern.unique_rotations.len(), 4);
}

// #[test]
// fn test_non_square_pattern_2x3() {
//     let pattern = Pattern::new("AB/CD/EF");
//     assert_eq!(pattern.width, 2);
//     assert_eq!(pattern.height, 3);
//     assert_eq!(
//         pattern.data,
//         #[rustfmt::skip] vec![
//             'A', 'B',
//             'C', 'D',
//             'E', 'F',
//         ]
//     );
//     assert_eq!(pattern.rotations.len(), 1);
// }
//
// #[test]
// fn test_wide_pattern() {
//     let pattern = Pattern::new("ABC");
//     assert_eq!(pattern.width, 3);
//     assert_eq!(pattern.height, 3);
//     assert_eq!(
//         pattern.data,
//         #[rustfmt::skip] vec![
//             'A', 'B', 'C',
//             ANYTHING, ANYTHING, ANYTHING,
//             ANYTHING, ANYTHING, ANYTHING
//         ]
//     );
// }
//
// #[test]
// fn test_tall_pattern() {
//     let pattern = Pattern::new("A/B/C");
//     assert_eq!(pattern.width, 3);
//     assert_eq!(pattern.height, 3);
//     assert_eq!(
//         pattern.data,
//         #[rustfmt::skip] vec![
//             'A', ANYTHING, ANYTHING,
//             'B', ANYTHING, ANYTHING,
//             'C', ANYTHING, ANYTHING
//         ]
//     );
// }
//
// #[test]
// fn test_pattern_with_anything_input() {
//     let pattern = Pattern::new("A*/C*");
//     assert_eq!(pattern.width, 2);
//     assert_eq!(pattern.height, 2);
//     assert_eq!(
//         pattern.data,
//         #[rustfmt::skip] vec![
//             'A', ANYTHING,
//             'C', ANYTHING
//         ]
//     );
// }
//
// #[test]
// fn test_rotations() {
//     let data = vec!['A', 'B', 'C', 'D'];
//     let rotated = Pattern::rotate_90(&data, 2, 2);
//     assert_eq!(rotated, vec!['C', 'A', 'D', 'B']);
//
//     let data = vec!['1', '2', '3', '4', '5', '6'];
//     let rotated = Pattern::rotate_90(&data, 3, 2);
//     assert_eq!(rotated, vec!['4', '1', '5', '2', '6', '3']);
//
//     let data = vec!['A', 'B', 'C', 'D'];
//     let rotated = Pattern::rotate_180(&data);
//     assert_eq!(rotated, vec!['D', 'C', 'B', 'A']);
//
//     let data = vec!['1', '2', '3', '4', '5', '6'];
//     let rotated = Pattern::rotate_180(&data);
//     assert_eq!(rotated, vec!['6', '5', '4', '3', '2', '1']);
//
//     let data = vec!['A', 'B', 'C', 'D'];
//     let rotated = Pattern::rotate_270(&data, 2, 2);
//     assert_eq!(rotated, vec!['B', 'D', 'A', 'C']);
//
//     let data = vec!['1', '2', '3', '4', '5', '6'];
//     let rotated = Pattern::rotate_270(&data, 3, 2);
//     assert_eq!(rotated, vec!['3', '6', '2', '5', '1', '4']);
// }
//
// #[test]
// fn test_mirror() {
//     let data = vec!['A', 'B', 'C', 'D'];
//     let mirrored = Pattern::mirror(&data, 2);
//     assert_eq!(mirrored, vec!['B', 'A', 'D', 'C']);
//
//     let data = vec!['1', '2', '3', '4', '5', '6'];
//     let mirrored = Pattern::mirror(&data, 3);
//     assert_eq!(mirrored, vec!['3', '2', '1', '6', '5', '4']);
// }
//
// #[test]
// fn test_compute_canonical_form() {
//     let data = vec!['A', 'B', 'C', 'D'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, 1);
//
//     let data = vec!['B', 'D', 'A', 'C'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, 2);
//
//     let data = vec!['D', 'C', 'B', 'A'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, 3);
//
//     let data = vec!['C', 'A', 'D', 'B'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, 4);
//
//     let data = vec!['B', 'A'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 1, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B']);
//     assert_eq!(canonical_form.rotation, 2);
// }
//
// #[test]
// fn test_compute_canonical_form_with_mirror() {
//     let data = vec!['B', 'A', 'D', 'C'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, -1);
//
//     let data = vec!['C', 'D', 'A', 'B'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, -3);
// }
//
// #[test]
// fn test_compute_canonical_form_1x1() {
//     let data = vec!['A'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 1, 1).0;
//     assert_eq!(canonical_form.data, vec!['A']);
//     assert_eq!(canonical_form.rotation, 1);
// }
//
// #[test]
// fn test_compute_canonical_form_2x2_no_rotation() {
//     let data = vec!['A', 'B', 'C', 'D'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, 1);
// }
//
// #[test]
// fn test_compute_canonical_form_2x2_90_rotation() {
//     let data = vec!['B', 'D', 'A', 'C'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, 2);
// }
//
// #[test]
// fn test_compute_canonical_form_2x2_180_rotation() {
//     let data = vec!['D', 'C', 'B', 'A'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, 3);
// }
//
// #[test]
// fn test_compute_canonical_form_2x2_270_rotation() {
//     let data = vec!['C', 'A', 'D', 'B'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, 4);
// }
//
// #[test]
// fn test_compute_canonical_form_3x3() {
//     let data = vec!['C', 'F', 'I', 'B', 'E', 'H', 'A', 'D', 'G'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 3, 3).0;
//     assert_eq!(
//         canonical_form.data,
//         vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I']
//     );
//     assert_eq!(canonical_form.rotation, 2);
// }
//
// #[test]
// fn test_compute_canonical_form_2x3() {
//     let mut data = vec!['B', 'D', 'F', 'A', 'C', 'E'];
//     data.reverse();
//
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 3).0;
//     assert_eq!(canonical_form.data, vec!['B', 'D', 'F', 'A', 'C', 'E']);
//     assert_eq!(canonical_form.rotation, 3);
// }
//
// #[test]
// fn test_compute_canonical_form_with_repeated_characters() {
//     let data = vec!['A', 'A', 'B', 'B'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['A', 'A', 'B', 'B']);
//     assert_eq!(canonical_form.rotation, 1);
// }
//
// #[test]
// fn test_compute_canonical_form_with_wildcard() {
//     let data = vec!['*', 'B', 'C', 'D'];
//     let canonical_form = Pattern::compute_canonical_form_and_rotations(&data, 2, 2).0;
//     assert_eq!(canonical_form.data, vec!['*', 'B', 'C', 'D']);
//     assert_eq!(canonical_form.rotation, 1);
// }
