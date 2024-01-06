use argh;
use argh::FromArgs;
use fermats_kitchen::BigInt;
use rug::integer::SmallInteger;

/// Utilities related to prime numbers
#[derive(FromArgs, PartialEq, Debug)]
struct Args {
    #[argh(subcommand)]
    action: SubCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    PTest(PTestCommand),
}

/// Run a primality test on a prime number. The result is reported as either composite,
/// probable prime or known prime.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "ptest")]
struct PTestCommand {
    #[argh(positional, description = "the number to test")]
    number: BigInt,
}

fn main() {
    let args: Args = argh::from_env();
    match args.action {
        SubCommands::PTest(cmd) => {
            if fermats_kitchen::primality::fermats_test(&cmd.number, SmallInteger::from(2).into()) {
                println!("Probable prime")
            } else {
                println!("Composite")
            }
        }
    }
}
