use std::ops::Range;

use anyhow::{Context, Result};
use fxhash::FxHashSet;

type Coord = (i32, i32);

pub struct Sensor {
    pos: Coord,
    beacon: Coord,
}

// A "curve" is a line with a slope of 1 (right curve) or -1 (left curve), they are represented by
// there value on the axis of y coordinate 0.
//
//     \          /
//      \r      l/
// 0 ----\------/-----
//        \    /
//         \  /
//          \/ res
//          /\
fn intersect(curve_r: i32, curve_l: i32) -> Option<Coord> {
    if (curve_l - curve_r).abs() % 2 != 0 {
        return None;
    }

    Some(((curve_r + curve_l) / 2, (curve_l - curve_r) / 2))
}

fn point_dist(p1: Coord, p2: Coord) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

impl Sensor {
    fn radius(&self) -> i32 {
        point_dist(self.pos, self.beacon)
    }

    fn contains(&self, pt: Coord) -> bool {
        point_dist(self.pos, pt) <= self.radius()
    }

    fn intesect_with_horizontal_line(&self, axis_y: i32) -> Option<Range<i32>> {
        let y_diff = (self.pos.1 - axis_y).abs();
        let radius = self.radius();
        let half_width = radius - y_diff;

        if half_width < 0 {
            return None;
        }

        let res = Range {
            start: self.pos.0 - half_width,
            end: self.pos.0 + half_width + 1,
        };

        Some(res)
    }

    /// Get a feasible right and left curve that separate two rombus if they are separated by a
    /// distance of exactly 1.
    fn curves_rl(&self, other: &Self) -> Option<(i32, i32)> {
        if point_dist(self.pos, other.pos) != self.radius() + other.radius() + 2 {
            return None;
        }

        let (x, y) = {
            if self.pos < other.pos {
                (self.pos.0 + self.radius() + 1, self.pos.1)
            } else {
                (other.pos.0 + self.radius() + 1, other.pos.1)
            }
        };

        Some((x - y, x + y))
    }
}

/// Count the number of values covered by input list of ranges
fn count_ranges_span(mut ranges: Vec<Range<i32>>) -> i32 {
    ranges.sort_unstable_by_key(|rng| rng.start);
    let mut res = 0;
    let mut last_end = i32::MIN;

    for range in ranges {
        if range.end > last_end {
            res += range.end - std::cmp::max(last_end, range.start);
            last_end = range.end;
        }
    }

    res
}

pub fn parse(input: &str) -> Result<Vec<Sensor>> {
    input
        .lines()
        .map(|line| {
            let line = line
                .strip_prefix("Sensor at x=")
                .context("missing line prefix")?;

            let (pos_x, line) = line.split_once(", y=").context("missing pos separator")?;

            let (pos_y, line) = line
                .split_once(": closest beacon is at x=")
                .context("missing beacon prefix")?;

            let (beac_x, beac_y) = line
                .split_once(", y=")
                .context("missing beacon separator")?;

            let pos_x = pos_x.parse().context("invalid 'x' pos")?;
            let pos_y = pos_y.parse().context("invalid 'y' pos")?;
            let beac_x = beac_x.parse().context("invalid 'x' beacon")?;
            let beac_y = beac_y.parse().context("invalid 'y' beacon")?;

            Ok(Sensor {
                pos: (pos_x, pos_y),
                beacon: (beac_x, beac_y),
            })
        })
        .collect()
}

pub fn part1(sensors: &[Sensor]) -> i32 {
    const ROW_Y: i32 = 2_000_000;

    let intersect_ranges: Vec<_> = sensors
        .iter()
        .filter_map(|sensor| sensor.intesect_with_horizontal_line(ROW_Y))
        .collect();

    let ranges_count = count_ranges_span(intersect_ranges);

    let beacons_on_line: FxHashSet<_> = (sensors.iter())
        .filter(|sensor| sensor.beacon.1 == ROW_Y)
        .map(|sensor| sensor.beacon.0)
        .collect();

    ranges_count - beacons_on_line.len() as i32
}

pub fn part2(sensors: &[Sensor]) -> Option<i64> {
    const MAX: i32 = 4_000_000;

    let (curves_l, curves_r): (FxHashSet<_>, FxHashSet<_>) = sensors
        .iter()
        .flat_map(|s1| sensors.iter().filter_map(move |s2| s1.curves_rl(s2)))
        .unzip();

    curves_l
        .iter()
        .flat_map(|cl| curves_r.iter().filter_map(move |cr| intersect(*cl, *cr)))
        .filter(|pos| !sensors.iter().any(|sensor| sensor.contains(*pos)))
        .map(|(x, y)| i64::from(x) * i64::from(MAX) + i64::from(y))
        .next()
}
