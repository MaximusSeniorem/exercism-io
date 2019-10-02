pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut proverb: String = list
        .iter()
        .zip(list.iter().skip(1))
        .map(|(want, lost)| format!("For want of a {} the {} was lost.\n", want, lost))
        .collect();

    proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    proverb
}
