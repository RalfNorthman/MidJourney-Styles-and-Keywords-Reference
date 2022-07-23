use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use std::iter::repeat_with;
use std::{fs, io};

fn main() -> io::Result<()> {
    let file_paths = fs::read_dir("lists")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut contents = String::new();

    for path in file_paths {
        let mut file = File::open(path)?;
        file.read_to_string(&mut contents)?;
    }

    let v: Vec<&str> = contents.lines().collect();

    let rand_items: String = repeat_with(|| v[fastrand::usize(..v.len())])
        .take(5)
        .map(|i| i.replace(" ", "-"))
        .intersperse(", ".to_string())
        .collect();

    println!("{rand_items}");

    Ok(())
}
