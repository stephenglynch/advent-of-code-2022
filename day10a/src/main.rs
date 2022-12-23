use std::fs;

struct CpuState {
    cycle: i32,
    ic: usize,
    arg: i32,
    x: i32,
    fsm: InstrState
}

enum InstrState {
    Fetch,
    NoOp,
    AddX0,
    AddX1
}

fn run_computation(mut state: CpuState, code: &[&str], cycles_to_run: i32) -> (CpuState, bool) {
    let end_cycle = state.cycle + cycles_to_run;
    while state.cycle < end_cycle {

        // Check if there are still instructions otherwise terminate early
        if state.ic >= code.len() {
            return (state, true);
        }

        match state.fsm {
            InstrState::Fetch => {
                let mut instr = code[state.ic].split_whitespace();
                let opword = instr.next().unwrap();
                if opword == "noop" {
                    state.fsm = InstrState::NoOp;
                } else {
                    state.fsm = InstrState::AddX0;
                    state.arg = instr.next().unwrap().parse().unwrap();
                }
                state.ic += 1;
            }
            InstrState::NoOp => {
                state.cycle += 1;
                state.fsm = InstrState::Fetch;
            }
            InstrState::AddX0 => {
                state.cycle += 1;
                state.fsm = InstrState::AddX1;
            }
            InstrState::AddX1 => {
                state.cycle += 1;
                state.x += state.arg;
                state.fsm = InstrState::Fetch;
            }
        }
    }

    // Finished issued cycles but more code left
    return (state, false);
}

fn signal_strength(state: &CpuState) -> i32 {
    state.cycle * state.x
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let code: Vec<&str> = contents.lines().collect();

    let mut answer = 0;
    let mut completed = false;
    let mut state = CpuState {cycle: 0, ic: 0, arg: 0, x: 1, fsm: InstrState::NoOp};
    (state, completed) = run_computation(state, &code, 20);
    answer += signal_strength(&state);
    println!("signal = {}", answer);

    while completed == false {
        (state, completed) = run_computation(state, &code, 40);
        if completed != true {
            let signal = signal_strength(&state);
            println!("signal = {}", signal);
            answer += signal;
        }
    }

    println!("answer = {}", answer);
}
