use std::{io, vec};

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer)?;
        let command = to_uci_command_in(buffer.trim_end()).unwrap();
        let actions = process_command(command);
        for action in actions {
            match action {
                Action::SendMessage(message) => println!("{message}"),
                Action::Quit => return Ok(()),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    SendMessage(String),
    Quit,
}

#[derive(PartialEq, Debug)]
enum UciCommandIn {
    Uci,
    Quit,
}

fn to_uci_command_in(input: &str) -> Result<UciCommandIn, String> {
    match input {
        "uci" => Ok(UciCommandIn::Uci),
        "quit" => Ok(UciCommandIn::Quit),
        _ => Err("Unsupported command".to_string()),
    }
}

fn process_command(command: UciCommandIn) -> Vec<Action> {
    match command {
        UciCommandIn::Uci => {
            vec![
                Action::SendMessage("id name chessticot".to_string()),
                Action::SendMessage("id author Simisticot".to_string()),
                Action::SendMessage("uciok".to_string()),
            ]
        }
        UciCommandIn::Quit => {
            vec![Action::Quit]
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
    fn can_parse_quit_command() {
        assert!(to_uci_command_in("quit").is_ok_and(|command| command == UciCommandIn::Quit));
    }

    #[test]
    fn responds_to_uci_command() {
        let actions = process_command(UciCommandIn::Uci);
        assert_eq!(
            actions,
            vec![
                Action::SendMessage("id name chessticot".to_string()),
                Action::SendMessage("id author Simisticot".to_string()),
                Action::SendMessage("uciok".to_string()),
            ]
        );
    }

    #[test]
    fn quits_on_quit_command() {
        assert_eq!(process_command(UciCommandIn::Quit), vec![Action::Quit]);
    }
}
