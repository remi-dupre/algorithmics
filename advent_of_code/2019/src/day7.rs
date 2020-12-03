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

#[aoc(day7, part2)]
pub fn part2(program: &[isize]) -> isize {
    (5..10)
        .permutations(5)
        .filter_map(|phase| {
            let computers: Vec<_> = (0..5)
                .map(|_| {
                    Rc::new(RefCell::new(Computer::new(
                        program,
                        Box::new(|| -> isize { todo!() }) as Box<dyn FnMut() -> isize>,
                    )))
                })
                .collect();

            // Input function : read from previous computer

            let inputs: Vec<Rc<RefCell<VecDeque<isize>>>> = phase
                .into_iter()
                .map(|phase| Rc::new(RefCell::new(vec![phase].into())))
                .collect();

            let read_input = inputs.iter().enumerate().map(|(i, input)| {
                let input = input.clone();
                let prev_computer = computers[(i + 4) % 5].clone();

                Box::new(move || {
                    if let Some(val) = input.borrow_mut().pop_front() {
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

            #[allow(unused_must_use)]
            for (i, new_input) in read_input.enumerate() {
                computers[i].borrow_mut().replace_input(new_input);
            }

            // Manually run the loop
            inputs[0].borrow_mut().push_back(0);

            iter::from_fn(move || {
                let val = computers[4].borrow_mut().next();
                inputs[0].borrow_mut().push_back(val?);
                val
            })
            .last()
        })
        .max()
        .expect("no filter outputted anything")
}
