
#![allow(non_snake_case)]

#[derive(Clone, Copy, Debug)]
pub struct BOState {
    pub bases: [i32; 3],
    pub outs: i32
}

impl BOState {
    pub fn runs_scored(&self, last_state: BOState) -> i32 {
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
    pub fn evolve_state(&self, ev: Event) -> BOState {
        
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
pub struct EventProbs {
    pub X1B: f64,
    pub X2B: f64,
    pub X3B: f64,
    pub X4B: f64,
    pub BB: f64
}

impl EventProbs {
    pub fn sum_probs(&self) -> f64 {
        self.X1B + self.X2B + self.X3B + self.X4B + self.BB
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TakeBaseProb {
    pub X1B_3H: f64,
    pub X1B_23: f64,
    pub X2B_3H: f64
}


#[derive(Clone, Copy, Debug)]
pub enum Event {
    X1B(bool, bool), // (take home, take third)
    X2B(bool), // take home
    X3B,
    X4B,
    BB,
    Out
}
