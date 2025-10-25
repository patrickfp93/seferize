pub mod samples_with_ignore;
pub mod simple_samples;
#[allow(unused)]
pub mod extract_into;
#[allow(unused)]
pub mod exposed_methods{

    pub mod simple_methods{

        #[seferize_base::stringify("ORIGINAL_SIMPLE_METHOD")]
        pub mod original{
            fn simple_method(){
                println!("hello!");
            }
        }
        #[seferize_base::stringify("EXPECTED_SIMPLE_METHOD")]
        pub mod expected{
            #[cfg(test)]
            pub fn testable_simple_method(){simple_method()} 

            fn simple_method(){
                println!("hello!");
            }
        }

    }

}
