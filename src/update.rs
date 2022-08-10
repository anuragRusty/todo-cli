pub fn check_in(strng: &String, str1: String) -> bool {
    let check: Vec<_> = strng.chars().collect();

    if check[0].to_string().to_lowercase() != str1 {
        return false;
    }
    for i in 1_usize..check.len() {
        if !check[i].to_string().parse::<i32>().is_ok() {
            return false;
        }
    }
    true
}

pub fn get_index(strng: String) -> usize {
    let get: Vec<_> = strng.chars().collect();
    let mut num_char: Vec<String> = vec![];

    for i in 1_usize..get.len() {
        num_char.push(get[i].to_string());
    }
    num_char.join("").parse().unwrap_or(0)
}
