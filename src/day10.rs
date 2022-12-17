struct CPU {
    reg_x: i32,
    history: Vec<i32>,
}

impl CPU {
    fn new() -> Self {
        CPU {
            reg_x: 1,
            history: Default::default(),
        }
    }
    fn cycle(&mut self) {
        self.history.push(self.reg_x)
    }
}

#[derive(Debug, Default)]
struct CTR {
    hpos: usize,
    sprite_pos: i32,
}

impl CTR {
    fn cycle(&mut self) {
        const SPRITE_LEN: i32 = 3;
        if (self.sprite_pos..self.sprite_pos + SPRITE_LEN)
            .contains(&(self.hpos).try_into().unwrap())
        {
            print!("#");
        } else {
            print!(".");
        }

        self.hpos = (self.hpos + 1) % 40;

        if self.hpos == 0 {
            print!("\n");
        }
    }
}

struct VideoSystem {
    cpu: CPU,
    ctr: CTR,
}

impl VideoSystem {
    fn new() -> Self {
        VideoSystem {
            cpu: CPU::new(),
            ctr: CTR::default(),
        }
    }

    fn cycle(&mut self) {
        self.cpu.cycle();
        self.ctr.cycle();
    }
    fn add(&mut self, num: i32) {
        self.cycle();
        self.cycle();
        self.cpu.reg_x = self.cpu.reg_x + num;
        self.ctr.sprite_pos = self.cpu.reg_x - 1;
    }
}

pub fn main_day10() {
    println!("----- DAY 10 --------");

    let test_data = std::fs::read_to_string("./data/day10.txt").unwrap();

    let mut vs = VideoSystem::new();

    for l in test_data.lines() {
        match l {
            "noop" => vs.cycle(),
            s => {
                let num: i32 = s
                    .split_whitespace()
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                vs.add(num);
            }
        }
    }

    let signal_strength: i32 = [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&i| i as i32 * vs.cpu.history[i - 1])
        .sum();
    println!("Signal strength: {}", signal_strength)
}
