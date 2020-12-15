pub fn part_1(input: &str) -> u32 {
    let target: u32 = input.lines().next().unwrap().parse().unwrap();
    let busses = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .filter(|bus| *bus != "x")
        .map(|bus| bus.parse::<u32>().unwrap());

    let (wait, bus) = busses
        .map(|bus| {
            let next_stop = bus * ((target + bus - 1) / bus);
            (next_stop - target, bus)
        })
        .min()
        .unwrap();

    wait * bus
}
