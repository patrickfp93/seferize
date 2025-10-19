#[allow(unused)]
#[seferize_base::stringify("ORIGINAL_STRUCT_MOD_SAMPLE")]
pub mod original {
    pub struct MyStruct {
        pub field_1: usize,
        pub field_2: String,
    }
}
#[allow(unused)]
#[seferize_base::stringify("EXPECTED_STRUCT_MOD_SAMPLE")]
pub mod expected {
    pub const MY_STRUCT: &'static str = "pub struct MyStruct{pub field_1:usize,pub field_2:String,}";
    pub struct MyStruct {
        pub field_1: usize,
        pub field_2: String,
    }
}
