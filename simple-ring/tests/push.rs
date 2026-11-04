use simple_ring::Polynomial;

#[test]
fn test_push_appends_at_end() {
    let mut poly = Polynomial::<u64>::zeros(4); // [0, 0, 0, 0]

    poly.push(7);

    assert_eq!(poly.len(), 5); // The pushed coefficient is appended at the end
    assert_eq!(poly.to_vec(), vec![0u64, 0, 0, 0, 7]);
}

#[test]
fn test_push_appends_to_non_zero_polynomial() {
    let mut poly = Polynomial::<u64>::zeros(2); // [0, 0]
    poly.push(1);
    poly.push(2); // Appends at the end, not at the beginning

    assert_eq!(poly.len(), 4);
    assert_eq!(poly.to_vec(), vec![0u64, 0, 1, 2]);
}

#[test]
fn test_push_multiple_times() {
    let mut poly = Polynomial::<u64>::zeros(1); // [0]
    poly.push(1);
    poly.push(2);
    poly.push(3);

    assert_eq!(poly.len(), 4);
    assert_eq!(poly.to_vec(), vec![0u64, 1, 2, 3]);
}
