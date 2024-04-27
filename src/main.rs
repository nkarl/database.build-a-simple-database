use std::io;
use std::io::Write;

struct InputBuffer {
    buffer: *mut char,
    buffer_length: u32,
    input_length: u64,
}

fn main() {
    loop {
        let cmd = prompt();
    }
}

fn prompt() -> io::Result<String> {
    print_prompt().ok();
    let s = format!("echo {}", get_prompt().ok().unwrap());
    print_string(&s).ok();
    Ok(s)
}

fn get_prompt() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn print_prompt() -> io::Result<()> {
    print_string(&String::from("db > "))
}

fn print_string(s: &String) -> io::Result<()> {
    let out = io::stdout().lock();
    let mut buf = io::BufWriter::new(out);
    buf.write(s.as_bytes())?;
    buf.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_asserts_sizeof_byte_char_is_same_as_u8() {
        let s = String::from("hello");
        assert_eq!(5 /* bytes */, s.len() * std::mem::size_of::<u8>());
    }

    #[test]
    fn it_asserts_sizeof_char_is_u32() {
        let s = vec!['h', 'e', 'l', 'l', 'o'];
        assert_eq!(5 /* bytes */, s.len() * std::mem::size_of::<u32>());
    }
}
