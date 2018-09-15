

#[macro_use(value_t)]
extern crate clap;
extern crate rand;

use clap::{Arg, ArgMatches, App};
use rand::distributions::
    {Weighted, WeightedChoice, Distribution, Bernoulli};

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

    let mut items = vec!(
        Weighted { weight: prob_to_weight(event_probs.X1B * take_base.X1B_23 * take_base.X1B_3H), item: Event::X1B(true, true) },
        Weighted { weight: prob_to_weight(event_probs.X1B * (1.0-take_base.X1B_23) * take_base.X1B_3H), item: Event::X1B(true, false) },
        Weighted { weight: 0, item: Event::X1B(false, true) },
        Weighted { weight: prob_to_weight(event_probs.X1B * (1.0-take_base.X1B_23) * (1.0-take_base.X1B_3H)), item: Event::X1B(false, false) },
        Weighted { weight: prob_to_weight(event_probs.X2B * take_base.X1B_23), item: Event::X2B(true) },
        Weighted { weight: prob_to_weight(event_probs.X2B * (1.0-take_base.X1B_23)), item: Event::X2B(true) },
        Weighted { weight: prob_to_weight(event_probs.X2B), item: Event::X3B },
        Weighted { weight: prob_to_weight(event_probs.X2B), item: Event::X4B },
        Weighted { weight: prob_to_weight(event_probs.X2B), item: Event::BB },
        Weighted { weight: prob_to_weight(outs_prob), item: Event::Out });
        
    println!("{:?}", items);
    simulate_event(&mut items);    
}

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
enum Event {
    X1B(bool, bool),
    X2B(bool),
    X3B,
    X4B,
    BB,
    Out
}


fn simulate_event(mut items: &mut Vec<Weighted<Event>>) -> Event {

    let wc = WeightedChoice::new(&mut items);
    
    let ev = wc.sample(&mut rand::thread_rng());

    // match ev {
    //     Event::X1B(take_base_3rdHome, take_base_2nd3rd) => {
    //         let d = Bernoulli::new(0.5);
    //         Event::X1B(
    //             d.sample(&mut rand::thread_rng()),
    //             d.sample(&mut rand::thread_rng())
    //             )
    //     }
    //     Event::X2B(take_base_3rdHome) => {
    //         let d = Bernoulli::new(0.5);
    //         Event::X2B(d.sample(&mut rand::thread_rng()))
    //     }
    //     _ => ev
    // }

    Event::X4B
}
