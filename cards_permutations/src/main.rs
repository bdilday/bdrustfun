
extern crate rand;
extern crate statistical;
extern crate rayon;

#[macro_use(value_t)]
extern crate clap;

use rand::distributions::{Distribution};
use rand::distributions::{Bernoulli};
use statistical::{mean, standard_deviation};
use rayon::prelude::*;
use clap::{Arg, ArgMatches, App};

fn main() {
    
     let matches = App::new("card_permutations")
        .version("0.1")
        .about("how many packs of cards to complete the set?")
        .arg(Arg::with_name("num_pool")
            .short("p")
            .long("num_pool")
            .takes_value(true)
            .multiple(false)
            .help("number of cards in set (the pool)")
            .required(true)
            )
        .arg(Arg::with_name("num_cards")
            .short("c")
            .long("num_cards")
            .takes_value(true)
            .multiple(false)
            .help("number of cards per pack")
            .required(true)
            )
        .arg(Arg::with_name("num_iter")
            .short("n")
            .long("num_iter")
            .takes_value(true)
            .multiple(false)
            .help("number of simulation iterations")
            .required(true)
            )
        .get_matches();

    run(matches);
}

fn run(matches: ArgMatches) {

    let num_suc = 0;

    let num_pool: i32 = value_t!(matches.value_of("num_pool"), i32).unwrap();
    let num_cards: i32 = value_t!(matches.value_of("num_cards"), i32).unwrap();
    let num_iter: i32 = value_t!(matches.value_of("num_iter"), i32).unwrap();

    let ans: Vec<f64> = (0..num_iter).into_par_iter().map(|_| {
        number_trials(num_suc, num_pool, num_cards) as f64}
        ).collect();
    
    let m = mean(&ans);
    let s = standard_deviation(&ans, Some(m));
   
    println!(
"
mean packs bought : {:.4}
standard dev      : {:.4}
std. error on mean: {:.4}", m, s, s/(num_iter as f64).sqrt());
}

fn number_trials(num_suc: i32, num_pool: i32, num_cards: i32) -> i32 {
    let mut counter = 0;
    let mut m = num_suc;
    while m < num_pool {
        m += update_collection(m, num_pool, num_cards, 0);
        counter += 1;
    }
    counter
}

fn update_collection(num_suc: i32, 
                     num_pool: i32, 
                     num_cards: i32, running_total: i32) -> i32 {
    
    match num_cards {
        0 => running_total,
        _ => {
            let c = generate_samp(num_suc, num_pool);
            update_collection(num_suc+c-1, num_pool-1, num_cards-1, running_total+c)
        }
    }

}

fn generate_samp(num_suc: i32, num_pool: i32) -> i32 {
    let p = ((num_pool - num_suc) as f64) / (num_pool as f64);
    let d = Bernoulli::new(p);
    let v = d.sample(&mut rand::thread_rng());    
    
    match v {
        true => 1,
        false => 0
    }

}