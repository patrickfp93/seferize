pub use original::*;
pub use expected::*;

pub mod original {
    struct Struct();

    #[seferize::stringify("ORIGINAL_IMPL_STRUCT_MOD_SAMPLE")]
    mod implemetation {
        impl super::Struct {
            fn name(&self) -> String {
                "SimpleStruct".into()
            }
        }
    }
}

pub mod expected {

    struct Struct();

    #[seferize::stringify("EXPECTED_IMPL_STRUCT_MOD_SAMPLE")]
    mod implemetation {

        pub const IMPL_STRUCT: &'static str = "impl super::Struct {fn name(&self) -> String {\"SimpleStruct\".into()}}"; 
        impl super::Struct {
            fn name(&self) -> String {
                "SimpleStruct".into()
            }
        }
    }
}
