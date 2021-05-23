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
struct Keychecks {
    key_is_int: bool,
    key_is_hex: bool,
    key_is_base58: bool,
    key_is_wif: bool,
}

fn main() {
    // command line flags and arguments
    let opt = Opt::from_args();
    if opt.debug {
        println!("{:?}", opt);
        println!("Private key passed: {:?}", opt.privkey);
    }

    let privkey = opt.privkey.as_str();

    // run checks on the passed privkey
    let checks = pivkeychecks(&privkey);

    // TODO:
    //   many other checks
    if opt.debug {
        println!("Private key assessment: {:?}", checks);
    }

    // bail out here if our checks find nothing usable.
    if !checks.key_is_int && !checks.key_is_hex && !checks.key_is_base58 && !checks.key_is_wif {
        println!("Could not interpret the private key.");
        return;
    }

    // TODO:
    //   process

    if opt.debug {
        println!("Done.");
    }
}

fn pivkeychecks(pk: &str) -> Keychecks {
    extern crate rust_base58;
    use rust_base58::{ToBase58, FromBase58};
    let mut checks: Keychecks = Default::default();
    // all integers?
    checks.key_is_int = pk.chars().all(char::is_numeric);
    // base58?
    checks.key_is_base58 = pk.from_base58().is_ok();
    // hex?
    checks.key_is_hex = hex::decode(pk).is_ok();
    checks
}

#[cfg(test)]
mod tests {
    use crate::pivkeychecks;
    #[test]
    fn key_odd_length_integer() {
        let checks = pivkeychecks("12345");
        assert_eq!(checks.key_is_int, true);
        assert_eq!(checks.key_is_hex, false); // odd length key
        assert_eq!(checks.key_is_base58, true);
        assert_eq!(checks.key_is_wif, false); // for now!
    }
    #[test]
    fn key_even_length_integer() {
        let checks = pivkeychecks("1234");
        assert_eq!(checks.key_is_int, true);
        assert_eq!(checks.key_is_hex, true); // even length key
        assert_eq!(checks.key_is_base58, true);
        assert_eq!(checks.key_is_wif, false); // for now!
    }
    #[test]
    fn key_negative_integer() {
        let checks = pivkeychecks("-1234");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, false); // for now!
        assert_eq!(checks.key_is_wif, false); // for now!
    }
    #[test]
    fn key_invalid() {
        let checks = pivkeychecks("love");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, false); // l and o are invalid
        assert_eq!(checks.key_is_wif, false); // for now!
    }
    #[test]
    fn key_base58_but() {
        let checks = pivkeychecks("yuve");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, true); // l and o are invalid
        assert_eq!(checks.key_is_wif, false); // for now!
    }

}
