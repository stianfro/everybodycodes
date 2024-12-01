fn main() {
    let p1 = part_one("AACBCCAACCCACABBAABBAABBBBCCCCABBACABCABCCBBCAAACBBACABAAACCCBBBACBCCACBCCACAACBAACCBCACBCABAABBACBCAACBBCBBBABCCBCCBCAAACCCCABBABABAABBABACCAABBCCABAABCBABCCACCBACCBABBCAAACBABBAACCCCCBACABCAABCCBBBCAAAACCBCABCCBBBAAABABBCAACCABACCABBACCBAABCCABCACBBBACBACACCAACACCACCBBCCBACACBCCBCCBBCCACAAABABBCCBBCCABCACCABACABBBBABBACBCBBAAABBBCCCAABCCCAAAACCBCAABAABBCCACBCCCBCBBBBABBACCBCCCACCAAABBAACACABBAACAACACBABABCAABCBAABAABBACACABABABCACCBCCAACBAACCCCBBBBCBAAABAACBBCBBAAACBCCAAAAABBCACCCCABBBABCBABBBABBCCBACABCACCCACBBBABACBABABACCBAACBCABBACCCBBCABCCAABAAAACCBABABABCCBABBABCCCBCCABAACACCACBCAACAACCCAABABABBACCCCBABBCCBCBCBACBABCBBBBACCACBACAABBBCACCACBCABBCBCCACCBACBBACBBCCCAACABCBCCCCCBBCCABCACBACBCCABCAABCAACBBABAACBBACCAAAACACCABCBBCABCCAABAABBCABCCAAACABCBBACCCCBBBACBCCBAACBABAACBBBCCBBCCBBABBACBBAABCBBCBAAAABACBCBABCBCBACBBBBBAAACACBCCCCBACBACCAACACABCCBBACBBABCACACCBBAABAACBBABCCCABBABCAAAAABCBCCBCAABAABCCACCCCCABBBBBCCAACBCBBCAAACBAAAAAACAABCAABBABBABCAAABBABACCAAACCAAACCACCABCCAABC");
    let p2 = part_two("AxBCDDCAxD");

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn part_one(input: &str) -> i32 {
    let list = input.split("");

    let mut potions = 0;

    for creature in list {
        if creature == "\n" {
            continue;
        }

        let mut potions_needed = 0;

        if creature == "A" {
            potions_needed = 0
        }
        if creature == "B" {
            potions_needed = 1
        }
        if creature == "C" {
            potions_needed = 3
        }

        potions += potions_needed
    }

    potions
}

fn part_two(input: &str) -> i32 {
    let list = input.split("");

    let mut creatures: Vec<&str> = Vec::new();

    for creature in list {
        if creature == "\n" || creature == "" {
            continue;
        }

        creatures.push(creature);
    }

    let mut pairs: Vec<Vec<&str>> = Vec::new();

    for i in 0..creatures.len() {
        if i % 2 == 0 {
            let c0 = creatures[i];
            let c1 = creatures[i + 1];

            let p = vec![c0, c1];

            pairs.push(p);
        }
    }

    for pair in pairs {
        let p0 = pair[0];
        let p1 = pair[1];

        println!("{p0}{p1}")
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("ABBAC");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("AxBCDDCAxD");
        assert_eq!(result, 28)
    }
}
