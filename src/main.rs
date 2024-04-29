use std::io;
use std::io::Write;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
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
    let mut cmd = String::new();
    'main_program: loop {
        cmd = prompt(_debug).unwrap();
        cmd = strip_newline_cr(&cmd); // TODO: this should be part of the Parser
        match _debug {
            false => match &cmd[..] {
                /*
                 * TODO: implment command lookup table
                 */
                ".quit;" | ".exit;" => break 'main_program,
                _ => continue,
            },
            true => break,
        }
    }
    Ok(())
}

fn strip_newline_cr(s: &str) -> String {
    let (carriage_return, newline) = ("\r\n", "\n");
    s.strip_suffix(carriage_return)
        .or(s.strip_suffix(newline))
        .unwrap_or(s)
        .to_string()
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
mod tests {
    use crate::strip_newline_cr;

    #[test]
    fn it_strips_newline_cr_from_strings() {
        assert_eq!(strip_newline_cr(".quit;\r\n\r\n"), ".quit;\r\n");
        assert_eq!(strip_newline_cr(".quit;\r\n"), ".quit;");
        assert_eq!(strip_newline_cr(".quit;\n"), ".quit;");
        assert_eq!(strip_newline_cr(".quit;"), ".quit;");
    }
}
