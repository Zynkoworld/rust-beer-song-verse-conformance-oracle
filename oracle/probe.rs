pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
              Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
              Take it down and pass it around, no more bottles of beer on the wall.\n"
            .to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\n\
              Take one down and pass it around, 1 bottle of beer on the wall.\n"
            .to_string(),
        n if n > 2 && n <= 99 => format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.\n\
             Take one down and pass it around, {n_minus_1} bottles of beer on the wall.\n",
            n = n,
            n_minus_1 = n - 1
        ),
        _ => panic!(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}


fn main() {
    let inputs: Vec<u32> = vec![99u32, 3u32, 2u32, 1u32, 0u32];
    let mut out: Vec<String> = Vec::new();
    for &x in inputs.iter() {
        let s = verse(x); let mut e = String::new(); for c in s.chars() { match c { '"' => e.push_str("\\\""), '\\' => e.push_str("\\\\"), '\n' => e.push_str("\\n"), '\t' => e.push_str("\\t"), '\r' => e.push_str("\\r"), _ => e.push(c) } } out.push(format!("\"{}\"", e));
    }
    println!("{{\"out\": [{}]}}", out.join(","));
}
