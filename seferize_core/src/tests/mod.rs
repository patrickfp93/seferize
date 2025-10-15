mod filter {}
mod utilities_for_test;


use super::*;
use rstest::rstest;

#[rstest]
#[case::struture(ORIGINAL_STRUCT_MOD_SAMPLE, EXPECTED_STRUCT_MOD_SAMPLE, "MY_STRUCT")]
#[case::tuple(ORIGINAL_TUPLE_MOD_SAMPLE, EXPECTED_TUPLE_MOD_SAMPLE, "MY_TUPLE")]
#[case::enumerate(ORIGINAL_ENUMERATE_MOD_SAMPLE, EXPECTED_ENUMERATE_MOD_SAMPLE, "MY_ENUMERATE")]
fn check_stringify(
    #[case] original_struct_mod_str: &'static str,
    #[case] expected_struct_mod_str: &'static str,
    #[case] name_attr_str: &'static str,
) {
    use syn::parse_str;
    use utilities_for_test::*;

    let original: TokenStream = parse_str(original_struct_mod_str).unwrap();

    let original: TokenStream = extract_content_from_module(&parse_quote!(#original)).unwrap();

    let expected: TokenStream = parse_str(expected_struct_mod_str).unwrap();
    let expected = extract_content_from_module(&parse_quote!(#expected)).unwrap();

    let name_attr: TokenStream = parse_str(name_attr_str).unwrap();

    let generated = stringify(name_attr, original);

    assert_eq!(generated.to_string().replace(" ", ""), expected.to_string().replace(" ", ""))

}


