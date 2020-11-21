use structopt::StructOpt;
use rand::Rng;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short)]
    max: usize,
    #[structopt(long, short)]
    sample_size: usize,
    #[structopt(default_value="1", long, short)]
    lead_digits: u8,
    #[structopt(default_value="1", long, short="n")]
    min: usize,
    #[structopt(short, long)]
    debug: bool,
}

fn generate_sample(max_value: usize, target_sample_size: usize, min_value: usize, debug: bool) {
    let mut sample: Vec<usize> = Vec::new();
    let division_denominator: usize = max_value + 1 - min_value;
    let target_chance:f32 = (1.0)/(division_denominator as f32);
    let mut float_0_1:f32;
    let mut rng=rand::thread_rng();
    let mut pushed_values:usize=0;
    let mut current_size:usize;
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
    if debug { println!("{:?}", sample);}
}

fn main() {
    let args=Opt::from_args();
    generate_sample(args.max, args.sample_size, args.min, args.debug)
}
