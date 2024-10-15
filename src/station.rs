use crate::transceive::{ReceiveCD, Transceiver};

pub struct Station {
    id: usize,
    p: usize,
    received: Vec<u8>, // buffer, store received data
    init_data: InitData,
}

struct InitData {
    n_i: usize,      // from Nakano, Et al.: N_i
    local_l: usize,  // from Nakano, Et al.: l
    global_l: usize, // from Nakano, Et al.: L
    l_i: usize,      // from Nakano, Et al.: l_i
    l_j: usize,      // from Nakano, Et al.: l_i-1
}

impl Transceiver for Station {
    fn send(&mut self) {
        // TODO
    }

    fn recv(&mut self) -> ReceiveCD {
        // TODO
        ReceiveCD::None
    }

    fn recv_timeslice(&mut self) -> ReceiveCD {
        // TODO
        // drain and cache data, use aggregating OR priority queue for synchronization
        ReceiveCD::None
    }

    fn interleaved_round(&mut self) {
        let l: usize = 0;
        // TODO: get l from previous round l_j (l_i - 1) (discard until message found?)

        for k in (self.init_data.l_j + 1)..self.init_data.l_i {
            self.interleaved_round_step(k);
        }
        self.init_data.l_i = l;
    }
}
