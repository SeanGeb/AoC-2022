use std::collections::HashSet;

use crate::dt::*;

pub fn solve_part2(voxels: &[Voxel]) -> u32 {
    // Work out the size of the cube in which the input shape would fit.
    let min = voxels
        .iter()
        .map(|v| *v)
        .reduce(|acc, v| Voxel {
            x: acc.x.min(v.x),
            y: acc.y.min(v.y),
            z: acc.z.min(v.z),
        })
        .unwrap();

    let max = voxels
        .iter()
        .map(|v| *v)
        .reduce(|acc, v| Voxel {
            x: acc.x.max(v.x),
            y: acc.y.max(v.y),
            z: acc.z.max(v.z),
        })
        .unwrap();

    // Expand the bounding box to ensure it has an air gap around it.
    let min = (min + (-1, -1, -1)).unwrap();
    let max = (max + (1, 1, 1)).unwrap();

    // Perform a search to create a "negative" or "cast" of the shape.
    let voxels: HashSet<Voxel> = voxels.iter().map(|v| *v).collect();

    let mut search_set: HashSet<Voxel> = vec![min].into_iter().collect();
    let mut negative: HashSet<Voxel> = HashSet::new();

    while let Some(search_at) = search_set.iter().next() {
        // This is bit of a hack to get an arbitrary item from the set.
        let search_at = *search_at;
        let search_at = search_set.take(&search_at).unwrap();

        // For each neighbouring voxel...
        for d_v in NEIGHBOUR_FACES {
            let v = (search_at + d_v).unwrap();

            // If this neighbour voxel is in bounds, is empty, and isn't already
            // part of the negative, it must be added to the search set.
            if v.in_bounds(&min, &max) && !voxels.contains(&v) && negative.insert(v) {
                search_set.insert(v);
            }
        }
    }

    // We've built a negative of the input shape, so we know the shape of its
    // outer surface. By inverting the negative we fill any hollow spaces in the
    // shape.
    let mut voxels: HashSet<Voxel> = HashSet::new();
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let v = Voxel { x, y, z };
                if !negative.contains(&v) {
                    voxels.insert(v);
                }
            }
        }
    }

    // Finally, running the non-hollow shape through the part1 solution returns
    // the outer surface area.
    voxels
        .iter()
        .map(|v| -> u32 {
            let mut s = 0;
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
