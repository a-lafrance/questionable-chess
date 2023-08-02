mod board;
mod err;
mod game;
mod mv;

use std::io::{self, Read, Write};

use crate::{game::Game, mv::Move};

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
        io::stdin().read_line(&mut move_buf)?;

        // repeatedly try to both parse & execute the move,
        // prompting for another on any kind of error
        loop {
            let err = match Move::try_from(move_buf.trim()) {
                Ok(mv) => match game.make_move(mv) {
                    Ok(_) => break,
                    Err(e) => e,
                },

                Err(e) => e,
            };

            println!("invalid move: {}", err);
            print!("try again: ");
            io::stdout().flush()?;
            move_buf.clear();
            io::stdin().read_line(&mut move_buf)?;
        }

        // check for winner
        if let Some(winner) = game.winner() {
            println!("Game over, {} wins!", winner);
            break;
        }
    }

    Ok(())
}
