use regex::Regex;

#[derive(Clone, Default)]
struct ContainerStack {
    containers: Vec<char>,
}

#[derive(Clone)]
struct ContainerShip {
    stacks: Vec<ContainerStack>,
}
impl ContainerShip {
    fn new(size: usize) -> ContainerShip {
        ContainerShip {
            stacks: vec![Default::default(); size],
        }
    }

    fn push_to_stack(&mut self, stack_id: usize, item: char) {
        self.stacks[stack_id - 1].containers.push(item);
    }

    fn pop_from_stack(&mut self, stack_id: usize) -> char {
        self.stacks[stack_id - 1].containers.pop().unwrap()
    }

    fn peek_top(&self) -> impl Iterator<Item = &char> {
        self.stacks.iter().map(|s| s.containers.last().unwrap())
    }
}

impl From<&str> for ContainerShip {
    fn from(data: &str) -> Self {
        let mut lines = data.lines().map(|l| l.chars()).rev();
        let indices: Vec<(usize, usize)> = lines
            .next()
            .unwrap()
            .enumerate()
            .filter(|(_, c)| c.is_numeric())
            .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
            .collect();

        let mut ship = ContainerShip::new(indices.len());

        for (index_pos, index) in indices {
            for mut line in lines.clone() {
                if let Some(c) = line.nth(index_pos).filter(|c| c.is_alphabetic()) {
                    ship.push_to_stack(index, c);
                }
            }
        }

        ship
    }
}

trait Command {
    fn execute(&mut self, ship: &mut ContainerShip);
    fn execute_new(&mut self, ship: &mut ContainerShip);
}

struct MoveCommand {
    n_containers: usize,
    from: usize,
    to: usize,
}

impl Command for MoveCommand {
    fn execute(&mut self, ship: &mut ContainerShip) {
        for _ in 0..self.n_containers {
            let c = ship.pop_from_stack(self.from);
            ship.push_to_stack(self.to, c);
        }
    }
    fn execute_new(&mut self, ship: &mut ContainerShip) {
        let mut chars: Vec<char> = (0..self.n_containers)
            .map(|_| ship.pop_from_stack(self.from))
            .collect();

        chars.reverse();
        for c in chars {
            ship.push_to_stack(self.to, c);
        }
    }
}

impl From<&str> for MoveCommand {
    fn from(s: &str) -> Self {
        let reg = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let cap = reg.captures(s).unwrap();
        let mut val = cap
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse().unwrap());
        MoveCommand {
            n_containers: val.next().unwrap(),
            from: val.next().unwrap(),
            to: val.next().unwrap(),
        }
    }
}

struct CommandQueue {
    cmds: Vec<MoveCommand>,
}

impl Command for CommandQueue {
    fn execute(&mut self, ship: &mut ContainerShip) {
        for cmd in &mut self.cmds {
            cmd.execute(ship);
        }
    }

    fn execute_new(&mut self, ship: &mut ContainerShip) {
        for cmd in &mut self.cmds {
            cmd.execute_new(ship);
        }
    }
}

impl From<&str> for CommandQueue {
    fn from(s: &str) -> Self {
        let cmds: Vec<MoveCommand> = s.lines().map(MoveCommand::from).collect();

        Self { cmds }
    }
}

pub fn main_day5() {
    println!("----- DAY 5 --------");
    let file = std::fs::read_to_string("./data/day5.txt").unwrap();
    let mut test_data = file.split("\n\n");

    let mut ship: ContainerShip = test_data.next().unwrap().into();
    let mut ship2 = ship.clone();
    let mut cmds: CommandQueue = test_data.next().unwrap().into();
    cmds.execute(&mut ship);
    println!(
        "Top items {:?}",
        ship.peek_top()
            .fold(String::new(), |acc, c| acc + &c.to_string())
    );

    cmds.execute_new(&mut ship2);
    println!(
        "Top items {:?}",
        ship2
            .peek_top()
            .fold(String::new(), |acc, c| acc + &c.to_string())
    );
}
