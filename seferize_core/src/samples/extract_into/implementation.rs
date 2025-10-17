#[allow(unused)]
pub use {expected::*, original::*};
#[allow(unused)]
pub mod original {
    struct Struct();
    #[seferize::stringify("ORIGINAL_EXTRACT_IMPL_STRUCT_MOD_SAMPLE")]
    mod implementation {
        #[cfg(false)]
        impl super::Struct {
            #[stringify("EXTRACT_IMPL_METHOD_NAME")]
            fn name(&self) -> String {
                "SimpleStruct".into()
            }
            fn hello() -> String {
                "Hello".into()
            }
        }
    }
}
#[allow(unused)]
pub mod expected {
    struct Struct();

    #[seferize::stringify("EXPECTED_EXTRACT_IMPL_STRUCT_MOD_SAMPLE")]
    mod implementation {
        pub const EXTRACT_IMPLAMENTATION_WITH_METHOD_NAME: &'static str = "impl super::Struct {fn name(&self) -> String {\"SimpleStruct\".into()}fn hello() -> String{\"Hello\".into()}}";
        pub const EXTRACT_IMPL_METHOD_NAME: &'static str ="fn name(&self) -> String {\"SimpleStruct\".into()}";
        impl super::Struct {
            fn name(&self) -> String {
                "SimpleStruct".into()
            }
            fn hello() -> String {
                "Hello".into()
            }
        }
    }
}
