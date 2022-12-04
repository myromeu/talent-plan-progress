use bb2::{Move, Direction};
use ron;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = Move { dir: Direction::Left, steps: 700 };
    let mut buf: Vec<u8> = Vec::new();
    ron::ser::to_writer(&mut buf, &a)?;
    let str = String::from_utf8(buf)?;
    println!("RON: {}", str);

    buf = Vec::new();
    ron::ser::to_writer_pretty(&mut buf, &a, Default::default())?;
    println!("RON pretty: {}", String::from_utf8(buf)?);
    Ok(())
}
