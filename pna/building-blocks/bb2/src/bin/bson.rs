use std::fs::File;

use bb2::{Direction, Move};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let moves = (100..=1100).map(|idx| Move {steps: idx+22, dir: match idx % 4 {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        _ => Direction::Right,
    }}).collect::<Vec<_>>();

    let file = File::create("output.bson")?;
    for item in moves.clone() {
        let doc = bson::to_document(&item)?;
        doc.to_writer(&file)?;
    }

    let file = File::open("output.bson")?;
    let mut v = Vec::with_capacity(1000);
    loop {
        let b: Move = match bson::from_reader(&file) {
            Ok(move_item) => move_item,
            Err(bson::de::Error::Io(e)) => {
                match e.kind() {
                    std::io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    kind => panic!("{}", kind),

                }
            },
            Err(e) => panic!("{:?}", e),
        };
        v.push(b);
    }

    let file = File::create("output1.bson")?;
    for item in v {
        let doc = bson::to_document(&item)?;
        doc.to_writer(&file)?;
    }

    // the same with Vec
    let mut serialized_to_vec: Vec<u8> = Vec::new();
    
    for item in moves {
        let doc = bson::to_document(&item)?;
        doc.to_writer(&mut serialized_to_vec)?;
    }

    // we need mut and passing by mut reference for working eof marker
    let mut input_vec = serialized_to_vec.as_slice();
    let mut out_vec = Vec::with_capacity(1000);
    loop {
        let m: Move = match bson::from_reader(&mut input_vec){
            Ok(m) => m,
            Err(bson::de::Error::Io(e)) => {
                match e.kind() {
                    std::io::ErrorKind::UnexpectedEof => { break; }
                    kind => panic!("{}", kind),
                }
            }   
            Err(e) => panic!("{:?}", e),
        };
        out_vec.push(m);
    }
    dbg!(&out_vec[..10]);


    Ok(())
}
