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

fn main() {
    // command line flags and arguments
    let opt = Opt::from_args();
    println!("{:?}", opt);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
