#![feature(rustc_private)]
#![feature(box_syntax)]

extern crate env_logger;
extern crate getopts;
extern crate rustc;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_resolve;
extern crate rustc_save_analysis;
extern crate syntax;

use rustc_driver::{run, run_compiler, CompilerCalls, RustcDefaultCalls, Compilation};
use rustc_driver::driver::CompileController;
use rustc_save_analysis as save;
use rustc_save_analysis::DumpHandler;
use rustc::session::{early_error, Session};
use rustc::session::config::{self, ErrorOutputType, Input};
use rustc::util::common::time;
use syntax::ast;

use std::path::PathBuf;
use std::{env, process};

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
                     c: &Input,
                     d: &Option<PathBuf>,
                     e: &Option<PathBuf>)
                     -> Compilation {
        RustcDefaultCalls.late_callback(a, b, c, d, e)
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

        // TODO use enable_save_analysis
        result.keep_ast = true;

        result.after_analysis.callback = box |state| {
            time(state.session.time_passes(), "save analysis", || {
                save::process_crate(state.tcx.unwrap(),
                                    state.expanded_crate.unwrap(),
                                    state.analysis.unwrap(),
                                    state.crate_name.unwrap(),
                                    None,
                                    DumpHandler::new(state.out_dir,
                                                     state.crate_name.unwrap()))
            });
        };
        result.after_analysis.run_callback_on_error = true;
        result.make_glob_map = ::rustc_resolve::MakeGlobMap::Yes;

        result
    }
}

// TODO use exported version
fn get_args() -> Vec<String> {
    env::args_os().enumerate()
        .map(|(i, arg)| arg.into_string().unwrap_or_else(|arg| {
             early_error(ErrorOutputType::default(),
                         &format!("Argument {} is not valid Unicode: {:?}", i, arg))
         }))
        .collect()
}

fn main() {
    env_logger::init().unwrap();

    let result = run(|| run_compiler(&get_args(),
                                     &mut ShimCalls,
                                     None,
                                     None));
    process::exit(result as i32);
}
