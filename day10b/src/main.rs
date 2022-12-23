use std::{fs, io::{self, Write}};

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

fn print_pixel(state: &CpuState) {
    let pixel_pos = (state.cycle - 1) % 40;
    if (state.cycle - 1) % 40 == 0 {
        print!("\n");
    }
    if pixel_pos >= state.x - 1 && pixel_pos <= state.x + 1 {
        print!("#");
    } else {
        print!(" ");
    }
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let code: Vec<&str> = contents.lines().collect();

    let mut answer = 0;
    let mut completed = false;
    let mut state = CpuState {cycle: 1, ic: 0, arg: 0, x: 1, fsm: InstrState::Fetch};

    while completed == false {
        print_pixel(&state);
        (state, completed) = run_computation(state, &code, 1);
    }
}
