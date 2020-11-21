use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short)]
    max: u64,
    #[structopt(long, short)]
    sample_size: u64,
    #[structopt(default_value="1", long, short)]
    digits: u8,
    #[structopt(default_value="1", long, short="n")]
    min: u64
}

fn generate_sample(max_value: u64, target_sample_size: u64, min_value: u64, number_digits: u8) {
    for current_value in min_value..=max_value {
        println!("{:?}", current_value)
    }
}

fn main() {
    let args=Opt::from_args();
    generate_sample(args.max, args.sample_size, args.min, args.digits)
}
