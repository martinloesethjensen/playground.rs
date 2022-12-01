use quicli::prelude::*;
use structopt::StructOpt;

fn main() -> CliResult {
    let args = Cli::from_args();

    let n = args.num;

    println!("{:?}", primes(n));

    Ok(())
}

fn primes(n: usize) -> Vec<usize> {
    let numbers = (1..=n).collect::<Vec<_>>();

    numbers.into_iter().filter(|&n| is_prime(n)).collect()
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }

    (2..n).all(|a| n % a != 0)
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "n")]
    num: usize,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn returns_primes() {
        let result = primes(11);
        assert_eq!(result, vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn is_valid_prime() {
        assert_eq!(is_prime(11), true);
    }

    #[test]
    fn is_not_valid_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
    }
}
