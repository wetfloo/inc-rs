use std::mem;

fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s)
}

// Default derive allows us to take values from a Vec
#[derive(Clone, Debug, PartialEq, Default)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Trinity<T> {
    fn rotate(&mut self) {
        // Yay, no cloning!
        mem::swap(&mut self.a, &mut self.b);
        mem::swap(&mut self.a, &mut self.c);
    }
}

impl<T: PartialEq> Trinity<T> {
    // Rotation config could be improved by pre-validating that it's correct
    fn try_solve(&mut self, expected: &Trinity<T>, rotate_times: u32) -> SolveResult {
        for _ in 0..rotate_times {
            if self == expected {
                return SolveResult::Solved;
            }
            self.rotate();
        }

        // Give up after rotating enough times
        SolveResult::Unsolved
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: PartialEq> Solver<T> {
    fn resolve(&mut self) {
        // Don't really know ahead of time how many items will need to be here
        let mut solved_indices: Vec<usize> = Vec::new();
        // Find all indices that could are resolved after rotating enough times
        for (index, trinity) in self.unsolved.iter_mut().enumerate() {
            let solve_result = trinity.try_solve(&self.expected, 3);
            if let SolveResult::Solved = solve_result {
                solved_indices.push(index);
            }
        }

        // Now that indices are ready, pop off items from the original Vec.
        // Go backwards, because deleting from the end is better for array lists
        for &index in solved_indices.iter().rev() {
            self.unsolved.remove(index);
        }
    }
}

#[derive(PartialEq, Eq)]
enum SolveResult {
    Solved,
    Unsolved,
}
