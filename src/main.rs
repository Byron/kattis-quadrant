use std::{io, io::Read, str::FromStr};

fn main() -> Result<(), Box<std::error::Error + 'static>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut nums = input.split('\n').map(isize::from_str);

    let (x, y) = match (nums.next(), nums.next()) {
        (Some(x), Some(y)) => (x?, y?),
        _ => panic!("exhausted"),
    };

    println!(
        "{}",
        match (x < 0, y < 0) {
            (false, true) => 4,
            (true, true) => 3,
            (true, false) => 2,
            (false, false) => 1,
        }
    );
    Ok(())
}
