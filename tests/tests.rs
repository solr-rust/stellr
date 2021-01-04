#[macro_use]
extern crate lazy_static;

mod it;

#[cfg(not(feature = "blocking"))]
mod asynchronous;

#[cfg(feature = "blocking")]
mod blocking;
