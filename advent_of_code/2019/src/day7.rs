use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter;
use std::rc::Rc;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::computer::Computer;
use crate::day2;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<isize> {
    day2::input_generator(input)
}

#[allow(clippy::many_single_char_names, clippy::clippy::match_ref_pats)]
#[aoc(day7, part1)]
pub fn part1(program: &[isize]) -> isize {
    fn run_circuit(program: &[isize], phase: &[isize], init: isize) -> Option<isize> {
        phase.iter().fold(Some(init), |snd, &fst| {
            let mut input = vec![fst, snd.expect("no input for next amplifier")].into_iter();
            Computer::new(program, move || input.next().expect("end of input")).last()
        })
    }

    (0..5)
        .permutations(5)
        .map(|phase| run_circuit(program, &phase, 0).expect("no output"))
        .max()
        .unwrap()
}

#[allow(unused_must_use)]
#[aoc(day7, part2)]
pub fn part2(program: &[isize]) -> isize {
    (5..10)
        .permutations(5)
        .filter_map(|phase| {
            let first_buffer: RefCell<VecDeque<_>> = RefCell::new(vec![phase[0], 0].into());

            let computers: Vec<_> = (0..5)
                .map(|_| {
                    Rc::new(RefCell::new(Computer::new(
                        program,
                        Box::new(|| -> isize { unreachable!("will be replaced") })
                            as Box<dyn FnMut() -> isize>,
                    )))
                })
                .collect();

            // Input function for [0]: read from buffer

            computers[0].borrow_mut().replace_input({
                let first_buffer = &first_buffer;
                Box::new(move || {
                    first_buffer
                        .borrow_mut()
                        .pop_front()
                        .expect("first computer reached end of input")
                }) as _
            });

            // Input function for [1..4]: read from previous computer

            for (i, phase) in phase.iter().enumerate().skip(1) {
                let mut initial = Some(*phase);
                let prev_computer = computers[(i + 4) % 5].clone();

                computers[i].borrow_mut().replace_input({
                    Box::new(move || {
                        if let Some(val) = initial.take() {
                            val
                        } else if i != 0 {
                            prev_computer
                                .borrow_mut()
                                .next()
                                .expect("no output from previous computer")
                        } else {
                            panic!("first computer has no input left")
                        }
                    }) as _
                });
            }

            // Manually run the loop

            iter::from_fn(|| {
                let val = computers[4].borrow_mut().next();
                first_buffer.borrow_mut().push_back(val?);
                val
            })
            .last()
        })
        .max()
        .expect("no filter outputted anything")
}
