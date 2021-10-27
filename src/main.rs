#[cfg(target_os = "emscripten")]
use std::{ffi::CString, mem, os::raw::c_char};

#[cfg(not(target_os = "emscripten"))]
use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
};
#[cfg(not(target_os = "emscripten"))]
use structopt::StructOpt;
#[cfg(not(target_os = "emscripten"))]
#[derive(StructOpt, Debug)]
#[structopt(name = "aussie++")]
struct Opt {
    /// Path to input file
    #[structopt(name = "File", parse(from_os_str))]
    filepath: Option<PathBuf>,
}
#[cfg(not(target_os = "emscripten"))]
fn main() {
    let opt = Opt::from_args();
    let code: String;
    if let Some(filepath) = opt.filepath {
        code = fs::read_to_string(filepath).expect("failed to read file");
        aussie_plus_plus::interpret(code.as_str()).unwrap();
        println!("CHEERS C***!");
        return;
    }

    let stdin = io::stdin();
    let mut i = aussie_plus_plus::runtime::Interpreter::new();
    let mut p = aussie_plus_plus::parser::parser::Parser::new(vec![]);
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match aussie_plus_plus::interpret_repl(line.as_str(), &mut i, &mut p) {
            Ok(stmts) => stmts,
            Err(_) => {
                // eprintln!("Failed to run: {}", e);
                return;
            }
        };
    }
    println!("CHEERS C***!");
}

#[cfg(target_os = "emscripten")]
fn main() {}

#[cfg(target_os = "emscripten")]
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn interpret(src: *mut c_char, upside_down: bool) -> usize {
    let code = CString::from_raw(src).to_str().unwrap().to_string();
    if !upside_down {
        let _ = aussie_plus_plus::interpret(&code);
    } else {
        let _ = aussie_plus_plus::interpret_upside_down(&code);
    }
    0
}

#[cfg(target_os = "emscripten")]
/// # Safety
/// `src` must point to memory previously allocated. The function will consume
/// the pointer and return a new pointer to the flipped text.
#[no_mangle]
pub unsafe extern "C" fn flip_text(
    src: *mut c_char,
    len_ptr: *mut usize,
    upside_down: bool,
) -> *const c_char {
    use std::ptr::null;

    let code = CString::from_raw(src).to_str().unwrap().to_string();

    // Allocate new string because flipping orientation may change byte length
    let output: String = if upside_down {
        code.chars()
            .rev()
            .map(aussie_plus_plus::upside_down::upside_down)
            .collect()
    } else {
        code.chars()
            .rev()
            .map(aussie_plus_plus::upside_down::rightside_up)
            .collect()
    };

    let output = match CString::new(output) {
        Ok(cstr) => cstr,
        Err(e) => {
            eprintln!("Failed to make cstring: {:?}", e);
            return null();
        }
    };

    *len_ptr = output.to_bytes().len();

    let ptr = output.as_ptr();
    mem::forget(output);

    ptr
}

#[cfg(target_os = "emscripten")]
#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(len);

    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

#[cfg(target_os = "emscripten")]
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    let data = Vec::from_raw_parts(ptr, len, len);

    mem::drop(data)
}
