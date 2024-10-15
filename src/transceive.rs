pub struct RedrData {
    // TODO: maybe...
    // id: usize
    // data: T
}

/// Indicates state of transmission for a given synchronized timeslice.
pub enum ReceiveCD {
    Collision,
    Single(RedrData),
    None,
}

pub trait Transceiver {
    /// Send/transmit single chunk of data.
    fn send(&mut self);

    /// Receive single chunk of data.
    /// Data deliniated by TODO (also probably change returned data type?) <<<<<<<<<<<<<<<<<<<<<<<<<<<<<
    fn recv(&mut self) -> ReceiveCD;

    /// Receive data for the current timeslice.
    fn recv_timeslice(&mut self) -> ReceiveCD;

    /// Transmit n_i and l values.
    /// See section 4 of Nakano, Et al.
    fn send_round_step(&mut self) {
        // TODO
        // transmit n_i and l
    }

    /// Facilitate a single iteration of a round of Interleaved_Initialize().
    /// See section 4 of Nakano, Et al.
    fn interleaved_round_step(&mut self, k: usize) {
        // detect if timeslice state is NULL, SINGLE, or COLLISION
        match self.recv_timeslice() {
            // TODO
            ReceiveCD::Collision => (),
            ReceiveCD::Single(_) => (),
            ReceiveCD::None => (),
        }
        // transmit n_i and l
        self.send_round_step();
    }

    /// Facilitate this station's round of Interleaved_Initialize().
    ///
    /// For the given station, conduct a round of the inner for-loop of
    /// Interleaved_Initialize() from section 4 of Nakano, Et al.
    fn interleaved_round(&mut self);

    /// Interleaved initialization to determine transmission order.
    ///
    /// Performs the Interleaved_Initialize algorithm proposed in section 4 of
    /// Nakano, Et al.
    fn interleaved_initialize(&mut self) {
        // TODO
    }
}
