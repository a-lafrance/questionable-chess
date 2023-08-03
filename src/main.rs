mod board;
mod err;
mod game;
mod mv;

use std::io::{self, Write};

use crate::{
    game::{Game, TurnOutcome},
    mv::Move,
};

fn main() -> io::Result<()> {
    println!("Welcome to Chess!");
    println!("Specify moves using standard notation (one character piece + start/end squares)\n");
    // TODO: message about which color is which player

    let mut move_buf = String::new();
    let mut game = Game::default();

    // it's unfortunate that all these infinite loops seem
    // like the most elegant way to do this
    loop {
        println!("{}\n", game);
        print!("{} to move: ", game.current_player());
        io::stdout().flush()?;
        move_buf.clear();
        io::stdin().read_line(&mut move_buf)?;

        // repeatedly try to both parse & execute the move,
        // prompting for another on any kind of error
        let outcome = loop {
            let err = match Move::try_from(move_buf.trim()) {
                Ok(mv) => match game.make_move(mv) {
                    Ok(outcome) => break outcome,
                    Err(e) => e,
                },

                Err(e) => e,
            };

            println!("invalid move: {}", err);
            print!("try again: ");
            io::stdout().flush()?;
            move_buf.clear();
            io::stdin().read_line(&mut move_buf)?;
        };

        match outcome {
            TurnOutcome::Taken(piece) => println!(
                "{} takes {}",
                piece.color().opposite(),
                piece,
            ),

            TurnOutcome::Win(winner) => {
                println!("Game over, {} wins!", winner);
                break;
            },

            _ => {},
        }

        println!();
    }

    Ok(())
}
