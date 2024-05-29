#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub(crate) mod driver;
pub(crate) mod error;
pub(crate) mod input;
pub(crate) mod processor;
pub(crate) mod sampler;
pub(crate) mod singletons;
pub(crate) mod task_trait;
pub(crate) mod ui;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod test {
    #[test]
    fn hello_world() {
        std::println!("hello_world");
    }
}
