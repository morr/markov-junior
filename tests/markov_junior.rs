use std::collections::HashMap;

use markov_junior::*;

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
