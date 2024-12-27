use std::collections::VecDeque;

use crate::transceive::{ReceiveCD, Transceive, Initialize, RoundAction};

pub struct Station {
    id: usize,
    l: usize,          // global L value for tracking initialization progress
    p: usize,          // graph size
    received: Vec<u8>, // buffer, store received data
    action_queue: VecDeque<RoundAction>,  // actoins to do for this outer-round
    init_data: InitData,
}

struct InitData {
    n_i: usize,      // from Nakano, Et al.: N_i
    local_l: usize,  // from Nakano, Et al.: l
    global_l: usize, // from Nakano, Et al.: L
    l_i: usize,      // from Nakano, Et al.: l_i
    l_j: usize,      // from Nakano, Et al.: l_i-1
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

impl Initialize for Station {
    fn get_n_i(&self) -> usize {
        self.init_data.n_i
    }

    fn set_n_i(&mut self, new_n_i: usize) {
        self.init_data.n_i = new_n_i;
    }

    fn get_l_i(&self) -> usize {
        self.init_data.l_i
    }

    fn set_l_i(&mut self, new_l_i: usize) {
        self.init_data.l_i = new_l_i;
    }

    fn get_local_l(&self) -> usize {
        self.init_data.local_l
    }

    fn set_local_l(&mut self, new_local_l: usize) {
        self.init_data.local_l = new_local_l;
    }

    fn get_l_j(&self) -> usize {
        self.init_data.l_j
    }

    fn set_l_j(&mut self, new_l_j: usize) {
        self.init_data.l_j = new_l_j;
    }

    fn recv_timeslice(&mut self) -> ReceiveCD {
        // TODO
        // drain and cache data, use aggregating OR priority queue for synchronization
        ReceiveCD::None
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
            init_data: InitData {
                n_i: 0,
                local_l: 0,
                global_l: 0,
                l_i: 0,
                l_j: 0,
            },
        };

        // TODO populate action_queue with initial actions for first round

        s
    }
}
