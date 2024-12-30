use std::ops::Range;

pub struct RedrData {
    // TODO: maybe...
    // id: usize
    // data: T
}

/// Actions which stations can perform during an outer-round.
pub enum RoundAction {
    // Leader: timeslice, l_(i-1), l_i
    Leader(usize, usize, usize),
    // Follower: timesice, previous N_i
    Follower(usize, usize),
    // Round Conclusion: timeslice, N, L
    RoundConclusion(usize, usize, usize),
}

/// Indicates state of transmission for a given synchronized timeslice.
pub enum ReceiveCD {
    Collision,
    // Follower broadcast: 1, 1
    // Leader step conclusion broadcast: N_i, l
    Single(usize, usize),
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

    // Synchronously receive data for next timeslice, returning timeslice id.
    fn sync_recv(&mut self, data: Vec<u8>) -> usize;
}

// Second impl attempt
pub trait Interleaved: Transceive {
    /// Get N_i for this station S(i). N_i is the number of stations with
    /// data destined for S(i).
    fn get_n_i(&self) -> usize;

    /// Increment N_i for this station S(i) by 1. N_i is the number of stations
    /// with data destined for S(i).
    fn increment_n_i(&mut self);

    /// Get the global L value.
    fn get_global_l(&self) -> usize;

    /// Get next action to perform.
    fn pop_action(&mut self) -> Option<RoundAction>;

    /// Schedule action for next round.
    fn push_action(&mut self, action: RoundAction);

    /// Get the next round's starting time.
    fn get_next_round_t(&self) -> usize;

    /// Update next round's starting time.
    /// Allows for scheduling events for the next round.
    fn update_next_round_t(&mut self, start_time: usize);

    /// Receive the result of the current timeslice.
    fn recv_timeslice(&mut self) -> ReceiveCD;

    /// Return once timeslice reached.
    ///
    /// While waiting for timeslice, we want to "release" the CPU. This allows
    /// us to conserve energy by turning off the device, or performing another
    /// operation such as sleep or calculate digits of pi, if the user desires.
    fn wait_until_timeslice(&mut self, timeslice: usize);

    /// Calculate and update next round's starting time.
    fn update_next_round_start(&mut self, l: usize) {
        // l is the number of partitions of P.
        // l is therefore also the number of steps needed for this round.
        // Each step takes 2 timeslices (follower + leader actions).
        self.update_next_round_t(self.get_next_round_t() + (2 * l));
    }

    /// Facilitate a single step of an inner-round of Interleaved_Initialize().
    /// See section 4 of Nakano and Olariu.
    fn leader_round_step(&mut self, l: &mut usize) {
        // detect if timeslice state is NULL, SINGLE, or COLLISION
        match self.recv_timeslice() {
            ReceiveCD::Collision => { *l += 2; },
            ReceiveCD::Single(_, _) => { self.increment_n_i(); },
            ReceiveCD::None => (),
        }
        // transmit n_i and l, concluding step
        self.send_round_step(self.get_n_i(), l);
    }

