#[macro_use] extern crate rustler;
// #[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

extern crate failure;
extern crate snips_nlu_lib;

use std::fs;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

use rustler::{Env, Error, Term, NifResult, Encoder};
use rustler::resource::ResourceArc;
use snips_nlu_lib::SnipsNluEngine;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom eof;

        // Posix
        atom enoent; // File does not exist
        atom eacces; // Permission denied
        atom epipe;  // Broken pipe
        atom eexist; // File exists
    }
}

struct EngineResource {
    pub engine: SnipsNluEngine
}

rustler_export_nifs! {
    "Elixir.Snips.NLU.Native",
    [
        ("engine_open", 1, engine_open),
        ("add", 2, add)
    ],
    Some(on_load)
}

fn io_error_to_term<'a>(env: Env<'a>, err: &IoError) -> Term<'a> {
    let error = match err.kind() {
        IoErrorKind::NotFound => atoms::enoent().encode(env),
        IoErrorKind::PermissionDenied => atoms::eacces().encode(env),
        IoErrorKind::BrokenPipe => atoms::epipe().encode(env),
        IoErrorKind::AlreadyExists => atoms::eexist().encode(env),
        _ => format!("{}", err).encode(env),
    };

    (atoms::error(), error).encode(env)
}

fn on_load(env: Env, _info: Term) -> bool {
    resource_struct_init!(EngineResource, env);
    true
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let num1: i64 = try!(args[0].decode());
    let num2: i64 = try!(args[1].decode());

    Ok((atoms::ok(), num1 + num2).encode(env))
}

fn engine_open<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let path: String = try!(args[0].decode());

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(ref err) => return Ok(io_error_to_term(env, err))
    };
    let engine = match SnipsNluEngine::from_zip(file) {
        Ok(result) => result,
        Err(error) => {
            println!("Error: {:#?}", error);
            return Err(Error::BadArg) // FIXME error handling
        }
    };

    let resource = ResourceArc::new(EngineResource {
        engine: engine,
    });

    Ok(resource.encode(env))
}
