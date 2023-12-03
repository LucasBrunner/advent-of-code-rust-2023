use std::str::FromStr;

#[derive(Debug)]
struct ColorSet {
    red: u32,
    blue: u32,
    green: u32,
}

impl ColorSet {
    fn new(red: u32, blue: u32, green: u32) -> Self {
        Self { red, blue, green }
    }

    fn empty() -> Self {
        Self {
            red: 0,
            blue: 0,
            green: 0,
        }
    }

    pub fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }

    pub fn max_elements(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }
}

#[derive(Debug)]
struct Game {
    rounds: Vec<ColorSet>,
    id: u32,
}

impl Game {
    fn check_possible(&self, availible_cubes: &ColorSet) -> bool {
        self.rounds.iter().all(|round| {
            round.blue <= availible_cubes.blue
                && round.red <= availible_cubes.red
                && round.green <= availible_cubes.green
        })
    }

    fn min_required(&self) -> ColorSet {
        self.rounds
            .iter()
            .fold(ColorSet::empty(), |acc, round| round.max_elements(&acc))
    }
}

fn parse_color_set(colors: &Vec<&str>, color: &str) -> Result<u32, &'static str> {
    colors
        .iter()
        .find(|count| count.ends_with(color))
        .map(|count| {
            count
                .trim()
                .split(" ")
                .next()
                .unwrap()
                .parse::<u32>()
                .or_else(|_err| Err("Could not parse cube count"))
        })
        .unwrap_or_else(|| Ok(0))
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");

        let id = parts
            .next()
            .ok_or("no game id found")?
            .split(" ")
            .skip(1)
            .next()
            .ok_or("no game id found")?
            .parse::<u32>()
            .map_err(|_err| "failed to parse game id")?;

        let rounds_str = parts.next().ok_or("no rountds found in game")?;

        let mut rounds = Vec::new();

        for round in rounds_str.split(";") {
            let colors = round.split(",").collect::<Vec<_>>();
            if colors.len() > 3 {
                return Err("too many colors");
            }

            let red = parse_color_set(&colors, "red")?;
            let blue = parse_color_set(&colors, "blue")?;
            let green = parse_color_set(&colors, "green")?;

            rounds.push(ColorSet { red, blue, green })
        }

        return Ok(Game { rounds, id });
    }
}

fn part_1(games: &Vec<Game>) {
    let availible_cubes = ColorSet::new(12, 14, 13);
    let possible_game_id_sum = games
        .iter()
        .map(|game| match game.check_possible(&availible_cubes) {
            true => game.id,
            false => 0,
        })
        .sum::<u32>();

    println!(
        "The sum of game IDs for valid games is {}",
        possible_game_id_sum
    );
}

fn part_2(games: &Vec<Game>) {
    let game_power_sum = games
        .iter()
        .map(|game| game.min_required().power())
        .sum::<u32>();

    println!(
        "The sum of set powers for the smallest sets to play each game is {}",
        game_power_sum
    );
}

pub fn run() {
    let games = include_str!("../inputs/day_02.txt")
        .lines()
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<_>, _>>()
        .expect("error parsing games");

    part_1(&games);
    part_2(&games);
}
