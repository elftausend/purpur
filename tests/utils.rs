
#[test]
fn test_this() {
    let rows = 9;
    let mut x = vec![0usize; rows];

    for idx in 1..rows {
        x[idx] = idx;
    }

    let x1: Vec<usize> = (0..rows).into_iter().collect();
    assert_eq!(x, x1);
}