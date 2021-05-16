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
    intkey: bool,
    hexkey: bool,
    base58key: bool,
    wifkey: bool,
}

fn main() {
    // command line flags and arguments
    let opt = Opt::from_args();
    if opt.debug {
        println!("{:?}", opt);
        println!("Private key passed: {:?}", opt.privkey);
    }

    // run checks on the passed privkey
    let checks= pivkeychecks(opt.privkey);

    // TODO:
    //   many other checks
    if opt.debug {
        println!("Private key assessment checks: {:?}", checks);
    }

    // bail out here if our checks find nothing usable.
    if !checks.intkey && !checks.hexkey && !checks.base58key && !checks.wifkey {
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

fn pivkeychecks(pk: String) -> Checks {
    let mut checks: Checks = Default::default();
    let allnumbers = pk.chars().all(char::is_numeric);
    if allnumbers {
      checks.intkey = true;
    }
    checks
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
