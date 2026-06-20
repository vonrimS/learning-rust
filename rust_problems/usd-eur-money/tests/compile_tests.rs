#[test]
fn test_compile_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/mix_currency.rs"); // Test will be passed if the code isn't compiled
}