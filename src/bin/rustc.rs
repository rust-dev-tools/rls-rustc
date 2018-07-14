// See rustc/rustc.rs in rust repo for explanation of stack adjustments.
#![feature(link_args)]
#[allow(unused_attributes)]
#[cfg_attr(all(windows, target_env = "msvc"), link_args = "/STACK:16777216")]
#[cfg_attr(all(windows, not(target_env = "msvc")), link_args = "-Wl,--stack,16777216")]
extern {}

extern crate rls_rustc;

fn main() {
    rls_rustc::run();
}
