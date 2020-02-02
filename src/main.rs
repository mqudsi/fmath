use std::io::{BufRead, BufReader};
use std::str::FromStr;

trait FoldOperation: Sized {
    type StateType: Default;
    type NumType: FromStr;

    fn fold(&self, acc: Self::StateType, n: Self::NumType) -> Self::StateType;
    fn finish(&self, acc: Self::StateType) -> Self::NumType;
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

enum DispatchedFoldOperation {
    Sum(<Sum as FoldOperation>::StateType),
    Avg(<Avg as FoldOperation>::StateType),
}

struct ParseError;
impl DispatchedFoldOperation {
    fn fold(&mut self, input: &str) -> Result<(), ParseError> {
        let num = input.parse().map_err(|_| ParseError {})?;
        match self {
            DispatchedFoldOperation::Sum(ref mut state) => *state = Sum{}.fold(*state, num),
            DispatchedFoldOperation::Avg(ref mut state) => *state = Avg{}.fold(*state, num),
        }

        Ok(())
    }

    fn finish(&self) -> String {
        let result = match self {
            DispatchedFoldOperation::Sum(state) => Sum{}.finish(*state),
            DispatchedFoldOperation::Avg(state) => Avg{}.finish(*state),
        };
        format!("{}", result)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("USAGE: fmath [sum|avg]");
        return;
    }

    let mut op = match args[1].as_ref() {
        "sum" => DispatchedFoldOperation::Sum(Default::default()),
        "avg" => DispatchedFoldOperation::Avg(Default::default()),
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
