fn list_digit(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![0];
    }
    let mut x = n;

    std::iter::from_fn(move || {
        if x == 0 {
            None
        } else {
            let current = x % 10;
            x /= 10;
            Some(current)
        }
    })
    .collect::<Vec<u32>>()
}

pub fn is_armstrong_number(num: u32) -> bool {
    let list = list_digit(num);
    println!("{}", num);
    let pow = list.len() as u32;

    num == list.iter().map(|x| x.pow(pow)).sum()
}
