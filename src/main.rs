use rand::Rng;
use std::collections::HashMap;
use structopt::StructOpt;
use textplots::{Chart, Plot, Shape};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short)]
    max: usize,
    #[structopt(long, short)]
    sample_size: usize,
    #[structopt(default_value = "1", long, short)]
    lead_digits: u8,
    #[structopt(default_value = "1", long, short = "n")]
    min: usize,
    #[structopt(short, long)]
    debug: bool,
}

fn generate_sample(
    max_value: usize,
    sample_size: usize,
    min_value: usize,
    lower_boundary: u16,
) -> Vec<usize> {
    let mut sample: Vec<usize> = Vec::new();
    let division_denominator: usize = max_value + 1 - min_value;
    let target_chance: f32 = (1.0) / (division_denominator as f32);
    let mut float_0_1: f32;
    let mut rng = rand::thread_rng();
    let mut pushed_values: usize = 0;
    let mut current_size: usize;
    for _ in 0..sample_size {
        current_size = pushed_values;
        while current_size == pushed_values {
            for current_value in min_value..=max_value {
                float_0_1 = rng.gen();
                if float_0_1 < target_chance && current_value > lower_boundary.into() {
                    sample.push(current_value);
                    pushed_values += 1;
                    break;
                }
            }
        }
    }
    sample
}

fn uint_first_n_digits(value: usize, digit_count: u8) -> u16 {
    value.to_string()[..usize::from(digit_count)]
        .parse::<u16>()
        .unwrap()
}

fn vector_lead_n_digits(sample: Vec<usize>, digit_count: u8, sample_size: usize) -> Vec<u16> {
    let mut vector_lead_digits: Vec<u16> = Vec::new();
    for value in 0..sample_size {
        vector_lead_digits.push(uint_first_n_digits(sample[value], digit_count));
    }
    vector_lead_digits
}

fn lower_digits_boundary(base: u16, count_lead_digits: u32) -> u16 {
    let lower_boundary: u16 = base.pow(count_lead_digits - 1);
    lower_boundary
}

fn upper_digits_boundary(base: u16, count_lead_digits: u32) -> u16 {
    (base.pow(count_lead_digits)) - 1
}

fn main() {
    let args = Opt::from_args();
    let debug = args.debug;
    let max = args.max;
    let sample_size = args.sample_size;
    let min = args.min;
    let num_lead_digits = args.lead_digits;
    let base_ten: u16 = 10;
    let lower_boundary: u16 = lower_digits_boundary(base_ten, num_lead_digits.into());
    let upper_boundary: u16 = upper_digits_boundary(base_ten, num_lead_digits.into());
    let sample: Vec<usize> = generate_sample(max, sample_size, min, lower_boundary);
    if debug {
        println!("Simulated sample data:\n{:?}", sample);
    }
    let lead_digits_vec: Vec<u16> = vector_lead_n_digits(sample, num_lead_digits, sample_size);
    if debug {
        println!("Lead digits:\n{:?}", lead_digits_vec);
    }
    let mut repeated_counts: HashMap<usize, usize> = HashMap::new();
    for index in 0..sample_size {
        let lead_digits_index: usize = usize::from(lead_digits_vec[index]);
        if let std::collections::hash_map::Entry::Vacant(e) = repeated_counts.entry(lead_digits_index) {
            e.insert(1);
        } else {
             *repeated_counts.get_mut(&lead_digits_index).unwrap() += 1;
        }
    }
    let mut points: Vec<(f32, f32)> = Vec::new();
    for i in lower_boundary..=upper_boundary {
        if repeated_counts.contains_key(&usize::from(i)) {
            points.push((i as f32, repeated_counts[&usize::from(i)] as f32))
        }
    }
    println!("Lead digits counts:\n{:?}", points);
    Chart::new(180, 60, lower_boundary.into(), upper_boundary as f32)
        .lineplot(&Shape::Steps(&points))
        .nice();
}
