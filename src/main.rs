use std::cmp::Ordering;
use std::time::SystemTime;
use std::ops::Add;

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

    pub fn make_choice(&self, choice: Choice) -> Participation {
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
        Round {
            player_one: self,
            player_two: rhs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Round {
    pub player_one: Participation,
    pub player_two: Participation,
}

impl Round {
    pub fn started_at(&self) -> SystemTime {
        std::cmp::min(
            self.player_one.picked_at,
            self.player_two.picked_at,
        )
    }

    pub fn finisehd_at(&self) -> SystemTime {
        std::cmp::max(
            self.player_one.picked_at,
            self.player_two.picked_at,
        )
    }

    pub fn outcome(&self) -> Outcome {
        match self.player_two.choice.cmp(&self.player_one.choice) {
            Ordering::Equal => Outcome::Draw,
            Ordering::Less => Outcome::Winner(self.player_one.player.clone()),
            Ordering::Greater => Outcome::Winner(self.player_two.player.clone()),
        }
    }
}

fn main() {
    let player_one = Player::new("John");
    let player_two = Player::new("Jeff");
    let round = player_one.make_choice(Choice::Rock) + player_two.make_choice(Choice::Scisors);

    println!("{round:#?}");
    println!("{:?}", round.outcome());
}
