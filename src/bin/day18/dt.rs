use std::error::Error;
use std::io;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Voxel {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Voxel {
    // Returns a boolean indicating if this voxel lies within the cube marked
    // out by the min and max voxels given. (Range is inclusive.)
    pub fn in_bounds(&self, min: &Self, max: &Self) -> bool {
        (min.x..=max.x).contains(&self.x)
            && (min.y..=max.y).contains(&self.y)
            && (min.z..=max.z).contains(&self.z)
    }
}

impl Add<(i16, i16, i16)> for Voxel {
    type Output = Option<Self>;

    fn add(self, rhs: (i16, i16, i16)) -> Self::Output {
        Some(Self {
            x: self.x.checked_add(rhs.0)?,
            y: self.y.checked_add(rhs.1)?,
            z: self.z.checked_add(rhs.2)?,
        })
    }
}

impl From<(i16, i16, i16)> for Voxel {
    fn from(value: (i16, i16, i16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<Voxel> for (i16, i16, i16) {
    fn from(value: Voxel) -> Self {
        (value.x, value.y, value.z)
    }
}

pub fn parse_lines(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<Vec<Voxel>, Box<dyn Error>> {
    let mut r: Vec<Voxel> = Vec::new();
    for line in lines {
        let line = line?;
        let (x, y, z) = scan_fmt!(line.as_str(), "{d},{d},{d}", i16, i16, i16)?;
        r.push(Voxel { x, y, z });
    }

    Ok(r)
}

pub const NEIGHBOUR_FACES: [(i16, i16, i16); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];
