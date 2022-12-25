use std::error::Error;
use std::io;

#[derive(Debug)]
pub struct SensorData {
    pub sensor_loc: (i32, i32),
    pub beacon_loc: (i32, i32),
}

impl SensorData {
    pub fn range(&self) -> i32 {
        self.dist_from(&self.beacon_loc)
    }

    pub fn dist_from(&self, other: &(i32, i32)) -> i32 {
        (self.sensor_loc.0 - other.0).abs() + (self.sensor_loc.1 - other.1).abs()
    }
}

pub fn parse_lines(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<Vec<SensorData>, Box<dyn Error>> {
    let mut r: Vec<SensorData> = Vec::new();

    for line in lines {
        let line = line?;
        let (s_x, s_y, b_x, b_y) = scan_fmt!(
            &line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i32,
            i32,
            i32,
            i32
        )?;

        r.push(SensorData {
            sensor_loc: (s_x, s_y),
            beacon_loc: (b_x, b_y),
        });
    }

    Ok(r)
}
