use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

//------------------------
#[test]
fn usage() -> TestResult{
    for flag in &["-h", "--help"]{
        Command::cargo_bin(PRG)?
            .args(flag)
            .assert()
            .stdout(predicates::str::contains("USAGE"));
    }
    Ok(())
}

#[test]
fn gen_bad_file() -> String {
    loop{
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err(){
            return filename;
        }
    }
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected)?);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult{
   let expect = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expect);
    Ok(())
}