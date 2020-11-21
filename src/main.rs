use structopt::StructOpt;
use rand::prelude::*;

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

fn generate_sample(max_value: u64, target_sample_size: u64, min_value: u64) {
    let mut sample: Vec<u64> = Vec::new();
    let target_chance:f64 = (1 as f64)/(max_value as f64 + 1 as f64 - min_value as f64);
    let mut float_0_1:f64;
    let mut rng=rand::thread_rng();
    println!("{:?}", target_chance);
    let mut pushed_values:u64=0;
    let mut current_size:u64;
    for _ in 0..target_sample_size {
        current_size=pushed_values;
        while current_size==pushed_values {
            for current_value in min_value..=max_value {
                float_0_1 = rng.gen();
                if float_0_1 < target_chance {
                    sample.push(current_value);
                    pushed_values+=1;
                    break;
                }
            };
        };
    };
    println!("{:?}", sample);
}

fn main() {
    let args=Opt::from_args();
    generate_sample(args.max, args.sample_size, args.min)
}
