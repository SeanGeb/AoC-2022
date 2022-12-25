use std::io::Write;
use std::{cmp, io};

use crate::parse::SensorData;

pub fn solve_part2(max: i32, sensors: &Vec<SensorData>) -> i64 {
    // A (4e6)^2 search space is far too large to use and search naively.
    // Instead, iterate over possible x and y co-ordinates. At each position,
    // iterate over the list of sensors we might be in range of. Work out the
    // largest possible jump to the right (x+) and do so. If that takes us off
    // the edge, go to (0, ++y).
    print!("finding uncovered cell: ");
    io::stdout().flush().unwrap();
    for y in 0..=max {
        let mut x: i32 = 0;

        while x <= max {
            // Check over the sensors.
            let mut next_x = 0;

            for s in sensors {
                let range = s.range();
                let dist = s.dist_from(&(x, y));

                if range < dist {
                    continue;
                }

                let x_end_of_range = s.sensor_loc.0 + range - (s.sensor_loc.1 - y).abs();
                // +1 to ensure next x value is outside this sensor's range.
                next_x = cmp::max(next_x, x_end_of_range + 1);
            }

            if next_x == 0 {
                // If we couldn't jump forwards, must have found a position
                // which isn't covered by any sensor.
                println!(" done!\nx={x} y={y}");
                return (x as i64) * 4_000_000 + (y as i64);
            }

            x = next_x;
        }

        if y & 0x1_FFFF == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }
    }
    println!("no solution found!");

    panic!("no solution found!");
}
