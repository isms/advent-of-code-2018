use crate::utils;

mod part1 {
    use std::collections::HashMap;

    pub fn count_letters(id: &str) -> HashMap<char, i32> {
        id.chars().into_iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
    }

    pub fn count_repetitions(hashmap: &HashMap<char, i32>, n: i32) -> usize {
        hashmap.iter().filter(|(&_, &v)| v == n).count()
    }

    pub fn checksum(entries: &Vec<String>) -> usize {
        let counts: Vec<HashMap<char, i32>> = entries.iter().map(|e| count_letters(e)).collect();
        let n2 = counts
            .iter()
            .filter(|&m| count_repetitions(m, 2) > 0)
            .count();
        let n3 = counts
            .iter()
            .filter(|&m| count_repetitions(m, 3) > 0)
            .count();
        n2 * n3
    }
}

mod part2 {
    use std::collections::HashMap;

    pub fn distance(str1: &str, str2: &str) -> usize {
        str1.chars()
            .zip(str2.chars())
            .filter(|(a, b)| *a != *b)
            .count()
    }

    pub fn letters_in_common(str1: &str, str2: &str) -> String {
        str1.chars()
            .zip(str2.chars())
            .filter(|(a, b)| *a == *b)
            .map(|t| t.0)
            .collect::<String>()
    }

    pub fn get_closest_pair(entries: &Vec<String>) -> (String, String) {
        let mut counts: HashMap<usize, (String, String)> = HashMap::new();
        for i in 0..entries.len() {
            for j in 0..entries.len() {
                if i != j {
                    let d = distance(&entries[i], &entries[j]);
                    counts
                        .entry(d)
                        .or_insert((entries[i].clone(), entries[j].clone()));
                }
            }
        }
        counts[counts.keys().min().unwrap()].clone()
    }
}

pub fn run() {
    let filename = "inputs/02/input.txt";
    let contents: String = utils::read_input(&filename);
    let entries = utils::str_to_trimmed_lines_vec(&contents);
    let closest = part2::get_closest_pair(&entries);
    let in_common = part2::letters_in_common(&closest.0, &closest.1);

    println!("Checksum: {:?}", part1::checksum(&entries));
    println!("Closest: {:?}", closest);
    println!("Chars in common: {:?}", in_common);
}

#[cfg(test)]
mod tests {
    use super::part1::*;
    use crate::day02::part2;
    use crate::utils;

    #[test]
    fn test_part1() {
        assert_eq!(count_letters("lol").get(&'l'), Some(&2));
        assert_eq!(count_letters("lol").get(&'o'), Some(&1));
        assert_eq!(count_letters("lol").get(&'b'), None);

        let counts = count_letters("abcdde");
        assert_eq!(count_repetitions(&counts, 1), 4);
        assert_eq!(count_repetitions(&counts, 2), 1);

        let contents = r#"abcdef
            bababc
            abbcde
            abcccd
            aabcdd
            abcdee
            ababab"#;
        let entries = utils::str_to_trimmed_lines_vec(contents);
        assert_eq!(checksum(&entries), 12)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::distance("fghij", "fghij"), 0);
        assert_eq!(part2::distance("fghij", "fguij"), 1);
        assert_eq!(part2::distance("fghix", "fguij"), 2);
        assert_eq!(part2::distance("abcde", "fghij"), 5);

        let contents = r#"abcde
            fghij
            klmno
            pqrst
            fguij
            axcye
            wvxyz"#;
        let entries = utils::str_to_trimmed_lines_vec(contents);
        let best = part2::get_closest_pair(&entries);
        assert_eq!(best, ("fghij".to_string(), "fguij".to_string()));
        assert_eq!(
            part2::letters_in_common(&best.0, &best.1),
            "fgij".to_string()
        );
    }
}
