use crate::computer::Computer;

pub use crate::day2::generator;

pub fn part_1(program: &[isize]) -> isize {
    let mut input = Some(1);

    Computer::new(program, || {
        input.take().expect("program reads inputs twice")
    })
    .last()
    .expect("computer didn't output")
}

pub fn part_2(program: &[isize]) -> isize {
    let mut input = Some(5);

    Computer::new(program, || {
        input.take().expect("program reads inputs twice")
    })
    .last()
    .expect("computer didn't output")
}
