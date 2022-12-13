use std::{
    cmp::max,
    collections::HashSet,
    ops::{Add, Sub},
};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i64, i64);

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Point {
    fn abs(&self) -> i64 {
        max(self.0.abs(), self.1.abs())
    }

    fn signum(&self) -> Point {
        Point(self.0.signum(), self.1.signum())
    }
}

#[derive(Default)]
struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(len: usize) -> Self {
        Self {
            knots: vec![Default::default(); len],
        }
    }

    fn simultate(&mut self, test_data: &str) {
        let mut visited = HashSet::<Point>::new();
        visited.insert(*self.knots.last().unwrap());

        for l in test_data.lines() {
            let mut split = l.split_whitespace();
            let dir = split.next().unwrap();
            let n: usize = split.next().unwrap().parse().unwrap();

            for _ in 0..n {
                let d = match dir {
                    "U" => Point(0, 1),
                    "D" => Point(0, -1),
                    "R" => Point(1, 0),
                    "L" => Point(-1, 0),
                    _ => panic!("Invalid direction"),
                };
                self.knots[0] = self.knots[0] + d;

                self.knots = self
                    .knots
                    .iter()
                    .take(1)
                    .cloned()
                    .chain(self.knots.windows(2).map(|w| {
                        let diff = w[0] - w[1];
                        if diff.abs() <= 1 {
                            return w[1];
                        }

                        let d2 = diff.signum();
                        let k = w[1] + d2;
                        k
                    }))
                    .collect();

                visited.insert(*self.knots.last().unwrap());
            }
        }
        println!("Sites visited {}", visited.len());
    }
}

pub fn main_day9() {
    println!("----- DAY 9 --------");

    let test_data = std::fs::read_to_string("./data/day9.txt").unwrap();
    let mut ropeA = Rope::new(2);
    ropeA.simultate(&test_data);

    let mut ropeB = Rope::new(10);
    ropeB.simultate(&test_data);
}
