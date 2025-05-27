mod cli;

use std::io::{self, Write};

use clap::Parser;
use cli::Cli;
use quizzer::{Question, Quiz, QuizError};

fn main() -> anyhow::Result<(), QuizError> {
    let args = Cli::parse();

    let mut q = Quiz::new("quiz.json".to_owned(), "quiz".to_owned())?;

    match args.mode {
        cli::Mode::Add => Ok(add_questions(&mut q)?),
        cli::Mode::Quiz => {
            q.load_quiz()?;
            Ok(start_quiz(&q)?)
        }
    }
}

fn start_quiz(q: &Quiz) -> anyhow::Result<(), QuizError> {
    let mut correct = 0;
    let mut total = 0;
    let stdin = io::stdin();
    let mut answer = String::new();

    for question in q.iter() {
        total += 1;
        println!("Question: {}", question.question());
        println!("Answer #1: {}", question.correct_answer());

        for (i, incorrect) in question.other_answers().iter().enumerate() {
            println!("Answer #{}: {}", i + 2, incorrect);
        }
        print!("Enter number of your answer: ");
        match io::stdout().flush() {
            Err(err) => return Err(quizzer::QuizError::Io(err)),
            _ => (),
        };
        match stdin.read_line(&mut answer) {
            Err(err) => return Err(quizzer::QuizError::Io(err)),
            _ => (),
        };

        if answer.trim_end() == "1" {
            correct += 1;
        }
    }

    println!("You have correctly guessed {} out of {}", correct, total);
    Ok(())
}

fn add_questions(q: &mut Quiz) -> anyhow::Result<(), QuizError> {
    println!("You are in mode for entering questions, to exit press enter on question prompt");
    let stdin = io::stdin();
    loop {
        let mut question = String::new();
        let mut correct = String::new();
        let mut incorrect = String::new();
        print!("Question: ");
        match io::stdout().flush() {
            Err(err) => return Err(quizzer::QuizError::Io(err)),
            _ => (),
        };
        match stdin.read_line(&mut question) {
            Err(err) => return Err(quizzer::QuizError::Io(err)),
            _ => (),
        };
        if question == "" {
            println!();
            break;
        }

        print!("Correct answer: ");
        match io::stdout().flush() {
            Err(err) => return Err(quizzer::QuizError::Io(err)),
            _ => (),
        };
        match stdin.read_line(&mut correct) {
            Err(err) => return Err(quizzer::QuizError::Io(err)),
            _ => (),
        };

        print!("Incorrect answers (separated by space): ");
        match io::stdout().flush() {
            Err(err) => return Err(quizzer::QuizError::Io(err)),
            _ => (),
        };
        match stdin.read_line(&mut incorrect) {
            Err(err) => return Err(quizzer::QuizError::Io(err)),
            _ => (),
        };
        let ques = Question::new(
            question.trim_end().to_string(),
            correct.trim_end().to_string(),
            incorrect.split_whitespace().map(str::to_string).collect(),
        );
        q.add_question(ques)?;
    }
    Ok(())
}
