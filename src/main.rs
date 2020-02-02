#![allow(bare_trait_objects)]

use std::io::{BufRead, BufReader};
use std::str::FromStr;

trait FoldOperation: Sized {
    type StateType: Default;
    type NumType: FromStr;

    fn fold(&self, acc: Self::StateType, n: Self::NumType) -> Self::StateType;
    fn finish(&self, acc: Self::StateType) -> Self::NumType;
}

struct ParseError;
trait GenericFoldOperation {
    fn fold(&mut self, input: &str) -> Result<(), ParseError>;
    fn finish(&self) -> String;
}

#[derive(Default)]
struct Sum;
impl FoldOperation for Sum {
    type StateType = i64;
    type NumType = i64;

    fn fold(&self, acc: Self::StateType, n: Self::NumType) -> Self::StateType {
        acc + n
    }

    fn finish(&self, acc: Self::StateType) -> Self::NumType {
        acc
    }
}

#[derive(Default)]
struct Avg;
impl FoldOperation for Avg {
    type StateType = (u32, i64);
    type NumType = i64;

    fn fold(&self, mut acc: Self::StateType, n: Self::NumType) -> Self::StateType {
        acc.0 += 1;
        acc.1 += n;

        acc
    }

    fn finish(&self, acc: Self::StateType) -> Self::NumType {
        acc.1 / acc.0 as i64
    }
}

#[derive(Default)]
struct Sum2 {
    op: Sum,
    state: i64,
}

impl GenericFoldOperation for Sum2 {
    fn fold(&mut self, input: &str) -> Result<(), ParseError> {
        let num = input.parse().map_err(|_| ParseError {})?;
        self.state = self.op.fold(self.state, num);
        Ok(())
    }

    fn finish(&self) -> String {
        let result = self.op.finish(self.state);
        format!("{}", result)
    }
}

#[derive(Default)]
struct Avg2 {
    op: Avg,
    state: (u32, i64),
}

impl GenericFoldOperation for Avg2 {
    fn fold(&mut self, input: &str) -> Result<(), ParseError> {
        let num = input.parse().map_err(|_| ParseError {})?;
        self.state = self.op.fold(self.state, num);
        Ok(())
    }

    fn finish(&self) -> String {
        let result = self.op.finish(self.state);
        format!("{}", result)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("USAGE: fmath [sum|avg]");
        return;
    }

    let mut sum = Sum2::default();
    let mut avg = Avg2::default();
    let op = match args[1].as_ref() {
        "sum" => &mut sum as &mut GenericFoldOperation,
        "avg" => &mut avg as &mut GenericFoldOperation,
        op @ _ => {
            eprintln!("{} is not a valid operation!", op);
            std::process::exit(1);
        }
    };

    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut reader = BufReader::new(stdin);

    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(2);
            }
            Ok(0) => break,
            _ => {}
        };

        match op.fold(line.trim()) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Cannot convert to a number: {}", line);
                continue;
            }
        };
    }

    println!("{}", op.finish());
}
