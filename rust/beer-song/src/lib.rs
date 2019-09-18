pub fn verse(n: i32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => format!("{x} bottles of beer on the wall, {x} bottles of beer.\nTake one down and pass it around, {x1} bottles of beer on the wall.\n", x=n, x1=n-1).to_string()
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut verses = vec![];
    for i in (end..=start).rev() {
        if i != end { verses.push(format!("{}\n", verse(i))); }
        else { verses.push(verse(i)); }
    }
    verses.concat()
}
