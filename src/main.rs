fn main() {
    let mut rl = rustyline::DefaultEditor::new().expect("failed to start repel");

    println!("crepel v0.1.0");

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                println!("{}", line.trim());
            }

            Err(e) => {
                match e {
                    rustyline::error::ReadlineError::Eof => println!("CTRL-D"),
                    rustyline::error::ReadlineError::Interrupted => println!("CTRL-C"),
                    _ => {}
                }
                break;
            }
        }
    }
}
