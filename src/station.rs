use std::collections::VecDeque;

use crate::transceive::{ReceiveCD, Transceive, RoundAction, Interleaved};

pub struct Station {
    id: usize,
    l: usize,          // global L value for tracking initialization progress
    p: usize,          // graph size
    n_i: usize,        // number of stations with data destined for this station
    leader_l: usize,        // l of last leader step
    last_follower_l: usize, // l of last follower step
    next_round_t: usize,    // next outer-round's starting timeslice
    received: Vec<u8>,      // buffer, store received data
    action_queue: VecDeque<RoundAction>,  // actoins to do for this outer-round
}

impl Transceive for Station {
    fn send(&mut self, data: Vec<u8>) {
        // TODO
    }

    fn recv(&mut self) -> ReceiveCD {
        // TODO
        ReceiveCD::None
    }

    fn sync_send(&mut self, timeslice: usize, data: Vec<u8>) {
        // TODO
    }

    fn sync_recv(&mut self, data: Vec<u8>) -> usize {
        // TODO
        0
    }
}


impl Interleaved for Station {
    fn get_n_i(&self) -> usize {
        self.n_i
    }

    fn increment_n_i(&mut self) {
        self.n_i += 1;
    }

    fn get_global_l(&self) -> usize {
        // TODO <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        self.l
    }

    fn set_global_l(&mut self, l: usize) {
        self.l = l;
    }

    fn pop_action(&mut self) -> Option<RoundAction> {
        // TODO <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
    }

    fn push_action(&mut self, action: RoundAction) {
        // TODO <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
    }

    fn get_next_round_t(&self) -> usize {
        self.next_round_t
    }

    fn update_next_round_t(&mut self, start_time: usize) {
        self.next_round_t = start_time;
    }

    fn wait_until_timeslice(&mut self, timeslice: usize) {
        // TODO <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
    }

    fn set_last_leader_action_concluding(&mut self) {
        match self.action_queue.back_mut() {
            Some(action) => *action = match *action {
                RoundAction::Leader(t, l_j, l_i, _) =>
                    RoundAction::Leader(t, l_j, l_i, true),
                RoundAction::Follower(_, _, _) => panic!("error: follower action
                    is back of queue during leader action conclude update"),
                RoundAction::RoundConclusion(_) => panic!("error: conclusion
                    action is back of queue during leader action conclude update"),
            },
            None => panic!("error: updating last leader action to be
                              concluding, but action queue is empty"),
        }
    }

    fn set_last_follower_action_concluding(&mut self) {
        match self.action_queue.back_mut() {
            Some(action) => *action = match *action {
                RoundAction::Leader(_, _, _, _) => panic!("error: leader action
                    is back of queue during follower action conclude update"),
                RoundAction::Follower(t, n_i, _) =>
                    RoundAction::Follower(t, n_i, true),
                RoundAction::RoundConclusion(_) => panic!("error: conclusion
                    action is back of queue during follower action conclude update"),
            },
            None => panic!("error: updating last follower action to be
                              concluding, but action queue is empty"),
        }
    }
}


impl Station {
    fn new(id: usize, p: usize) -> Station {
        let s: Station = Station {
            id,
            l: 1,
            p,
            received: Vec::new(),
            action_queue: VecDeque::new(),
        };

        // TODO populate action_queue with initial actions for first round

        s
    }
}
