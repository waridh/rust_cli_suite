use std::env::args;

fn main() {
    let mut slice: Vec<String> = args().collect();
    slice.remove(0);
    let Some(output) = slice.into_iter().reduce(|mut acc, ele| {
        acc.push(' ');
        acc.push_str(&ele);
        acc
    }) else {panic!();};
    println!("{output}");
}
