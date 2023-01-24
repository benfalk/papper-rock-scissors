use std::cmp::Ordering;
use std::ops::{Add, AddAssign};
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Choice {
    Paper,
    Rock,
    Scisors,
}

#[derive(Debug, Clone)]
pub enum Outcome {
    Winner(Player),
    Draw,
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::Scisors => match other {
                Self::Scisors => Ordering::Equal,
                Self::Paper => Ordering::Greater,
                Self::Rock => Ordering::Less,
            },
            Self::Rock => match other {
                Self::Scisors => Ordering::Greater,
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
            },
            Self::Paper => match other {
                Self::Scisors => Ordering::Less,
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
            },
        })
    }
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    pub fn pick(&self, choice: Choice) -> Participation {
        Participation {
            choice,
            player: self.clone(),
            picked_at: SystemTime::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Participation {
    pub player: Player,
    pub choice: Choice,
    pub picked_at: SystemTime,
}

impl Add for Participation {
    type Output = Round;

    fn add(self, rhs: Self) -> Self::Output {
        Round::new(self, rhs)
    }
}

#[derive(Debug, Clone)]
pub struct Round {
    pub player_one: Participation,
    pub player_two: Participation,
    pub outcome: Outcome,
}

impl Round {
    pub fn new(player_one: Participation, player_two: Participation) -> Self {
        let outcome = match player_one.choice.cmp(&player_two.choice) {
            Ordering::Equal => Outcome::Draw,
            Ordering::Less => Outcome::Winner(player_two.player.clone()),
            Ordering::Greater => Outcome::Winner(player_two.player.clone()),
        };

        Self {
            player_one,
            player_two,
            outcome,
        }
    }

    pub fn started_at(&self) -> SystemTime {
        std::cmp::min(self.player_one.picked_at, self.player_two.picked_at)
    }

    pub fn finisehd_at(&self) -> SystemTime {
        std::cmp::max(self.player_one.picked_at, self.player_two.picked_at)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Game {
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            rounds: Vec::with_capacity(capacity),
        }
    }

    pub fn add_round(&mut self, round: Round) {
        self.rounds.push(round);
    }
}

impl AddAssign<Round> for Game {
    fn add_assign(&mut self, rhs: Round) {
        self.add_round(rhs);
    }
}

fn main() {
    let player_one = Player::new("John");
    let player_two = Player::new("Jeff");
    let mut game = Game::default();

    game += player_one.pick(Choice::Rock) + player_two.pick(Choice::Scisors);

    println!("{game:#?}");
}
