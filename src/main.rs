use std::io;

fn main() -> io::Result<()> {
    let mut out = io::stdout();
    loop {
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer)?;
        let command = to_uci_command_in(buffer.trim_end()).unwrap();
        execute_command(command, &mut out);
    }
}

#[derive(PartialEq, Debug)]
enum UciCommandIn {
    Uci,
}

fn to_uci_command_in(input: &str) -> Result<UciCommandIn, String> {
    match input {
        "uci" => Ok(UciCommandIn::Uci),
        _ => Err("Unsupported command".to_string()),
    }
}

fn execute_command(command: UciCommandIn, out: &mut impl io::Write) {
    match command {
        UciCommandIn::Uci => {
            out.write_all("id name chessticot\n".as_bytes()).unwrap();
            out.write_all("id author Simisticot\n".as_bytes()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_existent_uci_command_returns_error() {
        assert!(to_uci_command_in("skibidi").is_err());
    }

    #[test]
    fn can_parse_uci_command() {
        assert!(to_uci_command_in("uci").is_ok_and(|command| command == UciCommandIn::Uci));
    }

    #[test]
    fn responds_to_uci_command() {
        let mut out: Vec<u8> = Vec::new();

        execute_command(UciCommandIn::Uci, &mut out);

        assert_eq!(&out, b"id name chessticot\nid author Simisticot\n");
    }
}
