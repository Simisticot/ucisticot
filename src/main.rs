use std::{
    io::{self, BufRead, Write},
    vec,
};

use libchessticot::{BetterEvaluationPlayer, ChessMove, Player, Position};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let input = stdin.lock();
    let output = io::stdout();
    server(input, output)
}

fn server<R, W>(mut reader: R, mut writer: W) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut current_position: Option<Position> = None;
    let mut best_move_for_current_search: Option<ChessMove> = None;
    let engine = BetterEvaluationPlayer {};
    loop {
        let mut buffer = String::new();
        let _ = reader.read_line(&mut buffer)?;
        let command = to_uci_command_in(buffer.trim_end()).unwrap();
        let actions = process_command(command);
        for action in actions {
            match action {
                Action::SendMessage(message) => writeln!(&mut writer, "{message}").unwrap(),
                Action::Quit => return Ok(()),
                Action::SetPosition(position) => current_position = Some(position),
                Action::FindBestMove => {
                    best_move_for_current_search = Some(
                        engine.offer_move(
                            &current_position
                                .clone()
                                .expect("should initialize position before asking for move"),
                        ),
                    )
                }
                Action::SendBestMove => writeln!(
                    &mut writer,
                    "bestmove {}",
                    best_move_for_current_search
                        .clone()
                        .expect("should have just computed best move")
                        .to_uci_long(
                            &current_position
                                .clone()
                                .expect("position should be initialized")
                        )
                )
                .unwrap(),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    SendMessage(String),
    SetPosition(Position),
    Quit,
    FindBestMove,
    SendBestMove,
}

#[derive(PartialEq, Debug)]
enum UciCommandIn {
    Uci,
    Position(Position),
    Quit,
    Go,
    Stop,
}

fn to_uci_command_in(input: &str) -> Result<UciCommandIn, String> {
    match input.split(" ").next().unwrap() {
        "uci" => Ok(UciCommandIn::Uci),
        "quit" => Ok(UciCommandIn::Quit),
        "position" => Ok(UciCommandIn::Position(position_from_command(input))),
        "go" => Ok(UciCommandIn::Go),
        "stop" => Ok(UciCommandIn::Stop),
        _ => Err("Unsupported command".to_string()),
    }
}

fn position_from_command(command: &str) -> Position {
    let mut params = command.split(" ");
    let command = params.next().unwrap();
    assert!(command == "position");
    let mode = params.next().unwrap();
    let mut position: Position = match mode {
        "startpos" => Position::initial(),
        "fen" => {
            let fen: String = params
                .clone()
                .take(6)
                .fold(String::new(), |a, b| a + " " + b)
                .trim_start()
                .to_string();
            Position::from_fen(&fen)
        }
        _ => panic!("unsupported position mode"),
    };
    while let Some(param) = params.next() {
        if param == "moves" {
            for param in params.by_ref() {
                let chess_move = ChessMove::from_uci_long(param, &position);
                position = position.after_move(&chess_move);
            }
        }
    }
    position
}

fn process_command(command: UciCommandIn) -> Vec<Action> {
    match command {
        UciCommandIn::Uci => vec![
            Action::SendMessage("id name chessticot".to_string()),
            Action::SendMessage("id author Simisticot".to_string()),
            Action::SendMessage("uciok".to_string()),
        ],

        UciCommandIn::Quit => vec![Action::Quit],
        UciCommandIn::Position(position) => vec![Action::SetPosition(position)],
        UciCommandIn::Go => vec![Action::FindBestMove],

        UciCommandIn::Stop => vec![Action::SendBestMove],
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
    fn can_parse_initial_position_with_additional_moves() {
        assert_eq!(
            to_uci_command_in("position startpos moves e2e4 e7e5").unwrap(),
            UciCommandIn::Position(Position::from_fen(
                "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 1"
            ))
        )
    }

    #[test]
    fn can_parse_fen_position_with_additional_moves() {
        assert_eq!(
            to_uci_command_in(
                "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4 e7e5"
            )
            .unwrap(),
            UciCommandIn::Position(Position::from_fen(
                "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 1"
            ))
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

    #[test]
    fn get_first_move() {
        let input = b"uci\nposition startpos\ngo\nstop\nquit\n";
        let mut output = Vec::new();
        server(&input[..], &mut output).expect("should not fail");
        assert_eq!(
            "id name chessticot\nid author Simisticot\nuciok\nbestmove e2e4\n",
            String::from_utf8(output).expect("please :)")
        )
    }
}
