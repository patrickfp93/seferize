#[allow(unused)]
pub use {original::*,expected::*};

#[allow(unused)]
pub mod original {
    struct Struct();

    #[seferize::stringify("ORIGINAL_IMPL_STRUCT_MOD_SAMPLE_WITH_IGNORE")]
    mod implemetation {
        impl super::Struct {            
            #[ignore]
            fn name(&self) -> String {
                "SimpleStruct".into()
            }
            fn hello() -> String{
                "Hello".into()
            }
        }
    }
}
#[allow(unused)]
pub mod expected {

    struct Struct();

    #[seferize::stringify("EXPECTED_IMPL_STRUCT_MOD_SAMPLE_WITH_IGNORE")]
    mod implemetation {

        pub const IMPL_STRUCT_WITH_IGNORE: &'static str = "impl super::Struct {fn hello() -> String{\"Hello\".into()}}"; 
        impl super::Struct {
            #[ignore]
            fn name(&self) -> String {
                "SimpleStruct".into()
            }            
            fn hello() -> String{
                "Hello".into()
            }
        }
    }
}
