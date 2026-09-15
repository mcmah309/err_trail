#[test]
fn invalid_field_syntax() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/*.rs");
}

#[cfg(not(any(feature = "tracing", feature = "log", feature = "defmt")))]
#[test]
fn invalid_formatting_without_backend() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/no_backend/*.rs");
}
