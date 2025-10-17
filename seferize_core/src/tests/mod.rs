mod utilities_for_test;

#[allow(unused)]
use crate::*;
use rstest::rstest;
#[rstest]
#[case::struture(ORIGINAL_STRUCT_MOD_SAMPLE, EXPECTED_STRUCT_MOD_SAMPLE, "MY_STRUCT")]
#[case::tuple(ORIGINAL_TUPLE_MOD_SAMPLE, EXPECTED_TUPLE_MOD_SAMPLE, "MY_TUPLE")]
#[case::enumerate(ORIGINAL_ENUMERATE_MOD_SAMPLE, EXPECTED_ENUMERATE_MOD_SAMPLE, "MY_ENUMERATE")]
#[case::implementation(ORIGINAL_IMPL_STRUCT_MOD_SAMPLE, EXPECTED_IMPL_STRUCT_MOD_SAMPLE, "IMPL_STRUCT")]
#[case::implementation_with_ignore(ORIGINAL_IMPL_STRUCT_MOD_SAMPLE_WITH_IGNORE, EXPECTED_IMPL_STRUCT_MOD_SAMPLE_WITH_IGNORE, "IMPL_STRUCT_WITH_IGNORE")]
#[case::module_with_ignore(ORIGINAL_MODULE_MOD_SAMPLE_WITH_IGNORE, EXPECTED_MODULE_MOD_SAMPLE_WITH_IGNORE, "MODULE_WITH_IGNORE")]
#[case::extract_struct_into(ORIGINAL_EXTRACT_STRUCT_MOD_SAMPLE, EXPECTED_EXTRACT_STRUCT_MOD_SAMPLE, "EXTRACT_STRUCT")]
#[case::extract_implamentation_with_into(ORIGINAL_EXTRACT_IMPL_STRUCT_MOD_SAMPLE, EXPECTED_EXTRACT_IMPL_STRUCT_MOD_SAMPLE, "EXTRACT_IMPLAMENTATION_WITH_METHOD_NAME")]
#[case::extract_enumerate_with_into(ORIGINAL_EXTRACT_ENUMERATE_MOD_SAMPLE, EXPECTED_EXTRACT_ENUMERATE_MOD_SAMPLE, "EXTRACT_ENUMERATE_MOD_SAMPLE_WITH_VAR")]
fn check_stringify(
    #[case] original_struct_mod_str: &'static str,
    #[case] expected_struct_mod_str: &'static str,
    #[case] name_attr_str: &'static str,
) {
    use syn::parse_str; 
    use utilities_for_test::*;

    let original: TokenStream = parse_str(&original_struct_mod_str.replace("#[cfg(false)]", "")).unwrap();

    let original: TokenStream = extract_content_from_module(&parse_quote!(#original)).unwrap();

    let expected: TokenStream = parse_str(&expected_struct_mod_str.replace("#[cfg(false)]", "")).unwrap();
    let expected = extract_content_from_module(&parse_quote!(#expected)).unwrap();

    let generated = stringify(name_attr_str.into(), original);

    assert_eq!(generated.to_string().replace(" ", ""), expected.to_string().replace(" ", ""))

}


