use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "seed", about = "Bitcoin wallet seed CLI.")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Bitcoin wallet private key.
    #[structopt(about = "Bitcoin wallet private key.")]
    privkey: String,
}

#[derive(Default, Debug)]
struct Checks {
    integer: bool,
    hex: bool,
    base58: bool,
    wif: bool,
}

fn main() {
    // command line flags and arguments
    let opt = Opt::from_args();
    if opt.debug {
        println!("{:?}", opt);
        println!("Private key passed: {:?}", opt.privkey);
    }

    // run checks on the passed privkey
    let mut checks: Checks = Default::default();

    // Test the privkey: is it all number characters
    let allnumbers = opt.privkey.chars().all(char::is_numeric);
    if allnumbers {
      checks.integer = true;
        if opt.debug {
            println!("Private key is all numbers: {:?}", allnumbers);
        }
    }

    // TODO:
    //   many other checks
    if opt.debug {
        println!("Checks: : {:?}", checks);
    }

    // bail out here if our checks find nothing usable.
    if !checks.integer && !checks.hex && !checks.base58 && !checks.wif {
        println!("Could not interpret the private key.");
        return;
    }

    // TODO:
    //   assess
    //   process

    if opt.debug {
        println!("Done.");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
