
#![allow(non_snake_case)]

#[macro_use(value_t)]
extern crate clap;
extern crate rand;

use clap::{Arg, ArgMatches, App};
use rand::distributions::
    {Distribution};
use rand::distributions::WeightedIndex;
use rand::thread_rng;
use rand::{SeedableRng};
use rand::rngs::{StdRng};

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
        .get_matches();

    run(matches);
}

fn run(matches: ArgMatches) {

    let num_iter: i32 = value_t!(matches.value_of("num_iter"), i32).unwrap();

    let take_base = TakeBaseProb {
        X1B_23: 0.25,
        X1B_3H: 0.5,
        X2B_3H: 0.25
    };

    let event_probs = EventProbs { 
        X1B: 0.08,
        BB: 0.15,
        X2B: 0.05,
        X3B: 0.005,
        X4B:  0.03
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

    let seed = [
        1,0,0,0, 23,0,0,0, 200,1,0,0, 210,30,0,0,
        0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];
    
    let mut rngX: StdRng = SeedableRng::from_seed(seed);
 
    let mut rng = thread_rng();

    let dist = WeightedIndex::new(&weights).unwrap();
    
    let mut simulate_event = || -> Event {
        items[dist.sample(&mut rngX)]
    };


    let mut bo = BOState {
        bases: [0,0,0],
        outs: 0
    };

    let mut old_bo = BOState {
        bases: [0,0,0],
        outs: 0
    };

    let mut state_transition = |initial_state: &mut BOState| -> BOState {
        let ev = simulate_event();
        initial_state.evolve_state(ev)
    };
   
    let sim_step = return_closure();
    // for _ in 0..num_iter {
    //     bo = state_transition(&mut old_bo);
    //     let runs_scored = bo.runs_scored(old_bo);
    //     println!("******************");
    //     println!("{:?} {}", bo, runs_scored);
    //     old_bo = bo;
    // }

    for _ in 0..num_iter {
        bo = sim_step(&mut old_bo);
        let runs_scored = bo.runs_scored(old_bo);
        println!("******************");
        println!("{:?} {:?} {}", old_bo, bo, runs_scored);
        old_bo = bo;
    }

}

fn return_closure() -> impl Fn(&mut BOState) -> BOState {
        let take_base = TakeBaseProb {
        X1B_23: 0.25,
        X1B_3H: 0.5,
        X2B_3H: 0.25
    };

    let event_probs = EventProbs { 
        X1B: 0.08,
        BB: 0.15,
        X2B: 0.05,
        X3B: 0.005,
        X4B:  0.03
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

    let seed = [
        1,0,0,0, 23,0,0,0, 200,1,0,0, 210,30,0,0,
        0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];
    
    let mut rngX: StdRng = SeedableRng::from_seed(seed);
 
    let mut rng = thread_rng();

    let dist = WeightedIndex::new(&weights).unwrap();

    move |s| {
        let ev = items[dist.sample(&mut thread_rng())];
        s.evolve_state(ev)
    }
    
}
 
// fn take_closure<F>(f: F, x: &mut BOState) -> BOState 
//     where F: Fn(i32) -> i32 {
//         f(x)
//     }

fn runs_from_state(mut state: BOState, running_total: i32) -> i32 {
    let sim_step = return_closure();
    match state.outs {
        3 => running_total,
        _ => {
            let updated_state = sim_step(&mut state);
            let updated_runs = updated_state.runs_scored(state);
            runs_from_state(updated_state, running_total + updated_runs)
        }
    }

}


// fn runs_from_state(initial_state: BOState, running_total: i32) -> i32 {
//     match initial_state.outs {
//         3 => running_total,
//         _ => {
//             let ev = simulate_event();
//             let new_state = initial_state.evolve_state(ev);
//             let dr = new_state.runs_scored(initial_state);
//             runs_from_state(new_state, running_total + dr)
//         }
//     }
// }

// fn runs_from_state<F>(go: F, initial_state: BOState) -> i32
//     where F: Fn(i: BOState, j: i32) -> i32 {
        
//         match initial_state.outs {
//             3 => j
//             _ => {
//                 let bo = go(initial_state);
//                 let runs_scored = bo.runs_scored(initial_state);
//                 runs_from_state(go, )
//             }
//         }        
//     }
    

fn prob_to_weight(p: f64) -> u32 {
    (10000.0 * p) as u32
}

#[derive(Clone, Copy, Debug)]
struct EventProbs {
    X1B: f64,
    X2B: f64,
    X3B: f64,
    X4B: f64,
    BB: f64
}

impl EventProbs {
    fn sum_probs(&self) -> f64 {
        self.X1B + self.X2B + self.X3B + self.X4B + self.BB
    }
}

#[derive(Clone, Copy, Debug)]
struct TakeBaseProb {
    X1B_3H: f64,
    X1B_23: f64,
    X2B_3H: f64
}

#[derive(Clone, Copy, Debug)]
struct BOState {
    bases: [i32; 3],
    outs: i32
}

#[derive(Clone, Copy, Debug)]
struct GameState {
    bo: BOState,
    runs: i32
}

impl BOState {
    fn runs_scored(&self, last_state: BOState) -> i32 {
        // before = after
        // runners + 1 = runners + runs scored + outs made
        // runs scored = -d(runners) - d(outs) + 1 
        let runners_end: i32 = self.bases.iter().sum();
        let runners_start: i32 = last_state.bases.iter().sum();
        let douts = self.outs - last_state.outs;
        -douts - (runners_end - runners_start) + 1
    }
}

impl BOState {
    fn evolve_state(&self, ev: Event) -> BOState {
        
        match ev {
            
            Event::Out => {
                BOState {
                    bases: self.bases,
                    outs: self.outs + 1,
                }
            },

            Event::BB => {
                let b1 = 1;
                let b2 = if self.bases[0] == 1 { 1 } else {self.bases[1]}; 
                let b3 = if self.bases[0] == 1 && self.bases[1] == 1 { 1 } else {self.bases[2]};
                BOState {
                    bases: [b1, b2, b3],
                    outs: self.outs,
                }
            },

            Event::X1B(go_34, go_23) => {
                let b1 = 1;

                let (b3, b2) = match (go_34, go_23) { 
                    // (take home, take 3rd) => (3rd base, 2nd base)
                    (true, true) => (self.bases[0], 0),
                    (true, false) => (0, self.bases[0]),
                    (false, false) => (self.bases[1], self.bases[0]),
                    (false, true) => panic!("trailing runner can't take a base unless lead runner does also!")
                };
                
                BOState {
                    bases: [b1, b2, b3],
                    outs: self.outs,
                }
            },

            Event::X2B(go_34) => {
                let b1 = 0;
                let b2 = 1;
                let b3 = if go_34 { 0 } else { self.bases[0] };
                BOState {
                    bases: [b1, b2, b3],
                    outs: self.outs,
                }
            },

            Event::X3B => {
                let b1 = 0;
                let b2 = 0;
                let b3 = 1;
                BOState {
                    bases: [b1, b2, b3],
                    outs: self.outs,
                }
            },

            Event::X4B => {
                BOState {
                    bases: [0, 0, 0],
                    outs: self.outs,
                }
            }

        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Event {
    X1B(bool, bool),
    X2B(bool),
    X3B,
    X4B,
    BB,
    Out
}


fn simulate_event_closure(weights: &[u32]) -> Box<dyn Fn(i32) -> i32> {
    
    Box::new(|x| x + 1)
}

fn simulate_eventX(weights: &[u32], items: &[Event], rng: &mut StdRng) -> Event {
    println!("{:?}", weights);
    let dist = WeightedIndex::new(weights).unwrap();
    items[dist.sample(&mut *rng)] 
}