    /// Facilitate the inner-round/inner-for-loop.
    ///
    /// timeslice: timeslice which inner-round starts
    /// l_i: l_i
    /// l_j: l_(i-1)
    fn leader_action(&mut self, timeslice: usize, l_j: usize, l_i: usize) {
        // TODO: for S(1), how to get conclusion info of previous round?
        self.wait_until_timeslice(timeslice);  // TODO: this probably prevents getting global_l?
        let mut global_l: usize = 0; // TODO: get l from previous step l_j (l_(i - 1))
                                 // (discard until message found for round l_j)
                                 // may need to fix above wait_until_timeslice() call <<<<<<<<<<<<<<<<<<
        let new_l_j: usize = global_l;
        let old_n_i: usize = self.get_n_i();

        for _ in (l_j + 1)..l_i {
            self.leader_round_step(&mut global_l);
        }

        // check if any single stations
        if self.get_n_i() < old_n_i {
            panic!("Error: leader's inner-round concluded with n_i < old_n_i.
                Possibly an overflow error.");
        }

        if global_l == new_l_j {
            // TODO if l doesn't change => no more uninitialized followers
        } else if global_l > new_l_j {
            // schedule leader action for next round
            self.push_action(
                RoundAction::Leader(
                    self.get_next_round_t() + new_l_j,
                    new_l_j,
                    global_l,
                )
            );
        } else {
            panic!("Error: leader's inner-round concluded with new_l_j < global_l.
                Possibly an overflow error.");
        }

        // TODO check if this is technically round conclusion
    }

    /// Handle leader's response, and update this follower's assignment status.
    ///
    /// Leader either assigned this follower an ID or partitioned.
    /// Either assign follower provided ID or randomly to new partition.
    fn follower_assignment(&mut self, n_i: usize, new_n_i: usize, l: usize) {
        // TODO
        if new_n_i > n_i {
            // TODO: new_n_i is ID assigned to this station for this leader
        } else if new_n_i == n_i {
            // TODO: randomly partition (50/50)
                // schedule next follower action for the partition that's
                // randomly joined
        } else {
            panic!("Error: follower received leader's conclusion step with
                new_n_i < n_i.")
        }
    }

    /// Follower action for one step the inner-round/inner-for-loop.
    ///
    /// The step occurs over timeslices 2j-1 and 2j, with followers transmitting
    /// at 2j-1 and the leader transmitting at 2j, as described in section 4 of
    /// Nakano and Olariu.
    ///
    /// timeslice: timeslice for step to participate in
    /// n_i: previous n_i
    fn follower_action(&mut self, timeslice: usize, n_i: usize) {
        // TODO
        // TODO: wait until 2j-2 (the timeslice before 2j-1) so we can transmit
        // during timeslice 2j-1
        self.wait_until_timeslice(timeslice);
        // TODO: timeslice 2j-1) transmit, claiming participation in P_j
        // TODO: timeslice 2j) leader -> (N_i, l) splits P_j OR assigns id N_i
            // if N_i sent by leader changes, take id
            // else, split P_j, calculate next participating timeslot
        match self.recv_timeslice() {
            ReceiveCD::Single(new_n_i, l) => { self.follower_assignment(n_i, new_n_i, l) },
            ReceiveCD::Collision => {
                panic!("Error: follower received collision instead of single
                    transmission from leader.");
            },
            ReceiveCD::None => {
                panic!("Error: follower received None instead of single
                    transmission from leader.");
            },
        }

        // TODO check if this is technically round conclusion
    }

    /// Action performed by all stations at the end of an outer-round.
    /// Update the next outer-round's starting timeslice, allowing stations to
    /// calculate when to schedule their actions for the following round.
    /// Update the global L value to determine if more rounds are necessary.
    fn round_conclusion_action(&mut self, timeslice: usize, n: usize, l: usize) {
        // TODO
        self.wait_until_timeslice(timeslice);
        // update values
        // calculate next rounds conclusion
        self.update_next_round_t(l);
        // TODO: does next rounds actions need to be calculated here? I think
        // they're done during other acitons (eg. new partitions know when to
        // participate next round since l value?)
    }

    /// Determine and perform next action
    fn handle_action(&mut self) {
        let action: RoundAction;
        match self.pop_action() {
            Some(a) => action = a,
            None => {
                // TODO
                // if no action and L >= 1, error!
                println!("Error: no action to perform, but L >= 1. There should
                    at least be a RoundConclusion action....");
                return;
            },
        }

        match action {
            RoundAction::Leader(timeslice, l_j, l_i) =>
                self.leader_action(timeslice, l_j, l_i),
            RoundAction::Follower(timeslice, n_i) =>
                self.follower_action(timeslice, n_i),
            RoundAction::RoundConclusion(timeslice, n_i, l) =>
                self.round_conclusion_action(timeslice, n_i, l),
        }
    }

    fn f(&mut self) {
        // TODO
        // iterate over action_queue, complete each action
        // in last timeslice of outer-round, all stations listen
        // rinse and repeat
        while self.get_global_l() >= 1 {
            self.handle_action();
        }
    }
}

// First impl attempt
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
    /// See section 4 of Nakano and Olariu.
    fn send_round_step(&mut self, round: usize) {
        let mut data: Vec<u8> = self.get_n_i().to_be_bytes().to_vec();
        data.append(&mut self.get_local_l().to_be_bytes().to_vec());
        self.sync_send(round, data);
    }

    /// Facilitate a single iteration of a round of Interleaved_Initialize().
    /// See section 4 of Nakano and Olariu.
    fn interleaved_leader_round_step(&mut self, round: usize) {
        // detect if timeslice state is NULL, SINGLE, or COLLISION
        match self.recv_timeslice() {
            ReceiveCD::Collision => self.handle_collision(),
            ReceiveCD::Single => self.handle_single(),
            ReceiveCD::None => (),
        }
        // transmit n_i and l
        self.send_round_step(round);
    }

    /// Participating stations' round step in leader's interleaved round.
    fn interleaved_round_step(&mut self, round: usize) {
        // TODO
        // timeslice 2j-1) transmit, claiming participation in P_j
        // timeslice 2j) leader -> (N_i, l) splits P_j OR assigns id N_i
            // if N_i sent by leader changes, take id
            // else, split P_j, calculate next participating timeslot
    }

    /// Facilitate this station's round of Interleaved_Initialize().
    ///
    /// For the given station, conduct a round of the inner for-loop of
    /// Interleaved_Initialize() from section 4 of Nakano and Olariu.
    fn interleaved_leader_round(&mut self) {
        let new_l_j: usize = 0;
        // TODO: get l from previous round l_j (l_i - 1) (discard until message found
        // for round l_j)

        for k in self.interleaved_round_range() {
            self.interleaved_leader_round_step(k);
        }

        self.set_l_i(self.get_local_l());
        self.set_l_j(new_l_j);
    }

    /// Interleaved initialization to determine transmission order.
    ///
    /// Performs the Interleaved_Initialize algorithm proposed in section 4 of
    /// Nakano and Olariu.
    fn interleaved_initialize(&mut self) {
        // TODO: state machine
        // while L >= 1
            // while timeslice != round terminating timelice
                // if timeslice == receiving/leading period
                    // interleaved_leader_round
                // if timeslice == a destination's receiving/leading period
                    // participate
                // mutually synchronize and sleep/twiddle thumbs
    }
}
