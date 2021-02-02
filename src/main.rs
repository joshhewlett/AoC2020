use std::env;

mod challenges;
mod utilities;

fn main() {
    let challenges: Vec<fn()> = vec![challenges::day01::run, challenges::day02::run];
    let args: Vec<String> = env::args().collect();

    let challenge_to_run: usize = &args[1].parse::<usize>().unwrap() - 1;

    challenges[challenge_to_run]();
}
