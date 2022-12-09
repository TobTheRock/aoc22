struct Assignment {
    start: u64,
    end: u64,
}

impl Assignment {
    fn fully_contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    fn overlaps(&self, other: &Assignment) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

impl From<&str> for Assignment {
    fn from(s: &str) -> Self {
        let mut values = s.split('-').map(|v| v.trim().parse::<u64>().unwrap());

        Self {
            start: values.next().unwrap(),
            end: values.next().unwrap(),
        }
    }
}

struct AssignmentPair {
    pub a: Assignment,
    pub b: Assignment,
}

impl From<&&str> for AssignmentPair {
    fn from(s: &&str) -> Self {
        let mut assignments = s.split(',').map(Assignment::from);
        Self {
            a: assignments.next().unwrap(),
            b: assignments.next().unwrap(),
        }
    }
}

fn assignments_fully_contain_each_other(pair: &AssignmentPair) -> bool {
    pair.a.fully_contains(&pair.b) || pair.b.fully_contains(&pair.a)
}
fn assignments_overlap(pair: &AssignmentPair) -> bool {
    pair.a.overlaps(&pair.b) || pair.b.overlaps(&pair.a)
}

pub fn main_day4() {
    println!("----- DAY 4 --------");
    let test_data = std::fs::read_to_string("./data/day4.txt").unwrap();

    let n_fully_contains: usize = test_data
        .lines()
        .filter(|l| assignments_fully_contain_each_other(&l.into()))
        .count();
    println!("Fully contains {}", n_fully_contains);

    let n_overlaps: usize = test_data
        .lines()
        .filter(|l| assignments_overlap(&l.into()))
        .count();
    println!("Overlaps {}", n_overlaps);
}
