use std::io;

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer)?;
        let command = to_uci_command(buffer.trim_end()).unwrap();
        println!("{command:?}");
    }
}

#[derive(PartialEq, Debug)]
enum UciCommand {
    Uci,
}

fn to_uci_command(input: &str) -> Result<UciCommand, String> {
    match input {
        "uci" => Ok(UciCommand::Uci),
        _ => Err("Unsupported command".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_existent_uci_command_returns_error() {
        assert!(to_uci_command("skibidi").is_err());
    }

    #[test]
    fn can_parse_uci_command() {
        assert!(to_uci_command("uci").is_ok_and(|command| command == UciCommand::Uci))
    }
}
