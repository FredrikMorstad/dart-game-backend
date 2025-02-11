use core::fmt;

use serde::{Deserialize, Serialize};

use crate::api::api_errors::NotationError;

const BASE_POINTS: &'static [u8] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
];

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum PointNotation {
    MISS,
    T(u8),
    D(u8),
    S(u8),
    B,
    DB,
}

impl fmt::Display for PointNotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PointNotation::MISS => write!(f, "MISS"),
            PointNotation::T(value) => write!(f, "T({})", value),
            PointNotation::D(value) => write!(f, "D({})", value),
            PointNotation::S(value) => write!(f, "S({})", value),
            PointNotation::B => write!(f, "B"),
            PointNotation::DB => write!(f, "DB"),
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Point {
    pub notation: PointNotation, //miss, double, triple bull ...
    pub score: u8,               // the counting score
}

impl Point {
    pub fn new(s: &str) -> Result<Point, NotationError> {
        let point = match s {
            "MISS" => Point {
                notation: PointNotation::MISS,
                score: 0,
            },

            "B" => Point {
                notation: PointNotation::B,
                score: 25,
            },

            "DB" => Point {
                notation: PointNotation::DB,
                score: 50,
            },
            _ => {
                if s.len() > 3 {
                    return Err(NotationError::InvalidFormat);
                }

                let base_point = s[1..].parse::<u8>()?;
                if !BASE_POINTS.contains(&base_point) {
                    return Err(NotationError::InvalidPoint);
                }
                let amp = &s[0..1];
                let point = match amp {
                    "T" => Point {
                        notation: PointNotation::T(base_point),
                        score: base_point * 3,
                    },

                    "D" => Point {
                        notation: PointNotation::D(base_point),
                        score: base_point * 2,
                    },

                    "S" => Point {
                        notation: PointNotation::S(base_point),
                        score: base_point,
                    },
                    _ => {
                        return Err(NotationError::InvalidFormat);
                    }
                };
                point
            }
        };
        Ok(point)
    }
}
