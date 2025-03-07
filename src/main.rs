use std::{io, vec};

use libchessticot::{BetterEvaluationPlayer, Player, Position};

fn main() -> io::Result<()> {
    let mut current_position: Option<Position> = None;
    let engine = BetterEvaluationPlayer {};
    loop {
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer)?;
        let command = to_uci_command_in(buffer.trim_end()).unwrap();
        let actions = process_command(command);
        for action in actions {
            match action {
                Action::SendMessage(message) => println!("{message}"),
                Action::Quit => return Ok(()),
                Action::SetPosition(position) => current_position = Some(position),
                Action::SendBestMove => {
                    let best_move = engine.offer_move(
                        &current_position
                            .clone()
                            .expect("should initialize position before asking for move"),
                    );
                    println!("bestmove {best_move:?}");
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    SendMessage(String),
    SetPosition(Position),
    Quit,
    SendBestMove,
}

#[derive(PartialEq, Debug)]
enum UciCommandIn {
    Uci,
    Position(Position),
    Quit,
    Go,
}

fn to_uci_command_in(input: &str) -> Result<UciCommandIn, String> {
    match input.split(" ").next().unwrap() {
        "uci" => Ok(UciCommandIn::Uci),
        "quit" => Ok(UciCommandIn::Quit),
        "position" => Ok(UciCommandIn::Position(position_from_command(input))),
        "go" => Ok(UciCommandIn::Go),
        _ => Err("Unsupported command".to_string()),
    }
}

fn position_from_command(command: &str) -> Position {
    let mut params = command.split(" ");
    let command = params.next().unwrap();
    assert!(command == "position");
    let mode = params.next().unwrap();
    let starting_position: Position = match mode {
        "startpos" => Position::initial(),
        "fen" => {
            let fen: String = params
                .take(6)
                .fold(String::new(), |a, b| a + " " + b)
                .trim_start()
                .to_string();
            Position::from_fen(&fen)
        }
        _ => panic!("unsupported position mode"),
    };
    starting_position
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
        UciCommandIn::Position(position) => vec![Action::SetPosition(position)],
        UciCommandIn::Go => {
            vec![Action::SendBestMove]
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
    fn can_parse_position_command_initial_position() {
        assert!(
            to_uci_command_in("position startpos")
                .is_ok_and(|command| command == UciCommandIn::Position(Position::initial()))
        )
    }

    #[test]
    fn can_parse_position_command_with_fen() {
        assert!(
            to_uci_command_in(
                "position fen rnbqkbnr/pp2pppp/2p5/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 1"
            )
            .is_ok_and(|command| command
                == UciCommandIn::Position(Position::from_fen(
                    "rnbqkbnr/pp2pppp/2p5/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 1"
                )))
        )
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
