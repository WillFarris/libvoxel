
pub mod engine;
pub mod physics;

#[cfg(target_os = "android")]
#[macro_use] extern crate log;
#[cfg(target_os = "android")]
extern crate android_log;
#[cfg(target_os = "android")]
mod java_interface;