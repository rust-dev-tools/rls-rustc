#![feature(rustc_private)]
#![feature(box_syntax)]

extern crate env_logger;
extern crate getopts;
extern crate rustc;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate syntax;

use rustc::middle::cstore::CrateStore;
use rustc::session::Session;
use rustc::session::config::{self, ErrorOutputType, Input};
use rustc_driver::driver::CompileController;
use rustc_driver::{run_compiler, CompilerCalls, RustcDefaultCalls, Compilation, enable_save_analysis, get_args};
use syntax::ast;

use std::path::PathBuf;
use std::process;

pub fn run() {
    env_logger::init().unwrap();

    let result = rustc_driver::run(|| run_compiler(&get_args(),
                                                   &mut ShimCalls,
                                                   None,
                                                   None));
    process::exit(result as i32);
}


struct ShimCalls;

impl<'a> CompilerCalls<'a> for ShimCalls {
    fn early_callback(&mut self,
                      a: &getopts::Matches,
                      b: &config::Options,
                      c: &ast::CrateConfig,
                      d: &rustc_errors::registry::Registry,
                      e: ErrorOutputType)
                      -> Compilation {
        RustcDefaultCalls.early_callback(a, b, c, d, e)
    }

    fn late_callback(&mut self,
                     a: &getopts::Matches,
                     b: &Session,
                     c: &CrateStore,
                     d: &Input,
                     e: &Option<PathBuf>,
                     f: &Option<PathBuf>)
                     -> Compilation {
        RustcDefaultCalls.late_callback(a, b, c, d, e, f)
    }

    fn some_input(&mut self,
                  a: Input,
                  b: Option<PathBuf>)
                  -> (Input, Option<PathBuf>) {
        RustcDefaultCalls.some_input(a, b)
    }

    fn no_input(&mut self,
                a: &getopts::Matches,
                b: &config::Options,
                c: &ast::CrateConfig,
                d: &Option<PathBuf>,
                e: &Option<PathBuf>,
                f: &rustc_errors::registry::Registry)
                -> Option<(Input, Option<PathBuf>)> {
        RustcDefaultCalls.no_input(a, b, c, d, e, f)
    }

    fn build_controller(&mut self, a: &Session, b: &getopts::Matches) -> CompileController<'a> {
        let mut result = RustcDefaultCalls.build_controller(a, b);

        result.continue_parse_after_error = true;
        enable_save_analysis(&mut result);

        result
    }
}
