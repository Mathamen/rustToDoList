use std::io;
use std::io::{BufRead, Write};




pub struct IOMock<R, W> {
    pub reader: R,
    pub writer: W,
}

impl<R, W> IOMock<R, W>
    where
        R: BufRead,
        W: Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
    pub fn prompt(&mut self, question: &str) -> String {
        write!(&mut self.writer, "{}", question).expect("Unable to write");
        let mut s = String::new();
        self.reader.read_line(&mut s).expect("Unable to read");
        s
    }
}

#[test]
fn test_with_in_memory() {
    let input = b"I'm George";
    let mut output = Vec::new();

    let answer = {
        let mut quizzer = IOMock {
            reader: &input[..],
            writer: &mut output,
        };

        quizzer.prompt("Who goes there?")
    };

    let output = String::from_utf8(output).expect("Not UTF-8");

    assert_eq!("Who goes there?", output);
    assert_eq!("I'm George", answer);
}

pub fn mockmain() {
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();

    let mut quizzer = IOMock {
        reader: input,
        writer: output,
    };

    let answer = quizzer.prompt("Who goes there?");
    println!("was: {}", answer);
}