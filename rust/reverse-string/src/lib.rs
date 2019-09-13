pub fn reverse(input: &str) -> String {
    input.chars().rev().collect() //collect::<String>()
}

/*
str.chars()
returns an iterable on a String characters

iter.rev()
reverse itrator's direction

iter.collect::<String>()
Transforms an iterator into a collection, String being a collection of chars

::<>
"turbo fish" helps the inferene system deduce the type
mandatory here since we have the return type of the function
*/