use crate::util::linalg::Vector2;

fn origin_in_triangle([a, b, c]: [Vector2<i64>; 3]) -> bool {
    // Rotate vector by 90°
    let rot = |u: Vector2<i64>| v![*u.y(), -*u.x()];

    // Sign of the angle between u and v
    let sign = |u: Vector2<i64>, v: Vector2<i64>| {
        let angle = rot(u).dot(v);
        angle / angle.abs()
    };

    let signs = [sign(b - a, -a), sign(c - b, -b), sign(a - c, -c)];
    signs.iter().all(|s| *s >= 0) || signs.iter().all(|s| *s <= 0)
}

pub fn solve() -> usize {
    include_str!("data/problem_102.txt")
        .lines()
        .map(|raw| {
            let coords: Vec<_> = raw
                .split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect();

            match *coords.as_slice() {
                [x1, y1, x2, y2, x3, y3] => [v![x1, y1], v![x2, y2], v![x3, y3]],
                _ => unreachable!(),
            }
        })
        .filter(|triangle| origin_in_triangle(*triangle))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_in_triangle() {
        assert!(origin_in_triangle([
            v![-340, 495],
            v![-153, -910],
            v![835, -947],
        ]));

        assert!(!origin_in_triangle([
            v![-175, 41],
            v![-421, -714],
            v![574, -645],
        ]));
    }
}
