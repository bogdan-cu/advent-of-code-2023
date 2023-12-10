use std::io::Read;

use color_eyre::{
    eyre::{eyre, Ok},
    Result,
};

pub fn read_input(path: &str) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Pipe {
    e: bool,
    n: bool,
    w: bool,
    s: bool,
}

impl Pipe {
    pub fn new(input: &char) -> Result<Self> {
        match *input {
            'S' => Ok(Pipe {
                e: true,
                n: true,
                w: true,
                s: true,
            }),
            '|' => Ok(Pipe {
                e: false,
                n: true,
                w: false,
                s: true,
            }),
            '-' => Ok(Pipe {
                e: true,
                n: false,
                w: true,
                s: false,
            }),
            'L' => Ok(Pipe {
                e: true,
                n: true,
                w: false,
                s: false,
            }),
            'J' => Ok(Pipe {
                e: false,
                n: true,
                w: true,
                s: false,
            }),
            '7' => Ok(Pipe {
                e: false,
                n: false,
                w: true,
                s: true,
            }),
            'F' => Ok(Pipe {
                e: true,
                n: false,
                w: false,
                s: true,
            }),
            '.' => Ok(Pipe {
                e: false,
                n: false,
                w: false,
                s: false,
            }),
            x => Err(eyre!("found improper character in input: {}", x)),
        }
    }

    pub fn is_source(&self) -> bool {
        self.e && self.n && self.w && self.s
    }

    pub fn is_ground(&self) -> bool {
        !self.e && !self.n && !self.w && !self.s
    }

    pub fn is_connected_to(
        &self,
        other: &Self,
        idx_1: (usize, usize),
        idx_2: (usize, usize),
    ) -> bool {
        if idx_1.0 == idx_2.0 {
            if idx_1.1 == idx_2.1 + 1 {
                return self.w && other.e;
            }
            if idx_1.1 == idx_2.1 - 1 {
                return self.e && other.w;
            }
        }
        if idx_1.1 == idx_2.1 {
            if idx_1.0 == idx_2.0 + 1 {
                return self.n && other.s;
            }
            if idx_1.0 == idx_2.0 - 1 {
                return self.s && other.n;
            }
        }
        false
    }
}

pub enum Direction {
    East,
    North,
    West,
    South,
}

impl Direction {
    pub fn from_pos(&self, origin: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::East => Some((origin.0, origin.1 + 1)),
            Self::North => {
                if origin.0 == 0 {
                    return None;
                }
                Some((origin.0 - 1, origin.1))
            }
            Self::West => {
                if origin.1 == 0 {
                    return None;
                }
                Some((origin.0, origin.1 - 1))
            }
            Self::South => Some((origin.0 + 1, origin.1)),
        }
    }
}
