struct Supplies {
    pub calories: u64,
}

impl TryFrom<&str> for Supplies {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let calorie_values: Result<Vec<u64>, _> = value
            .split_whitespace()
            .map(|v| v.trim().parse::<u64>())
            .collect();
        let calories = calorie_values?.iter().sum();

        Ok(Supplies { calories })
    }
}

impl From<u64> for Supplies {
    fn from(val: u64) -> Self {
        Supplies { calories: val }
    }
}

pub fn main_day1() {
    let test_data = std::fs::read_to_string("./data/day1_input.txt").unwrap();

    let mut supplies: Vec<Supplies> = test_data
        .split("\n\n")
        .map(|data| Supplies::try_from(data).unwrap())
        .collect();
    supplies.sort_by_key(|p| p.calories);

    println!("Most calories has {}", supplies.last().unwrap().calories);

    let calories_top3: u64 = supplies.iter().rev().take(3).map(|s| s.calories).sum();
    println!("Top 3 calories is {}", calories_top3);
}
