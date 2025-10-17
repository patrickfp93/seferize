#[allow(unused)]
pub use {expected::*,original::*};

#[allow(unused)]
pub mod original {

    #[seferize::stringify("ORIGINAL_MODULE_MOD_SAMPLE_WITH_IGNORE")]
    mod implemetation {
        mod module {
            #[ignore]
            fn name() -> String {
                "module".into()
            }
            #[ignore]
            const CONSTANT: usize= 55; 
            fn hello() -> String {
                "Hello".into()
            }
        }
    }
}

#[allow(unused)]
pub mod expected {

    struct Struct();

    #[seferize::stringify("EXPECTED_MODULE_MOD_SAMPLE_WITH_IGNORE")]
    mod implemetation {

        pub const MODULE_WITH_IGNORE: &'static str =
            "mod module {fn hello() -> String{\"Hello\".into()}}";
        mod module {
            #[ignore]
            fn name() -> String {
                "module".into()
            }
            #[ignore]
            const CONSTANT: usize= 55; 
            fn hello() -> String {
                "Hello".into()
            }
        }
    }
}
