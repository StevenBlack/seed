extern crate hex;

//
//  https://observablehq.com/@jimbojw/grokking-bip39
//

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

#[derive(Default, Debug)]
struct Output {
    private_key_string: String,
    mainnet_byte_string: String,
    mainnet_wif: String,

    testnet_byte_string: String,
    testnet_wif: String,
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

    // process
    // The hex case
    if checks.key_is_hex {
        // ref: https://en.bitcoin.it/wiki/Wallet_import_format
        // $ cargo run -- 0C28FCA386C7A227600B2FE50B7CAE11EC86D3BF1FBE471BE89827E19D72AA1D
        //
        // we need the hex string to be 64 characters
        let paddedprivkey = format!("{:0>64}", privkey);
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(paddedprivkey, &mut bytes).expect("Hex decoding failed");
        println!("decoded pk: {:?}", bytes);

        println!("Privkey with prefix: {:?}", ["80", privkey].join(""));
        let mut output: Output = Default::default();

        // mainnet
        let mainnet = ["80", privkey].join("");
        let mainnet_checksum: String = sha256_twice_checksum(["80", privkey].join(""));

        output.private_key_string = format!("{:0>64}", privkey);
        output.mainnet_byte_string = [mainnet, mainnet_checksum].join("");

        // testnet
        let testnet = ["ef", privkey].join("");
        let testnet_checksum: String = sha256_twice_checksum(["ef", privkey].join(""));

        output.private_key_string = format!("{:0>64}", privkey);
        output.testnet_byte_string = [testnet, testnet_checksum].join("");

        println!("Output: {:?}", output);
    }

    if opt.debug {
        println!("Done.");
    }
}

fn sha256_twice_checksum(k: String) -> String {
    use sha256::digest_bytes;
    let bytes = hex::decode(k.clone()).unwrap();
    let hash1 = digest_bytes(&bytes);
    let bytes1 = hex::decode(hash1.clone()).unwrap();
    let hash2 = digest_bytes(&bytes1);
    let checksum: String = hash2[..8].to_string();
    checksum
}

fn pivkeychecks(pk: &str) -> Keychecks {
    extern crate rust_base58;
    use rust_base58::FromBase58;
    let mut checks: Keychecks = Default::default();
    // all integers?
    checks.key_is_int = pk.chars().all(char::is_numeric);
    // base58?
    checks.key_is_base58 = pk.from_base58().is_ok();
    // hex?
    checks.key_is_hex = hex::decode(pk).is_ok();
    // wif?
    checks.key_is_wif = bitcoin::PrivateKey::from_wif(pk).is_ok();
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
        assert_eq!(checks.key_is_wif, false);
    }
    #[test]
    fn key_even_length_integer() {
        let checks = pivkeychecks("1234");
        assert_eq!(checks.key_is_int, true);
        assert_eq!(checks.key_is_hex, true); // even length key
        assert_eq!(checks.key_is_base58, true);
        assert_eq!(checks.key_is_wif, false);
    }
    #[test]
    fn key_even_length_integer_0x_prefix() {
        // not accepting 0x prefix for hex.
        let checks = pivkeychecks("0x1234");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false); // even length key
        assert_eq!(checks.key_is_base58, false);
        assert_eq!(checks.key_is_wif, false);
    }
    #[test]
    fn key_negative_integer() {
        let checks = pivkeychecks("-1234");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, false); // for now!
        assert_eq!(checks.key_is_wif, false);
    }
    #[test]
    fn key_invalid() {
        let checks = pivkeychecks("love");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, false); // l and o are invalid
        assert_eq!(checks.key_is_wif, false);
    }
    #[test]
    fn key_base58_but() {
        let checks = pivkeychecks("yuve");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, true); // l and o are invalid
        assert_eq!(checks.key_is_wif, false);
    }
    #[test]
    fn key_wif_testnet_compressed() {
        let checks = pivkeychecks("cVt4o7BGAig1UXywgGSmARhxMdzP5qvQsxKkSsc1XEkw3tDTQFpy");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, true);
        assert_eq!(checks.key_is_wif, true);
    }
    #[test]
    fn key_wif_mainnet_uncompressed() {
        let checks = pivkeychecks("5JYkZjmN7PVMjJUfJWfRFwtuXTGB439XV6faajeHPAM9Z2PT2R3");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, true);
        assert_eq!(checks.key_is_wif, true);
    }
    #[test]
    fn key_wif_invalid_length() {
        let checks = pivkeychecks("5JYkZjmN7PVMjJUfJWfRFwtuXTGB439XV6faajeHPAM9Z2PT2R3444");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, true);
        assert_eq!(checks.key_is_wif, false);
    }
    #[test]
    fn key_wif_invalid_length2() {
        let checks = pivkeychecks("5JYkZj7PVMjJUfJWfRFwtuXTGB439XV6faajeHPAM9Z2PT2R3");
        assert_eq!(checks.key_is_int, false);
        assert_eq!(checks.key_is_hex, false);
        assert_eq!(checks.key_is_base58, true);
        assert_eq!(checks.key_is_wif, false);
    }

    use crate::sha256_twice_checksum;
    #[test]
    fn sha256_twice_mainnet() {
        assert_eq!(
            sha256_twice_checksum(
                "800C28FCA386C7A227600B2FE50B7CAE11EC86D3BF1FBE471BE89827E19D72AA1D".to_string()
            ),
            "507A5B8D".to_lowercase().to_string()
        );
    }

    #[test]
    fn sha256_twice_testnet() {
        assert_eq!(
            sha256_twice_checksum(
                "ef619c335025c7f4012e556c2a58b2506e30b8511b53ade95ea316fd8c3286feb9".to_string()
            ),
            "5ea65746".to_lowercase().to_string()
        );
    }
}
