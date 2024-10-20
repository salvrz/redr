use std::ops::Range;

pub struct RedrData {
    // TODO: maybe...
    // id: usize
    // data: T
}

/// Indicates state of transmission for a given synchronized timeslice.
pub enum ReceiveCD {
    Collision,
    Single,
    None,
}

pub trait Transceive {
    /// Send/transmit single chunk of data.
    fn send(&mut self, data: Vec<u8>);

    /// Receive single chunk of data.
    /// Data deliniated by TODO (also probably change returned data type?) <<<<<<<<<<<<<<<<<<<<<<<<<<<<<
    fn recv(&mut self) -> ReceiveCD;

    // Synchronously send data
    fn sync_send(&mut self, timeslice: usize, data: Vec<u8>);
}

pub trait Initialize: Transceive {
    fn get_n_i(&self) -> usize;

    fn set_n_i(&mut self, new_n_i: usize);

    fn get_l_i(&self) -> usize;

    fn set_l_i(&mut self, new_l_i: usize);

    fn get_local_l(&self) -> usize;

    fn set_local_l(&mut self, new_local_l: usize);

    fn get_l_j(&self) -> usize;

    fn set_l_j(&mut self, new_l_j: usize);

    /// Receive data for the current synchronized timeslice.
    fn recv_timeslice(&mut self) -> ReceiveCD;

    /// The range for a stations round. l_(i-1)+1..l_i
    fn interleaved_round_range(&self) -> Range<usize> {
        (self.get_l_j() + 1)..self.get_l_i()
    }

    /// Handle a collision received during a round of Interleaved_Initialize().
    fn handle_collision(&mut self) {
        self.set_local_l(self.get_local_l() + 2)
    }

    /// Handle a single response received during a round of
    /// Interleaved_Initialize().
    fn handle_single(&mut self) {
        self.set_n_i(self.get_n_i() + 1)
    }

    /// Transmit n_i and l values.
    /// See section 4 of Nakano, Et al.
    fn send_round_step(&mut self, round: usize) {
        let mut data: Vec<u8> = self.get_n_i().to_be_bytes().to_vec();
        data.append(&mut self.get_local_l().to_be_bytes().to_vec());
        self.sync_send(round, data);
    }

    /// Facilitate a single iteration of a round of Interleaved_Initialize().
    /// See section 4 of Nakano, Et al.
    fn interleaved_round_step(&mut self, round: usize) {
        // detect if timeslice state is NULL, SINGLE, or COLLISION
        match self.recv_timeslice() {
            ReceiveCD::Collision => self.handle_collision(),
            ReceiveCD::Single => self.handle_single(),
            ReceiveCD::None => (),
        }
        // transmit n_i and l
        self.send_round_step(round);
    }

    /// Facilitate this station's round of Interleaved_Initialize().
    ///
    /// For the given station, conduct a round of the inner for-loop of
    /// Interleaved_Initialize() from section 4 of Nakano, Et al.
    fn interleaved_round(&mut self) {
        let new_l_j: usize = 0;
        // TODO: get l from previous round l_j (l_i - 1) (discard until message found
        // for round l_j)

        for k in self.interleaved_round_range() {
            self.interleaved_round_step(k);
        }

        self.set_l_i(self.get_local_l());
        self.set_l_j(new_l_j);
    }

    /// Interleaved initialization to determine transmission order.
    ///
    /// Performs the Interleaved_Initialize algorithm proposed in section 4 of
    /// Nakano, Et al.
    fn interleaved_initialize(&mut self) {
        // TODO
    }
}
