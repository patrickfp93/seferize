#[seferize::stringify("EXPECTED_STRUCT_MOD_SAMPLE")]
pub mod my_struct {
    pub const MY_STRUCT: &'static str = "pub struct MyStruct{field_1:usize,field_2:String,}";
    pub struct MyStruct {
        field_1: usize,
        field_2: String,
    }
}
