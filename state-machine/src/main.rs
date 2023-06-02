use futures::stream::StreamExt;
// use state_machine_future::machine::Machine;
// use state_machine_future::machine::MachineBuilder;
// use state_machine_future::machine::RentToOwnMachine;
// use state_machine_future::transition::TransitionResult;
//use state_machine_future::RentToOwn;

use futures::Async;
use futures::Future;
use futures::Poll;
use state_machine_future::RentToOwn;

use state_machine_future::{StateMachine, StateMachineFuture}; // Define the states of the traffic light

//use state_machine_future::{RentToOwn, StateMachineFuture};

enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

// Define the events that trigger state transitions
enum TrafficLightEvent {
    TimerExpired,
}

// Create a struct to hold the traffic light's data
struct TrafficLightData {
    // ... additional data fields could be added here
}

// Implement the state machine logic using the `state_machine_future` crate
impl Machine for TrafficLightData {
    type State = TrafficLightState;
    type Event = TrafficLightEvent;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }

    fn dispatch(&mut self, event: Self::Event) -> TransitionResult<Self::State, Self::Event> {
        match (self.state(), event) {
            (TrafficLightState::Red, TrafficLightEvent::TimerExpired) => {
                // Transition from Red to Green
                self.state = TrafficLightState::Green;
                TransitionResult::ok(TrafficLightEvent::TimerExpired)
            }
            (TrafficLightState::Green, TrafficLightEvent::TimerExpired) => {
                // Transition from Green to Yellow
                self.state = TrafficLightState::Yellow;
                TransitionResult::ok(TrafficLightEvent::TimerExpired)
            }
            (TrafficLightState::Yellow, TrafficLightEvent::TimerExpired) => {
                // Transition from Yellow to Red
                self.state = TrafficLightState::Red;
                TransitionResult::ok(TrafficLightEvent::TimerExpired)
            }
            _ => TransitionResult::error(),
        }
    }
}

fn main() {
    // Create a new TrafficLightData instance
    let mut traffic_light = TrafficLightData {
        state: TrafficLightState::Red,
    };

    // Create a machine builder and register the transitions
    let mut builder = MachineBuilder::new(&mut traffic_light);
    builder.add_transition(
        TrafficLightState::Red,
        TrafficLightEvent::TimerExpired,
        TrafficLightState::Green,
    );
    builder.add_transition(
        TrafficLightState::Green,
        TrafficLightEvent::TimerExpired,
        TrafficLightState::Yellow,
    );
    builder.add_transition(
        TrafficLightState::Yellow,
        TrafficLightEvent::TimerExpired,
        TrafficLightState::Red,
    );
    let machine = builder.build();

    // Create a stream of events
    let events = futures::stream::repeat(TrafficLightEvent::TimerExpired).take(10);

    // Process the events using the state machine
    let mut future = machine.process_stream(events);

    // Run the state machine until completion
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { while let Some(_) = future.next().await {} });
}

// use state_machine_future::{StateMachine, StateMachineFuture};

// #[derive(StateMachine)]
// struct MyMachine {
//     state: State,
// }

// #[derive(State)]
// enum State {
//     Waiting,
//     Filling,
//     Done,
// }

// impl MyMachine {
//     fn new() -> Self {
//         Self {
//             state: State::Waiting,
//         }
//     }

//     fn fill(&mut self) -> StateMachineFuture<()> {
//         self.state = State::Filling;
//         async {}
//     }

//     fn done(&mut self) -> StateMachineFuture<()> {
//         self.state = State::Done;
//         async {}
//     }
// }

// fn main() {
//     let mut machine = MyMachine::new();

//     let future = machine.fill();
//     future.await;
//     assert_eq!(machine.state, State::Filling);

//     let future = machine.done();
//     future.await;
//     assert_eq!(machine.state, State::Done);
// }
