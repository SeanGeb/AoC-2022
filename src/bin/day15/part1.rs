use std::collections::HashSet;
use std::io::{self, Write};

use crate::parse::SensorData;

pub fn solve_part1(row: i32, sensors: &Vec<SensorData>) -> usize {
    // For each SensorData, work out the taxicab distance to the beacon.
    // Then work out all the cells <= the taxicab distance in row 2_000_000 and
    // add them to the set.
    // Finally, remove all cells from the row 2_000_000 set with a beacon
    // already present, count the number of items remaining, and return that.
    let mut no_beacon_cells = HashSet::<i32>::new();
    print!("adding beacon data: ");
    io::stdout().flush().unwrap();

    for s in sensors {
        let taxicab_dist =
            (s.sensor_loc.0 - s.beacon_loc.0).abs() + (s.sensor_loc.1 - s.beacon_loc.1).abs();
        let d_y = (row - s.sensor_loc.1).abs();
        let d_x = taxicab_dist - d_y;
        for x in (s.sensor_loc.0 - d_x)..=(s.sensor_loc.0 + d_x) {
            no_beacon_cells.insert(x);
        }
        print!(".");
        io::stdout().flush().unwrap();
    }

    println!(" done!");

    // Remove cells containing beacons.
    for s in sensors {
        let (x, y) = s.beacon_loc;
        if y == row {
            no_beacon_cells.remove(&x);
        }
    }

    println!("count cells...");

    no_beacon_cells.len()
}
