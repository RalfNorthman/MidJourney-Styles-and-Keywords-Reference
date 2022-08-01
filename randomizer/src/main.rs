use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use std::iter::repeat_with;
use std::{fs, io};

fn from_dir(n: usize, dir_name: &str) -> io::Result<String> {
    let file_paths = fs::read_dir(dir_name)?
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
        .map(|i| i.replace(' ', "-"));

    let rand = Itertools::intersperse(iter, ", ".to_string()).collect();
    Ok(rand)
}

fn from_file(m: usize, file_name: &str) -> io::Result<String> {
    let mut contents = String::new();
    let mut file = File::open(file_name)?;
    file.read_to_string(&mut contents)?;

    let v: Vec<&str> = contents.lines().collect();

    let iter = repeat_with(|| v[fastrand::usize(..v.len())])
        .take(m)
        .map(|i| i.replace(' ', "-"));

    let rand: String = Itertools::intersperse(iter, ", ".to_string()).collect();
    Ok(rand)
}

fn few() -> usize {
    fastrand::usize(..=1) + fastrand::usize(..=1)
}

fn maybe() -> usize {
    fastrand::usize(..=1)
}

fn maybe_expl() -> usize {
    let mut n = maybe();
    let mut acc = n;
    while n > 0 {
        n = maybe();
        acc += n;
    }
    acc
}

fn main() -> io::Result<()> {
    // How many of each?

    let misc = from_dir(maybe_expl() + few(), "opinionated")?;
    let aeth = from_file(maybe_expl(), "aesthetics.txt")?;

    // Print the style prompt

    println!("\n --iw 1.5 cinematic feature-film 35mm, motion-picture, movie --ar 3:2");
    println!(" --style {misc}, {aeth}\n");

    Ok(())
}
