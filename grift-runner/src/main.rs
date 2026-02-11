use grift::{Lisp, Evaluator, Value, StdIoProvider, ArenaIndex};
use grift::repl::{format_error, value_to_string};
use std::env;
use std::fs;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <scheme-file.scm>", args[0]);
        std::process::exit(1);
    }

    let file_path = args[1].clone();

    // Run with increased stack size for deep recursion in the evaluator
    let builder = thread::Builder::new()
        .name("grift-runner".into())
        .stack_size(64 * 1024 * 1024);

    let handle = builder.spawn(move || run_file(&file_path)).unwrap();

    match handle.join() {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("Thread panicked: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run_file(path: &str) -> i32 {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", path, e);
            return 1;
        }
    };

    // Use Box for the large arena to avoid stack overflow (as recommended)
    let lisp: Box<Lisp<500_000>> = Box::new(Lisp::new());
    let mut io = StdIoProvider::new();
    let mut eval = match Evaluator::new(&*lisp) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to create evaluator: {}", format_error(&*lisp, &e));
            return 1;
        }
    };

    // Set up output callback for display/newline and I/O provider for port operations
    eval.set_output_callback(Some(display_callback));
    eval.set_io_provider(&mut io);

    // Wrap in (begin ...) to evaluate all top-level forms
    let wrapped = format!("(begin {})", content);
    match eval.eval_str(&wrapped) {
        Ok(result) => {
            if !matches!(lisp.get(result), Ok(Value::Void)) {
                println!("{}", value_to_string(&*lisp, result));
            }
            0
        }
        Err(e) => {
            eprintln!("{}", format_error(&*lisp, &e));
            1
        }
    }
}

/// Output callback using display semantics (no quotes around strings).
fn display_callback<const N: usize>(lisp: &Lisp<N>, val: ArenaIndex) {
    use std::io::Write;
    let mut stdout = std::io::stdout();

    if val.is_nil() {
        let _ = stdout.write_all(b"\n");
    } else {
        let mut buf = String::new();
        display_value(lisp, val, &mut buf);
        let _ = stdout.write_all(buf.as_bytes());
    }
    let _ = stdout.flush();
}

/// Format a value using display semantics (strings without quotes, chars without #\).
fn display_value<const N: usize>(lisp: &Lisp<N>, idx: ArenaIndex, buf: &mut String) {
    match lisp.get(idx) {
        Ok(Value::String { .. }) => {
            let len = lisp.string_len(idx).unwrap_or(0);
            for i in 0..len {
                if let Ok(c) = lisp.string_char_at(idx, i) {
                    buf.push(c);
                }
            }
        }
        Ok(Value::Char(c)) => {
            buf.push(c);
        }
        _ => {
            grift::repl::format_value(lisp, idx, buf);
        }
    }
}
