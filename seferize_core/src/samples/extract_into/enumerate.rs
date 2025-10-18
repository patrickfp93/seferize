#[allow(unused)]
pub mod original {
    #[seferize::stringify("ORIGINAL_EXTRACT_ENUMERATE_MOD_SAMPLE")]
    pub mod my_enumerate {
        #[cfg(false)]
        pub enum MyEnumerate {
            #[stringify(EXTRACT_ENUMARATE_VAR)]Var1,
            Var2,
        }
    }
}

#[allow(unused)]
pub mod expected {
    #[seferize::stringify("EXPECTED_EXTRACT_ENUMERATE_MOD_SAMPLE")]
    pub mod my_enumerate {
        pub const EXTRACT_ENUMERATE_MOD_SAMPLE_WITH_VAR: &'static str = "pub enum MyEnumerate{Var1, Var2,}";
        pub const EXTRACT_ENUMARATE_VAR: &'static str ="Var1";
        pub enum MyEnumerate {
            Var1,
            Var2,
        }
    }
}
#[allow(unused)]
pub use {expected::*, original::*};
