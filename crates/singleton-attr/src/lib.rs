#![allow(internal_features)]
#![feature(negative_impls, negative_bounds)]
mod manual;
pub mod traits;
#[macro_use]
extern crate singleton_attr_proc_macro;
pub mod derive {
    pub use singleton_attr_proc_macro::{SafeSingleton, Singleton};
}
