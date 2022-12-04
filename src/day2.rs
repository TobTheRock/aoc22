#[derive(PartialEq, Debug)]
enum Strategy {
    Win,
    Loose,
    Draw,
}

impl From<&str> for Strategy {
    fn from(s: &str) -> Self {
        match s {
            "X" => Strategy::Loose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            &_ => panic!("Wrong string"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Action {
    Rock,
    Paper,
    Scissor,
}

impl Action {
    pub fn from_attack(s: &str) -> Action {
        match s {
            "A" => Action::Rock,
            "B" => Action::Paper,
            "C" => Action::Scissor,
            &_ => panic!("Wrong string"),
        }
    }

    pub fn from_response(s: &str) -> Action {
        match s {
            "X" => Action::Rock,
            "Y" => Action::Paper,
            "Z" => Action::Scissor,
            &_ => panic!("Wrong string"),
        }
    }

    pub fn from_strategy(action: &Action, strategy: &Strategy) -> Action {
        match strategy {
            Strategy::Win => action.winner(),
            Strategy::Loose => action.looser(),
            Strategy::Draw => action.clone(),
        }
    }

    pub fn wins(&self, a: &Action) -> bool {
        match self {
            Action::Rock => a == &Action::Scissor,
            Action::Paper => a == &Action::Rock,
            Action::Scissor => a == &Action::Paper,
        }
    }

    pub fn winner(&self) -> Action {
        match self {
            Action::Rock => Action::Paper,
            Action::Paper => Action::Scissor,
            Action::Scissor => Action::Rock,
        }
    }

    pub fn looser(&self) -> Action {
        match self {
            Action::Rock => Action::Scissor,
            Action::Paper => Action::Rock,
            Action::Scissor => Action::Paper,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Round {
    attack: Action,
    response: Action,
}

impl Round {
    fn from_my_strategy(line: &str) -> Self {
        let mut actions = line.split_whitespace();
        Self {
            attack: Action::from_attack(actions.next().unwrap()),
            response: Action::from_response(actions.next().unwrap()),
        }
    }

    fn from_actual_strategy(line: &str) -> Self {
        let mut actions = line.split_whitespace();
        let attack = Action::from_attack(actions.next().unwrap());
        let strategy = actions.next().unwrap().into();
        let response = Action::from_strategy(&attack, &strategy);

        Self { attack, response }
    }

    pub fn score(&self) -> u64 {
        self.score_win() + self.score_response()
    }

    fn score_response(&self) -> u64 {
        match self.response {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissor => 3,
        }
    }

    fn score_win(&self) -> u64 {
        if self.attack == self.response {
            3
        } else if self.response.wins(&self.attack) {
            6
        } else {
            0
        }
    }
}

pub fn main_day2() {
    println!("----- DAY 2 --------");

    let test_data = std::fs::read_to_string("./data/day2.txt").unwrap();

    let rounds_first_try = test_data.lines().map(|l| Round::from_my_strategy(l));
    let total_score: u64 = rounds_first_try.map(|r| r.score()).sum();
    println!("Total score {}", total_score);

    let rounds_next_try = test_data.lines().map(|l| Round::from_actual_strategy(l));
    let total_score: u64 = rounds_next_try.map(|r| r.score()).sum();
    println!("Total score {}", total_score);
}
