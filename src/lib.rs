mod java_interface;
pub mod engine;
pub mod graphics;
pub mod physics;

#[cfg(target_os = "android")]
#[macro_use] extern crate log;
#[cfg(target_os = "android")]
extern crate android_log;