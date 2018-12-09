use crate::utils;

fn is_polar_pair(cs: &(char, char)) -> bool {
    let (c1, c2) = cs;
    (*c1 != *c2) && (*c1 == c2.to_ascii_uppercase() || c1.to_ascii_uppercase() == *c2)
}

fn remove_polar_pairs(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut result = String::new();
    let mut i = 0usize;
    while i < (text.len() - 1) {
        let pair = (chars[i], chars[i + 1]);
        if is_polar_pair(&pair) {
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result.push(chars[i]);
    result
}

fn remove_all_polar_pairs(text: &str) -> String {
    let mut old_len = text.len();
    let mut curr = remove_polar_pairs(text);
    let mut i = 1;
    while curr.len() < old_len {
        old_len = curr.len();
        curr = remove_polar_pairs(&curr);
        i += 1
    }
    curr
}

pub fn run() {
    let filename = "inputs/05/input.txt";
    let contents: String = utils::read_input(&filename).trim().to_string();

    println!("Final:\n{:?}", remove_polar_pairs(&contents).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polar_pair_elimination() {
        assert_eq!(is_polar_pair(&('l', 'o')), false);
        assert_eq!(remove_polar_pairs("lol"), "lol".to_string());
        assert_eq!(remove_polar_pairs("Llol"), "ol".to_string());
        assert_eq!(
            remove_polar_pairs("dabAcCaCBAcCcaDA"),
            "dabAaCBAcaDA".to_string()
        );
        assert_eq!(
            remove_all_polar_pairs("dabAcCaCBAcCcaDA"),
            "dabCBAcaDA".to_string()
        );
    }
}
