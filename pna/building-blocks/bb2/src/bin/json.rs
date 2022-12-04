use std::fs::File;
use std::io::{Result, BufReader};
use serde_json;
use bb2::{Move, Direction};

fn main() -> Result<()> {
    {
        let f = File::create("output.json")?;
        let a = Move { dir: Direction::Up, steps: 25 };
        serde_json::to_writer(f, &a)?;
    }
    {
        let f = File::open("output.json")?;
        let r = BufReader::new(f);
        let a: Move = serde_json::from_reader(r)?;
        dbg!(a);
    }
    Ok(())
}
