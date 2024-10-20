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
    fn send(&mut self, data: Vec<u8>) {
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

    fn handle_collision(&mut self) {
        self.init_data.local_l += 2;
    }

    fn handle_single(&mut self) {
        self.init_data.n_i += 1;
    }

    fn send_round_step(&mut self) {
        // TODO
        // transmit n_i and l
        let data = self.init_data.n_i.to_be_bytes().to_vec();  // TODO: still need l
        self.send(data);
    }

    fn interleaved_round(&mut self) {
        let new_l_j: usize = 0;
        // TODO: get l from previous round l_j (l_i - 1) (discard until message found?)

        for _ in (self.init_data.l_j + 1)..self.init_data.l_i {
            self.interleaved_round_step();
        }

        self.init_data.l_i = self.init_data.local_l;
        self.init_data.l_i = new_l_j;
    }
}
