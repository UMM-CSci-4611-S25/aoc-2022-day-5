use std::{fs, str::FromStr};

static INPUT_FILE: &str = "input.txt";

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE).expect(&format!("Failed to open file '{INPUT_FILE}'"));

    let (stack_config, instructions) = contents
        .split_once("\n\n")
        .expect("There was no blank line in the input");

    let stacks: Stacks = stack_config
        .parse()
        .expect("Failed to parse stack configuration");

    let instructions: CraneInstructions = instructions
        .parse()
        .expect("Failed to parse crane instructions");

    let final_state = stacks
        .apply_instructions(&instructions)
        .expect("Applying an instruction set failed");

    println!(
        "The top of the stacks is {}",
        final_state
            .tops_string()
            .expect("Tried to take the top of an empty stack")
    );
}

#[derive(Debug)]
enum ParseError {
    // Add different variants as you discover different kinds of parsing errors.
    // This could include things like too many stacks, too many characters in a stack, etc.
}

const NUM_STACKS: usize = 9;

#[derive(Debug, Default)]
struct Stacks {
    stacks: [Stack; NUM_STACKS],
}

#[derive(Debug)]
enum CraneError {
    // Add different variants as you discover different kinds of errors.
    // This could include things like trying to move from an empty stack,
    // trying to get the top of an empty stack, etc.
}

impl Stacks {
    /// Perform each of these instructions in order on the set of stacks
    /// in `self`. Return the new set of stacks, or a `CraneError` if
    /// any of the instructions are invalid.
    fn apply_instructions(&self, instructions: &CraneInstructions) -> Result<Self, CraneError> {
        todo!()
    }

    /// Return a string containing the top character of each stack in order.
    /// The stacks should all be non-empty; if any is empty return a `CraneError`.
    fn tops_string(&self) -> Result<String, CraneError> {
        todo!()
    }
}

impl FromStr for Stacks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct Stack {
    stack: Vec<char>,
}

impl FromStr for Stack {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct CraneInstruction {
    num_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for CraneInstruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct CraneInstructions {
    instructions: Vec<CraneInstruction>,
}

impl FromStr for CraneInstructions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
