#[derive(Debug)]
pub struct HighScores {
    scores_list: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores_list: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores_list.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores_list.last() {
            Some(value) => Some(*value),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores_list.iter().fold(None, |a, b| a.max(Some(*b)))
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut best = self.scores_list.to_vec();
        best.sort();
        best.reverse();
        if best.len() < 3 {
            return best.to_vec();
        }
        best[0..3].to_vec()
    }
}
