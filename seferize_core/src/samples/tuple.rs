pub mod expected {
    #[seferize::stringify("EXPECTED_TUPLE_MOD_SAMPLE")]
    pub mod my_tuple {
        pub const MY_TUPLE: &'static str =
            "pub struct MyTuple(usize,String);";
        pub struct MyTuple(usize,String);
    }
}
pub mod original {
    #[seferize::stringify("ORIGINAL_TUPLE_MOD_SAMPLE")]
    pub mod my_tuple {
        pub struct MyTuple(usize,String);
    }
}

pub use original::*;
pub use expected::*;
