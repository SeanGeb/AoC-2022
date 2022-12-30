use std::collections::HashSet;

use crate::dt::*;

pub fn solve_part1(voxels: &[Voxel]) -> u32 {
    let voxels: HashSet<Voxel> = voxels.iter().map(|v| *v).collect();

    // For each voxel, count the number of exposed faces, and sum up.
    voxels
        .iter()
        .map(|v| -> u32 {
            let mut s = 0;
            // Check in each different direction.
            for d_v in NEIGHBOUR_FACES {
                let v = (*v + d_v).unwrap();
                if !voxels.contains(&v) {
                    s += 1;
                }
            }
            s
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let voxels: Vec<Voxel> = vec![(1, 1, 1).into(), (2, 1, 1).into()];

        assert_eq!(solve_part1(&voxels), 10);
    }
}
