#[derive(PartialEq, Clone, Copy, Debug)]
struct Item(char);

impl Item {
    fn priority(&self) -> u64 {
        if self.0.is_ascii_uppercase() {
            const ASCII_OFFSET: u8 = b'A';
            const MIN_POINTS: u8 = 27;
            (self.0 as u8 - ASCII_OFFSET + MIN_POINTS).into()
        } else {
            const ASCII_OFFSET: u8 = b'a';
            const MIN_POINTS: u8 = 1;
            (self.0 as u8 - ASCII_OFFSET + MIN_POINTS).into()
        }
    }
}

struct Compartment {
    pub items: Vec<Item>,
}

impl From<&str> for Compartment {
    fn from(s: &str) -> Self {
        Self {
            items: s.chars().map(Item).collect(),
        }
    }
}

struct Backpack {
    compartment_a: Compartment,
    compartment_b: Compartment,
}

impl Backpack {
    pub fn duplicate_item(&self) -> Item {
        *self
            .compartment_a
            .items
            .iter()
            .find(|v| self.compartment_b.items.iter().any(|v2| Item::eq(v, v2)))
            .unwrap()
    }

    pub fn items(&self) -> Vec<Item> {
        self.compartment_a
            .items
            .iter()
            .chain(self.compartment_b.items.iter())
            .cloned()
            .collect()
    }
}

impl From<&str> for Backpack {
    fn from(val: &str) -> Self {
        let (a, b) = val.trim().split_at(val.len() / 2);
        Self {
            compartment_a: a.into(),
            compartment_b: b.into(),
        }
    }
}

fn find_common_items<'a>(items: &Vec<Item>, other_items: &Vec<Item>) -> Vec<Item> {
    items
        .iter()
        .filter(|v| other_items.iter().any(|v2| *v == v2))
        .cloned()
        .collect()
}

fn find_badge(backpacks: &[Backpack]) -> Item {
    let group_items = backpacks.iter().map(Backpack::items);

    *group_items
        .reduce(|accum, items| find_common_items(&accum, &items))
        .unwrap()
        .first()
        .unwrap()
}

pub fn main_day3() {
    println!("----- DAY 3 --------");
    let test_data = std::fs::read_to_string("./data/day3.txt").unwrap();

    let backpacks: Vec<Backpack> = test_data.lines().map(Backpack::from).collect();
    println!("Prio of first {}", backpacks[0].duplicate_item().priority());

    let prio_sum: u64 = backpacks
        .iter()
        .map(Backpack::duplicate_item)
        .map(|i| i.priority())
        .sum();
    println!("Prio sum {}", prio_sum);

    let groups = backpacks.chunks(3);
    let badge_prios: u64 = groups.map(find_badge).map(|i| i.priority()).sum();
    println!("Prio badge sum {}", badge_prios);
}
