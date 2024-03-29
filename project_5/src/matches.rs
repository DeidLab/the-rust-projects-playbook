use bevy::{math::UVec2, utils::HashSet};

pub enum MatchDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub enum Match {
    Straight(HashSet<UVec2>),
}

#[derive(Default, Clone)]
pub struct Matches {
    pub(crate) matches: Vec<Match>,
}

impl Matches {
    pub fn add(&mut self, mat: Match) {
        self.matches.push(mat)
    }

    pub fn append(&mut self, other: &mut Matches) {
        self.matches.append(&mut other.matches);
    }

    /// Returns the coordinates of all matches in this collection without any repeated values
    pub fn without_duplicates(&self) -> HashSet<UVec2> {
        self.matches
            .iter()
            .flat_map(|mat| match mat {
                Match::Straight(mat) => mat,
            })
            .cloned()
            .collect()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }
}