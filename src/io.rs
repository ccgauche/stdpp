use std::io::*;

pub fn read<T,E>(message: E) -> Option<T> where T: std::str::FromStr, E: std::fmt::Display {
    print!("{}",message);
    std::io::stdout().flush().ok()?;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok()?;
    if buffer.trim().is_empty() {
        None
    } else {
        buffer.parse().ok()
    }
}