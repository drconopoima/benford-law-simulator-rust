use structopt::StructOpt;
use rand::Rng;
use std::collections::HashMap;

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

fn generate_sample(max_value: usize, sample_size: usize, min_value: usize) -> Vec<usize> {
    let mut sample: Vec<usize> = Vec::new();
    let division_denominator: usize = max_value + 1 - min_value;
    let target_chance:f32 = (1.0)/(division_denominator as f32);
    let mut float_0_1:f32;
    let mut rng=rand::thread_rng();
    let mut pushed_values:usize=0;
    let mut current_size:usize;
    for _ in 0..sample_size {
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
    return sample
}

fn uint_first_n_digits(value:usize, digit_count:u8) -> u16 {
    return value.to_string()[..usize::from(digit_count)].parse::<u16>().unwrap();
}

fn vector_lead_n_digits(sample:Vec<usize>, digit_count:u8, sample_size: usize) -> Vec<u16> {
    let mut vector_lead_digits:Vec<u16>=Vec::new();
    for value in 0..sample_size {
        vector_lead_digits.push(uint_first_n_digits(sample[value],digit_count));
    }
    return vector_lead_digits
}

fn main() {
    let args=Opt::from_args();
    let debug=args.debug;
    let max=args.max;
    let sample_size=args.sample_size;
    let min=args.min;
    let lead_digits=args.lead_digits;
    let sample: Vec<usize> = generate_sample(max, sample_size, min);
    if debug { println!("Simulated sample data:\n{:?}", sample);}
    let lead_digits: Vec<u16> = vector_lead_n_digits(sample, lead_digits, sample_size);
    if debug { println!("Lead digits:\n{:?}", lead_digits);}
    let mut repeated_counts:HashMap<usize,usize> = HashMap::new();
    for index in 0..sample_size {
        let usize_index:usize=usize::from(lead_digits[index]);
        if repeated_counts.contains_key(&usize_index) {
            *repeated_counts.get_mut(&usize_index).unwrap()+=1;
        } else {
            repeated_counts.insert(usize_index, 1);
        }
    }
    println!("{:?}", repeated_counts)
}
