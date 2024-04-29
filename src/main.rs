use std::io;
use std::io::Write;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    //name: String,
    //count: u8,
    #[clap(long, short, action)]
    debug: bool,
}

struct InputBuffer {
    buffer: *mut char,
    buffer_length: u32,
    input_length: u64,
}

fn main() {
    let args = Args::parse();
    let _ = main_body(args);
}

fn main_body(args: Args) -> io::Result<()> {
    let _debug: bool = args.debug;
    let _ = match _debug {
        true => prompt(_debug),
        false => loop {
            // program loop
            let s = prompt(_debug).unwrap();
        },
    };
    Ok(())
}

fn prompt(_debug: bool) -> io::Result<String> {
    let _ = write_string("db > ");
    let cmd = get_cmd(_debug);
    let _ = write_string(&cmd);
    Ok(cmd)
}

fn get_cmd(_debug: bool) -> String {
    match _debug {
        true => String::from("debug command\n"),
        false => prompt_user().unwrap(),
    }
}

fn prompt_user() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s)
}

fn write_string(s: &str) -> io::Result<()> {
    let out = io::stdout().lock();
    let mut buf = io::BufWriter::new(out);
    buf.write(s.as_bytes())?;
    buf.flush()
}

#[cfg(test)]
mod tests {}
