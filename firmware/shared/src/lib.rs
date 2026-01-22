#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub(crate) mod driver;
pub(crate) mod error;
pub(crate) mod input;
pub(crate) mod processor;
pub(crate) mod samplers;
pub(crate) mod singletons;
pub(crate) mod tasks;
pub(crate) mod ui;

pub use samplers::max31865_sampler::Max31865Sampler;
pub use samplers::mock_sampler::MockSampler;
pub use samplers::sampler_trait::SamplerTrait;

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    #[test]
    fn hello_world() {
        std::println!("hello_world");
    }
}
