extern crate hex;

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
    let checks = pivkeychecks(opt.privkey);

    // TODO:
    //   many other checks
    if opt.debug {
        println!("Private key assessment: {:?}", checks);
    }

    // bail out here if our checks find nothing usable.
    if !checks.intkey && !checks.hexkey && !checks.base58key && !checks.wifkey {
        println!("Could not interpret the private key.");
        return;
    }

    // TODO:
    //   process

    if opt.debug {
        println!("Done.");
    }
}

fn pivkeychecks(pk: String) -> Checks {
    let mut checks: Checks = Default::default();
    // all integers?
    checks.intkey = pk.chars().all(char::is_numeric);
    // hex?
    checks.hexkey = hex::decode(pk).is_ok();
    checks
}

#[cfg(test)]
mod tests {
    use crate::pivkeychecks;
    #[test]
    fn key_format_assessment() {
        let mut checks = pivkeychecks("12345".to_string());
        assert_eq!(checks.intkey, true);
        assert_eq!(checks.hexkey, false);  // odd length key
        assert_eq!(checks.base58key, false); // for now!
        assert_eq!(checks.wifkey, false); // for now!
        checks = pivkeychecks("1234".to_string());
        assert_eq!(checks.intkey, true);
        assert_eq!(checks.hexkey, true);  // even length key
        assert_eq!(checks.base58key, false); // for now!
        assert_eq!(checks.wifkey, false); // for now!

    }
}
