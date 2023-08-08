use crate::day_2::Outcome::*;
use crate::day_2::RPS::*;
use std::cmp::{
    Ordering,
    Ordering::{Equal, Greater, Less},
};

/**
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack
storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round,
the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a
winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats
Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle
input) that they say will be sure to help you win. "The first column is what your opponent is going
to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is
called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper,
and Z for Scissors. Winning every time would be suspicious, so the responses must have been
carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum
of your scores for each round. The score for a single round is the score for the shape you selected
(1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you
lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the
score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:
```txt
A Y
B X
C Z
```
This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This
    ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).

    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This
    ends in a loss for you with a score of 1 (1 + 0).

    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15
(8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
*/
pub fn calculate_rps_strategy_guide_score(list_text: &str) -> i32 {
    list_text.lines().fold(0, |score, line| {
        let mut line = line.chars();
        let opponent: RPS = line.next().expect("Invalid input!").into();
        let player: RPS = line.next_back().expect("Invalid input!").into();

        score + player.value() + player.play(&opponent).value()
    })
}

/**
--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says
how the round needs to end: X means you need to lose, Y means you need to end the round in a draw,
and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to
choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw
    (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.

    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X)
    with a score of 1 + 0 = 1.

    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total
score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything
goes exactly according to your strategy guide?
*/
pub fn calculate_rps_strategy_guide_moves(list_text: &str) -> i32 {
    list_text.lines().fold(0, |score, line| {
        let mut chars = line.chars();
        let opponent: RPS = chars.next().unwrap().into();
        let outcome: Outcome = chars.next_back().unwrap().into();
        let player = match (&opponent, &outcome) {
            (_, Tie) => opponent,
            (Rock, Loss) => Scissors,
            (Rock, Win) => Paper,
            (Paper, Loss) => Rock,
            (Paper, Win) => Scissors,
            (Scissors, Loss) => Paper,
            (Scissors, Win) => Rock,
        };

        score + player.value() + outcome.value()
    })
}

#[derive(Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Tie,
}

impl Outcome {
    fn value(&self) -> i32 {
        match self {
            Loss => 0,
            Tie => 3,
            Win => 6,
        }
    }
}

impl RPS {
    fn value(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn play(&self, other: &RPS) -> Outcome {
        match self.partial_cmp(other).unwrap_or(Equal) {
            Less => Loss,
            Equal => Tie,
            Greater => Win,
        }
    }
}

impl From<char> for RPS {
    fn from(c: char) -> Self {
        match c {
            'A' => Rock,
            'X' => Rock,
            'B' => Paper,
            'Y' => Paper,
            'C' => Scissors,
            'Z' => Scissors,
            _ => panic!("Invalid input!"),
        }
    }
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Loss,
            'Y' => Tie,
            'Z' => Win,
            _ => panic!("Invalid input!"),
        }
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Equal);
        }
        match (self, other) {
            (Rock, Paper) => Some(Less),
            (Paper, Scissors) => Some(Less),
            (Scissors, Rock) => Some(Less),
            _ => Some(Greater),
        }
    }
}
