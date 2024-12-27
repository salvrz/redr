# Energy-Efficient Distributed Routing Algorithm

An energy-efficient routing algorithm, hinging on an energy-efficient initialization algorithm.
Based on [Koji Nakano and Stephen Olariu's Energy-Efficient Randomized Routing in Radio Networks](https://dl.acm.org/doi/pdf/10.1145/345848.345856).
The routing algorithm balances runtime and energy-efficiency by allowing the
device to sleep or run a user defined function while it does not need to
transmit/receive data during probabilistically assigned time slots.

## Definitions
I have tried to maintain the terminology used in Nakano and Olariu's work. That
said, I have adopted some new terms to help clarify some functionality. I will
document the new terminology I catch myself using here as best I can.
- Round — An iteration of the outer for-loop of the `Interleaved_Initialize(...)`
protocol (Nakano and Olariu 40).
    - Outer-round — Synonymous with round.
    - Inner-round — An iteration of the inner for-loop of the
    `Interleaved_Initialize(...)` protocol (Nakano and Olariu 40). I beleive
    there is enough work done during the inner-round to warrant this term.

## Citations
[Koji Nakano and Stephen Olariu's Energy-Efficient Randomized Routing in Radio Networks](https://dl.acm.org/doi/pdf/10.1145/345848.345856)
