use bevy::{
    prelude::*,
    utils::{hashbrown::hash_map, HashMap, HashSet},
};
use rand::prelude::IteratorRandom;

use crate::matches::*;

/// The main struct representing the logical match 3 board
#[derive(Eq, PartialEq, Debug, Clone, Resource)]
pub struct Board {
    pub dimensions: UVec2,
    pub gems: HashMap<UVec2, u32>,
    pub types: HashSet<u32>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = (0..self.dimensions.y).map(|y| {
            f.write_fmt(format_args!(
                "{:?}\n",
                (0..self.dimensions.x)
                    .map(|x| self.gems[&<[u32; 2] as Into<UVec2>>::into([x, y])])
                    .collect::<Vec<u32>>()
            ))
        });
        for res in res {
            match res {
                Ok(_) => {}
                err => return err,
            }
        }
        Ok(())
    }
}

impl From<Vec<Vec<u32>>> for Board {
    fn from(rows: Vec<Vec<u32>>) -> Self {
        let mut gems = HashMap::default();
        let mut width = 0;
        let mut height = 0;
        let mut types = HashSet::default();
        rows.iter().enumerate().for_each(|(y, row)| {
            height += 1;
            row.iter().enumerate().for_each(|(x, gem)| {
                gems.insert([x as u32, y as u32].into(), *gem);
                types.insert(*gem);
                if height == 1 {
                    width += 1;
                }
            })
        });
        Board {
            gems,
            dimensions: [width, height].into(),
            types,
        }
    }
}

impl Board {
    /// Returns a reference to the gem type at the given position.
    pub fn get(&self, pos: &UVec2) -> Option<&u32> {
        self.gems.get(pos)
    }

    /// Returns an iterator over the kvps in the board
    pub fn iter(&self) -> hash_map::Iter<UVec2, u32> {
        self.gems.iter()
    }

    pub fn remove(&mut self, pos: &UVec2) {
        self.gems.remove(pos);
    }

    pub fn insert(&mut self, pos: UVec2, typ: u32) {
        self.gems.insert(pos, typ);
    }

    pub fn drop(&mut self) -> HashSet<(UVec2, UVec2)> {
        let mut moves = HashSet::default();
        for x in 0..self.dimensions.x {
            for y in (0..self.dimensions.y).rev() {
                if self.get(&[x, y].into()).is_none() {
                    let mut offset = 0;
                    for above in (0..y).rev() {
                        if let Some(typ) = self.get(&[x, above].into()).cloned() {
                            let new_pos = [x, y - offset];
                            moves.insert(([x, above].into(), new_pos.into()));
                            self.remove(&[x, above].into());
                            self.insert(new_pos.into(), typ);
                            offset += 1;
                        }
                    }
                }
            }
        }
        moves
    }

