use std::io;
use entradasaida::*;
use mock::IOMock;

fn main() {
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();
    let mut io_control = IOMock {
        reader: input,
        writer: output,
    };

    loop_principal(&mut io_control);
}
