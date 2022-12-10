use crate::computer::Computer;

pub fn generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|line| {
            line.parse()
                .unwrap_or_else(|err| panic!("invalid number `{}`: `{}`", line, err))
        })
        .collect()
}

pub fn part_1(input: &[isize]) -> isize {
    let mut program = input.to_vec();
    program[1] = 12;
    program[2] = 2;
    Computer::new(program, || panic!("program should not read input")).run()[0]
}

pub fn part_2(input: &[isize]) -> isize {
    let target = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input.to_vec();
            program[1] = noun;
            program[2] = verb;
            let res = Computer::new(program, || panic!("program should not read input")).run()[0];

            if res == target {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!("no feasible solution")
}
