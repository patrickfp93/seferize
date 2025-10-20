mod utilities_for_test;

#[allow(unused)]
use crate::*;
use rstest::rstest;
#[rstest]
#[case::struture(ORIGINAL_STRUCT_MOD_SAMPLE, EXPECTED_STRUCT_MOD_SAMPLE, "MY_STRUCT")]
#[case::tuple(ORIGINAL_TUPLE_MOD_SAMPLE, EXPECTED_TUPLE_MOD_SAMPLE, "MY_TUPLE")]
#[case::enumerate(
    ORIGINAL_ENUMERATE_MOD_SAMPLE,
    EXPECTED_ENUMERATE_MOD_SAMPLE,
    "MY_ENUMERATE"
)]
#[case::implementation(
    ORIGINAL_IMPL_STRUCT_MOD_SAMPLE,
    EXPECTED_IMPL_STRUCT_MOD_SAMPLE,
    "IMPL_STRUCT"
)]
#[case::implementation_with_ignore(
    ORIGINAL_IMPL_STRUCT_MOD_SAMPLE_WITH_IGNORE,
    EXPECTED_IMPL_STRUCT_MOD_SAMPLE_WITH_IGNORE,
    "IMPL_STRUCT_WITH_IGNORE"
)]
#[case::module_with_ignore(
    ORIGINAL_MODULE_MOD_SAMPLE_WITH_IGNORE,
    EXPECTED_MODULE_MOD_SAMPLE_WITH_IGNORE,
    "MODULE_WITH_IGNORE"
)]
#[case::extract_struct_into(
    ORIGINAL_EXTRACT_STRUCT_MOD_SAMPLE,
    EXPECTED_EXTRACT_STRUCT_MOD_SAMPLE,
    "EXTRACT_STRUCT"
)]
#[case::extract_enumerate_with_into(
    ORIGINAL_EXTRACT_ENUMERATE_MOD_SAMPLE,
    EXPECTED_EXTRACT_ENUMERATE_MOD_SAMPLE,
    "EXTRACT_ENUMERATE_MOD_SAMPLE_WITH_VAR"
)]
#[case::extract_implamentation_with_into(
    ORIGINAL_EXTRACT_IMPL_STRUCT_MOD_SAMPLE,
    EXPECTED_EXTRACT_IMPL_STRUCT_MOD_SAMPLE,
    "EXTRACT_IMPLAMENTATION_WITH_METHOD_NAME"
)]
fn check_stringify(
    #[case] original_struct_mod_str: &'static str,
    #[case] expected_struct_mod_str: &'static str,
    #[case] name_attr_str: &'static str,
) {
    use syn::parse_str;
    use utilities_for_test::*;

    let original: TokenStream =
        parse_str(&original_struct_mod_str.replace("#[cfg(false)]", "")).unwrap();

    let original: TokenStream = extract_content_from_module(&parse_quote!(#original)).unwrap();

    let expected: TokenStream =
        parse_str(&expected_struct_mod_str.replace("#[cfg(false)]", "")).unwrap();
    let expected = extract_content_from_module(&parse_quote!(#expected)).unwrap();

    let generated = stringify(name_attr_str.into(), original);

    assert_eq!(
        generated.to_string().replace(" ", ""),
        expected.to_string().replace(" ", "")
    )
}

#[rstest]
#[case::simple_method(ORIGINAL_SIMPLE_METHOD, EXPECTED_SIMPLE_METHOD)]
fn check_expose_for_tests(
    #[case] original_method_str: &'static str,
    #[case] expected_method_str: &'static str,
) {
    use crate::tests::utilities_for_test::extract_content_from_module;

    let original_method: TokenStream = parse_str(&original_method_str).unwrap();
    let original_method: TokenStream = extract_content_from_module(&parse_quote!(#original_method)).unwrap();
    let expected_method: TokenStream = parse_str(&expected_method_str).unwrap();
    let expected_method: TokenStream = extract_content_from_module(&parse_quote!(#expected_method)).unwrap();

    let generate_method = expose_for_tests(TokenStream::new(), original_method);    

    assert_eq!(generate_method.to_string(),expected_method.to_string())
}
