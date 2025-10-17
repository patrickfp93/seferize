#[seferize::stringify("ORIGINAL_EXTRACT_STRUCT_MOD_SAMPLE")]
mod original {            
    #[cfg(false)]
    pub struct ExtractStruct {
        pub field_1: usize,
        #[stringify("FIELD_2")]
        pub field_2: String,
    }
}

#[seferize::stringify("EXPECTED_EXTRACT_STRUCT_MOD_SAMPLE")]
pub mod expected {
    pub const EXTRACT_STRUCT: &'static str =
        "pub struct ExtractStruct{pub field_1:usize,pub field_2:String,}";
    pub const FIELD_2: &'static str = "pub field_2: String";
    pub struct ExtractStruct {
        pub field_1: usize,
        pub field_2: String,
    }
}
