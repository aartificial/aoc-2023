use crate::custom_error::GameError;
use crate::score::Score;
use crate::set::Set;
use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub(crate) id: u32,
    sets: Vec<Set>,
}

impl Game {
    pub fn is_possible(&self) -> bool {
        self.sets.iter().all(|set| set.is_possible())
    }

    pub fn minset(&self) -> Score {
        let mut max_score = Score::default();
        for set in &self.sets {
            let set_score = set.calculate_color_totals();
            max_score.red = set_score.red.max(max_score.red);
            max_score.green = set_score.green.max(max_score.green);
            max_score.blue = set_score.blue.max(max_score.blue);
        }
        max_score
    }
}

impl FromStr for Game {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        let id = parts[0][5..].trim().parse()?;
        let sets = parts[1]
            .split(';')
            .map(|set| set.parse())
            .collect::<Result<Vec<Set>, _>>()?;

        Ok(Game { id, sets })
    }
}
