pub struct Station {
    id: usize,
    p: usize,
    received: Vec<u8>,   // buffer, store received data
    init_data: InitData,
}

struct InitData {
    n_i: usize,      // from Nakano, Et al.: N_i
    local_l: usize,  // from Nakano, Et al.: l
    global_l: usize, // from Nakano, Et al.: L
    l_i: usize,      // from Nakano, Et al.: l_i
    l_j: usize,      // from Nakano, Et al.: l_i-1
}

impl Station {
    /// A round of the inner for-loop of Interleaved_Initialize() from the paper.
    fn receiver_interleaved_round(mut self) {
        let l: usize = 0;
        // get l from previous round l_j (l_i - 1) (discard until message found?)

        for k in (self.init_data.l_j + 1)..self.init_data.l_i {
            // detect if NULL, SINGLE, or COLLISION
                // call fn defined by trait
            // transmit n_i and l
                // call fn defined by trait
        }
        self.init_data.l_i = l;
    }
}