    pub fn fill(&mut self) -> HashSet<(UVec2, u32)> {
        let mut drops = HashSet::default();
        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                let pos = [x, y];
                if self.get(&pos.into()).is_none() {
                    let new_type = self
                        .types
                        .iter()
                        .choose(&mut rand::thread_rng())
                        .copied()
                        .unwrap();
                    self.insert(pos.into(), new_type);
                    drops.insert((pos.into(), new_type));
                }
            }
        }
        drops
    }

    pub fn swap(&mut self, pos1: &UVec2, pos2: &UVec2) -> Result<(), SwapError> {
        let gem1 = self.get(pos1).copied().ok_or(SwapError::NoGem(*pos1))?;
        let gem2 = self.get(pos2).copied().ok_or(SwapError::NoGem(*pos2))?;
        self.gems.insert(*pos1, gem2);
        self.gems.insert(*pos2, gem1);
        if self.get_matches().is_empty() {
            self.gems.insert(*pos1, gem1);
            self.gems.insert(*pos2, gem2);
            Err(SwapError::NoMatches)
        } else {
            Ok(())
        }
    }

    /// Like swap but doesn't permanently change the board, useful for match checking
    fn try_swap(&mut self, pos1: &UVec2, pos2: &UVec2) -> Result<(), SwapError> {
        let gem1 = self.get(pos1).copied().ok_or(SwapError::NoGem(*pos1))?;
        let gem2 = self.get(pos2).copied().ok_or(SwapError::NoGem(*pos2))?;
        self.gems.insert(*pos1, gem2);
        self.gems.insert(*pos2, gem1);
        if self.get_matches().is_empty() {
            self.gems.insert(*pos1, gem1);
            self.gems.insert(*pos2, gem2);
            Err(SwapError::NoMatches)
        } else {
            self.gems.insert(*pos1, gem1);
            self.gems.insert(*pos2, gem2);
            Ok(())
        }
    }

    pub fn get_matches(&self) -> Matches {
        let mut matches = self.straight_matches(MatchDirection::Horizontal);
        matches.append(&mut self.straight_matches(MatchDirection::Vertical));
        matches
    }

    fn straight_matches(&self, direction: MatchDirection) -> Matches {
        let mut matches = Matches::default();
        let mut current_match = vec![];
        let mut previous_type = None;
        for one in match direction {
            MatchDirection::Horizontal => 0..self.dimensions.x,
            MatchDirection::Vertical => 0..self.dimensions.y,
        } {
            for two in match direction {
                MatchDirection::Horizontal => 0..self.dimensions.y,
                MatchDirection::Vertical => 0..self.dimensions.x,
            } {
                let pos = [
                    match direction {
                        MatchDirection::Horizontal => one,
                        MatchDirection::Vertical => two,
                    },
                    match direction {
                        MatchDirection::Horizontal => two,
                        MatchDirection::Vertical => one,
                    },
                ]
                .into();

                let current_type = *self.get(&pos).unwrap();
                if current_match.is_empty() || previous_type.unwrap() == current_type {
                    previous_type = Some(current_type);
                    current_match.push(pos);
                } else if previous_type.unwrap() != current_type {
                    match current_match.len() {
                        0 | 1 | 2 => {}
                        _ => matches.add(Match::Straight(current_match.iter().cloned().collect())),
                    }
                    current_match = vec![pos];
                    previous_type = Some(current_type);
                }
            }
            match current_match.len() {
                0 | 1 | 2 => {}
                _ => matches.add(Match::Straight(current_match.iter().cloned().collect())),
            }
            current_match = vec![];
            previous_type = None;
        }
        matches
    }

    pub fn clear_matches(&mut self) {
        loop {
            let matches = self.get_matches();
            if matches.is_empty() {
                break;
            }
            for mat in matches.matches.iter() {
                match mat {
                    Match::Straight(gems) => {
                        for gem in gems {
                            self.remove(gem);
                        }
                    }
                }
            }
            self.drop();
            self.fill();
        }
    }

    fn adjacents(&self, pos: UVec2) -> Vec<UVec2> {
        let mut adjacents = Vec::with_capacity(4);
        if pos.x != 0 {
            adjacents.push(pos.left());
        }
        if pos.x != self.dimensions.x {
            adjacents.push(pos.right());
        }
        if pos.y != 0 {
            adjacents.push(pos.up());
        }
        if pos.y != self.dimensions.y {
            adjacents.push(pos.down());
        }
        adjacents
    }

    /// Returns any moves that would result in a match by swapping with a neighboring gem
    pub fn get_matching_moves(&self) -> HashSet<BoardMove> {
        let mut moves = HashSet::new();
        let mut temp_board = self.clone(); // NOTE: This clone is not ideal. First candidate for optimizing
        for (pos, _) in self.iter() {
            for adjacent in self.adjacents(*pos) {
                if temp_board.try_swap(pos, &adjacent).is_ok() {
                    moves.insert(BoardMove(*pos, adjacent));
                }
            }
        }
        moves
    }
}

pub enum SwapError {
    NoGem(UVec2),
    NoMatches,
}

/// Represents a swap between two gems, order of gems doesn't matter
#[derive(Eq, Debug)]
pub struct BoardMove(pub UVec2, pub UVec2);

impl PartialEq for BoardMove {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl core::hash::Hash for BoardMove {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Self(a, b) = self;
        if a.x < b.x {
            a.hash(state);
            b.hash(state);
        } else if b.x < a.x {
            b.hash(state);
            a.hash(state);
        } else if a.y < b.y {
            a.hash(state);
            b.hash(state);
        } else {
            b.hash(state);
            a.hash(state);
        }
    }
}

trait BoardPosition {
    fn left(&self) -> Self;
    fn right(&self) -> Self;
    fn up(&self) -> Self;
    fn down(&self) -> Self;
    fn cardinally_adjacent(&self, other: &Self) -> bool;
}

impl BoardPosition for UVec2 {
    fn left(&self) -> Self {
        Self::new(self.x.saturating_sub(1), self.y)
    }

    fn right(&self) -> Self {
        Self::new(self.x.saturating_add(1), self.y)
    }

    fn up(&self) -> Self {
        Self::new(self.x, self.y.saturating_sub(1))
    }

    fn down(&self) -> Self {
        Self::new(self.x, self.y.saturating_add(1))
    }

    fn cardinally_adjacent(&self, other: &Self) -> bool {
        self == &other.left()
            || self == &other.right()
            || self == &other.up()
            || self == &other.down()
    }
}