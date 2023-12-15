fn prompt_string(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}: ", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    s.trim().to_string()
}

fn prompt_bool(prompt: &str) -> bool {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}: ", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    s.trim().to_lowercase() == "y"
}