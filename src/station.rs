use crate::transceive::{ReceiveCD, Transceive, Initialize};

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
