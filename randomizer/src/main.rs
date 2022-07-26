use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use std::iter::repeat_with;
use std::{fs, io};

fn main() -> io::Result<()> {
    // How many of each?

    let n = fastrand::usize(..=4);
    let m = fastrand::usize(..=4);

    // Randomize n items

    let file_paths = fs::read_dir("lists")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut contents = String::new();

    for path in file_paths {
        let mut file = File::open(path)?;
        file.read_to_string(&mut contents)?;
    }

    let v: Vec<&str> = contents.lines().collect();

    let iter = repeat_with(|| v[fastrand::usize(..v.len())])
        .take(n)
        .map(|i| i.replace(" ", "-"));

    let rand_items: String = Itertools::intersperse(iter, ", ".to_string()).collect();

    // Randomize m people

    let mut contents = String::new();
    let mut file = File::open("people.txt")?;
    file.read_to_string(&mut contents)?;

    let v: Vec<&str> = contents.lines().collect();

    let iter = repeat_with(|| v[fastrand::usize(..v.len())])
        .take(m)
        .map(|i| i.replace(" ", "-"));

    let rand_people: String = Itertools::intersperse(iter, ", ".to_string()).collect();

    // Print the style prompt

    println!("\n--style {rand_people}, {rand_items}\n");

    Ok(())
}
