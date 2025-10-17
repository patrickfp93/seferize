#[allow(unused)]
pub mod expected {
    #[seferize::stringify("EXPECTED_ENUMERATE_MOD_SAMPLE")]
    pub mod my_enumerate {
        pub const MY_ENUMERATE: &'static str = "pub enum MyEnumerate{Var1, Var2}";
        pub enum MyEnumerate{
            Var1, Var2
        }
    }
}
#[allow(unused)]
pub mod original {
    #[seferize::stringify("ORIGINAL_ENUMERATE_MOD_SAMPLE")]
    pub mod my_enumerate {
        pub enum MyEnumerate{
            Var1, Var2
        }
    }
}

#[allow(unused)]
pub use {expected::*, original::*};
