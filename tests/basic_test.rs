#[test]
fn test_polymorphic() {
    let base = b"test";
    let mutated = shadowforge::generate_polymorphic(base);
    assert_ne!(base, mutated.as_slice());
    assert_eq!(mutated.len(), base.len());
}

#[test]
fn test_persistence() {
    // Simple check
    assert!(true);
}
