#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::{ensure, Context, Result};
use std::{fs, str::FromStr};

static INPUT_FILE: &str = "input.txt";

const NUM_STACKS: usize = 9;

struct Instruction {
    num_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    // The instruction specification lines have the form
    //     move 13 from 8 to 7
    // All we need to capture are the three numbers, which happen to
    // be in the odd positions in the input line. The `filter` statement
    // below extracts those three items from the list, which we can
    // then parse into `usize` in the `map` statement immediately after.
    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<usize> = s
            .split_ascii_whitespace()
            .enumerate()
            .filter(|(pos, _)| pos % 2 == 1)
            .map(|(_, val)| {
                val.parse::<usize>()
                    .with_context(|| format!("Couldn't parse '{val}' to an int"))
            })
            .collect::<Result<Vec<_>>>()?;
        ensure!(
            parts.len() == 3,
            "Line '{s}' didn't have the appropriate format"
        );
        ensure!(
            parts[1] != parts[2],
            "The source {} and destination {} stacks must be different",
            parts[1],
            parts[2],
        );
        Ok(Self {
            num_to_move: parts[0],
            from_stack: parts[1],
            to_stack: parts[2],
        })
    }
}

#[derive(Default, Debug)]
struct Stacks {
    stacks: [Vec<char>; NUM_STACKS],
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    // You probably want to use `s.lines()` to create an iterator over the lines (one per stack).
    // Then for each line:
    //   (a) extract the number at the front as the stack number
    //   (b) extract the following characters as the stack contents
    // The function `split_ascii_whitespace()` should prove useful.
    // Note that the stack numbers start at 1 and you'll need the indices
    // in `Stacks::stack` to start at 0.
    fn from_str(s: &str) -> Result<Self> {
        todo!("Implement parsing a `Stacks` value from the input")
    }
}

impl Stacks {
    // Two solutions:
    //   - One that allocates on the heap (using `collect()`)
    //   - One that avoids the additional heap allocation by using `split_at_mut()`
    //   - Discussion of pros and cons of each

    // - Give them the version that doesn't compile
    // - Have them explain why it doesn't work
    // - Have them fix it through re-ordering of existing lines, with explanation
    // - Have them fix it using `split_at_mut()` with explanation
    // - GitHub CoPilot solved the problem in a fairly reasonable way similar to
    //   the `get_two_mut()` solution, but without extracting it as a function.

    // Question? Should I remove the `collect()` call from the version they get?
    // If I do, they will need to move the `destination` declaration _and_ introduce
    // a `collect()` call in the first version.
    fn apply(
        mut self,
        Instruction {
            num_to_move,
            from_stack,
            to_stack,
        }: Instruction,
    ) -> Result<Self> {
        // We know from the parsing check that `from_stack` and `to_stack` are different.
        let (source, destination) =
            Self::get_two_mut(&mut self.stacks, from_stack - 1, to_stack - 1);

        ensure!(
            num_to_move <= source.len(),
            "We tried to take {num_to_move} items from {source:?}"
        );

        let crates_to_move = source.drain((source.len() - num_to_move)..);
        destination.extend(crates_to_move);
        Ok(self)
    }

    // The method `get_many_mut()` (https://doc.rust-lang.org/stable/std/primitive.slice.html#method.get_many_mut)
    // would do this for us, but it's still in nightly. This will be renamed to `get_disjoint_mut` with stabilization.
    fn get_two_mut<T>(source: &mut [T], index1: usize, index2: usize) -> (&mut T, &mut T) {
        assert!(index1 != index2);
        if index1 < index2 {
            let (left, right) = source.split_at_mut(index2);
            (&mut left[index1], &mut right[0])
        } else {
            let (left, right) = source.split_at_mut(index1);
            (&mut right[0], &mut left[index2])
        }
    }

    fn tops_string(&self) -> Result<String> {
        todo!()
    }
}

fn main() -> anyhow::Result<()> {
    let contents = fs::read_to_string(INPUT_FILE)
        .with_context(|| format!("Failed to open file '{INPUT_FILE}'"))?;

    // This splits the input into two parts, the text before the blank
    // line (`stack_config`) and the part after the blank line (`instructions`).
    let (stack_config, instructions) = contents
        .split_once("\n\n")
        .context("There was no blank line in the input")?;

    // The `.parse()` call actually calls the appropriate `from_str()`, which
    // in this case is in the `impl FromStr for Stacks` block.
    let stacks: Stacks = stack_config.parse()?;

    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>>>()?;

    let final_state = stacks.apply_instructions(instructions); //  instructions.into_iter().try_fold(stacks, Stacks::apply)?;

    println!("The top of the stacks is {}", final_state.tops_string()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        // The `\` at the end of the line escapes the newline and all following whitespace.
        let input = "1 Z N\n\
                           2 M C D\n\
                           3 P";
        println!("{input}");
        #[allow(clippy::unwrap_used)]
        let stacks: Stacks = input.parse().unwrap();
        assert_eq!(2, stacks.stacks[0].len());
        assert_eq!(vec!['Z', 'N'], stacks.stacks[0]);
        assert_eq!(3, stacks.stacks[1].len());
        assert_eq!(vec!['M', 'C', 'D'], stacks.stacks[1]);
        assert_eq!(1, stacks.stacks[2].len());
        assert_eq!(vec!['P'], stacks.stacks[2]);
    }
}
