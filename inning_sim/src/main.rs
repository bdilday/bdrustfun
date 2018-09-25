
#![allow(non_snake_case)]

#[macro_use(value_t)]
extern crate clap;
extern crate rand;
extern crate rayon;
extern crate statistical;
extern crate serde;
extern crate serde_json; // future
mod GameState;

use GameState::{BOState, Event, EventProbs, TakeBaseProb};
use clap::{Arg, ArgMatches, App};
use rand::distributions::
    {Distribution};
use rand::distributions::WeightedIndex;
use rand::thread_rng;
// use rand::{SeedableRng};
// use rand::rngs::{StdRng};
use rayon::prelude::*;
use statistical::{mean, standard_deviation};

fn main() {
    let matches = App::new("inning_sim")
        .version("0.1")
        .about("simulate the number of runs scored in a 3-out inning")

        .arg(Arg::with_name("num_iter")
            .short("n")
            .long("num_iter")
            .takes_value(true)
            .multiple(false)
            .help("number of simulations to run")
            .required(true)
            )

        .arg(Arg::with_name("p0")
            .long("p0")
            .takes_value(true)
            .multiple(false)
            .help("probability for BB")
            .required(true)
            )

        .arg(Arg::with_name("p1")
            .long("p1")
            .takes_value(true)
            .multiple(false)
            .help("probability for X1B")
            .required(true)
            )

        .arg(Arg::with_name("p2")
            .long("p2")
            .takes_value(true)
            .multiple(false)
            .help("probability for X2B")
            .required(true)
            )

        .arg(Arg::with_name("p3")
            .long("p3")
            .takes_value(true)
            .multiple(false)
            .help("probability for X3B")
            .required(true)
            )

        .arg(Arg::with_name("p4")
            .long("p4")
            .takes_value(true)
            .multiple(false)
            .help("probability for X4B")
            .required(true)
            )

        .get_matches();

    run(matches);
}

fn run(matches: ArgMatches) {

    let num_iter: i32 = value_t!(matches.value_of("num_iter"), i32).unwrap();
    let p0: f64 = value_t!(matches.value_of("p0"), f64).unwrap();
    let p1: f64 = value_t!(matches.value_of("p1"), f64).unwrap();
    let p2: f64 = value_t!(matches.value_of("p2"), f64).unwrap();
    let p3: f64 = value_t!(matches.value_of("p3"), f64).unwrap();
    let p4: f64 = value_t!(matches.value_of("p4"), f64).unwrap();
    

    
    let bo = BOState {
        bases: [0,0,0],
        outs: 0
    };

    let event_probs = EventProbs { 
        BB: p0,
        X1B: p1,
        X2B: p2,
        X3B: p3,
        X4B: p4
    };


    let mut sim_step = return_closure(event_probs);

    let ans: Vec<f64> = (0..num_iter).into_par_iter().map(|_| {
        runs_from_state(bo, 0, &sim_step) as f64}
        ).collect();

    let m = mean(&ans);
    let s = standard_deviation(&ans, Some(m));
   
    println!(
"
mean run          : {:.4}
standard dev      : {:.4}
std. error on mean: {:.4}", m, s, s/(num_iter as f64).sqrt());
   
}

fn return_closure(event_probs: EventProbs) -> impl Fn(&mut BOState) -> BOState {
    
    let take_base = TakeBaseProb {
        X1B_23: 0.25,
        X1B_3H: 0.5,
        X2B_3H: 0.25
    };

    let outs_prob = 1.0 - event_probs.sum_probs();
    assert!(outs_prob > 0.0);


    let items = [
        Event::X1B(true, true),
        Event::X1B(true, false),
        Event::X1B(false, false),
        Event::X2B(true),
        Event::X2B(false),
        Event::X3B,
        Event::X4B,
        Event::BB,
        Event::Out
        ];

    let weights = [
        prob_to_weight(event_probs.X1B * take_base.X1B_3H * take_base.X1B_23), 
        prob_to_weight(event_probs.X1B * take_base.X1B_3H * (1.0-take_base.X1B_23)),
        prob_to_weight(event_probs.X1B * (1.0-take_base.X1B_3H) * (1.0-take_base.X1B_23)), 
        prob_to_weight(event_probs.X2B * take_base.X2B_3H), 
        prob_to_weight(event_probs.X2B * (1.0-take_base.X2B_3H)), 
        prob_to_weight(event_probs.X3B), 
        prob_to_weight(event_probs.X4B), 
        prob_to_weight(event_probs.BB), 
        prob_to_weight(outs_prob)
        ];
         
    assert!(weights.len() == items.len());

    // let seed = [
    //     1,0,0,0, 23,0,0,0, 200,1,0,0, 210,30,0,0,
    //     0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];
    

    let dist = WeightedIndex::new(&weights).unwrap();

    move |s| {
        let ev = items[dist.sample(&mut thread_rng())];
        s.evolve_state(ev)
    }
    
}
 

fn prob_to_weight(p: f64) -> u32 {
    (10000.0 * p) as u32
}

fn runs_from_state<F>(mut state: BOState, running_total: i32, sim_step: F) -> i32 
    where F: Fn(&mut BOState) -> BOState {
    match state.outs {
        3 => running_total,
        _ => {
            let updated_state = sim_step(&mut state);
            let updated_runs = updated_state.runs_scored(state);
            runs_from_state(updated_state, running_total + updated_runs, sim_step)
        }
    }
}


