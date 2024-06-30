use mock::IOMock;



// Estes testes estão conseguindo 100% de cobertura, segundo o tarpaulin
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