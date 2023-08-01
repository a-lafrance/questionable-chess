mod err;
mod game;
mod mv;

use std::io::{self, Read};

use crate::{err::MoveError, game::Game, mv::Move};

fn main() -> io::Result<()> {
    println!("Welcome to Chess!");
    println!("Specify moves using standard notation (one character piece + start/end squares)");

    let mut move_buf = [0; 5];
    let mut game = Game::new();

    // it's unfortunate that all these infinite loops seem
    // like the most elegant way to do this
    loop {
        print!("{} to move: ", game.current_player());
        io::stdin().read_exact(&mut move_buf)?;

        // repeatedly try to both parse & execute the move,
        // prompting for another on any kind of error
        loop {
            let err = match Move::try_from(move_buf) {
                Ok(mv) => match game.make_move(mv) {
                    Ok(_) => break,
                    Err(e) => e,
                },

                Err(_) => MoveError::InvalidFormat,
            };

            println!("invalid move: {}", err);
            print!("try again: ");
            io::stdin().read_exact(&mut move_buf)?;
        }

        // check for winner
        if let Some(winner) = game.winner() {
            println!("Game over, {} wins!", winner);
            break;
        }
    }

    Ok(())
}
