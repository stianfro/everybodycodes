fn main() {
    let p1 = part_one("AACBCCAACCCACABBAABBAABBBBCCCCABBACABCABCCBBCAAACBBACABAAACCCBBBACBCCACBCCACAACBAACCBCACBCABAABBACBCAACBBCBBBABCCBCCBCAAACCCCABBABABAABBABACCAABBCCABAABCBABCCACCBACCBABBCAAACBABBAACCCCCBACABCAABCCBBBCAAAACCBCABCCBBBAAABABBCAACCABACCABBACCBAABCCABCACBBBACBACACCAACACCACCBBCCBACACBCCBCCBBCCACAAABABBCCBBCCABCACCABACABBBBABBACBCBBAAABBBCCCAABCCCAAAACCBCAABAABBCCACBCCCBCBBBBABBACCBCCCACCAAABBAACACABBAACAACACBABABCAABCBAABAABBACACABABABCACCBCCAACBAACCCCBBBBCBAAABAACBBCBBAAACBCCAAAAABBCACCCCABBBABCBABBBABBCCBACABCACCCACBBBABACBABABACCBAACBCABBACCCBBCABCCAABAAAACCBABABABCCBABBABCCCBCCABAACACCACBCAACAACCCAABABABBACCCCBABBCCBCBCBACBABCBBBBACCACBACAABBBCACCACBCABBCBCCACCBACBBACBBCCCAACABCBCCCCCBBCCABCACBACBCCABCAABCAACBBABAACBBACCAAAACACCABCBBCABCCAABAABBCABCCAAACABCBBACCCCBBBACBCCBAACBABAACBBBCCBBCCBBABBACBBAABCBBCBAAAABACBCBABCBCBACBBBBBAAACACBCCCCBACBACCAACACABCCBBACBBABCACACCBBAABAACBBABCCCABBABCAAAAABCBCCBCAABAABCCACCCCCABBBBBCCAACBCBBCAAACBAAAAAACAABCAABBABBABCAAABBABACCAAACCAAACCACCABCCAABC");
    let p2 = part_two("xABACBCCBBBxCDACABDDCCCCDDxDDCCxCxAxCAABDCCCCxADDCADCCBDCCDCAAAADxCBBBCAACxBxBCDDBDBCCACDDDCADABCABCDCABBDBDBDCAADCCCCDBACCCxCCDBCAACBAABBxADDAAADxAxCCBxAADCACxDDDABxDBBBxCDBADCADABBDCCxAAAAACBxDADBBxDBxAADCAABBBBDDBxBCBxCDCABDBCBBACAxCDDDBBDBxABBDBBxADBBABBCCBDACxBBxBCBBADADADDACBDBCADDxBBxBCDxACACDADCBADBCBBDACBxCAADAAxCCDCCDDABCDDABABACABCABABADDCADDCxAxBDCDBACBxxBDDCBBDADxADDDBADBCxxDAAxCABxDACABBBCDDBAxDACCADAxDDDBADCADADADAABDBDCCCACxBDAACACxBCBCDDADDCCCBADBAAxCDBCDCCCCxCxxCBACBBBCDCDxxDDBABBBBCBDADxCDADCADCADxBDDxACCAAABCCCxAADBDxBDDACDBxDxBCDDADBDDDBxDBBDDBDBAxCAACDCCADBDCBADABCBABDxACCxDDxCDABDCBDxCBxBDAADBADxDCABCAxDBDBADBDCDACDABAxAABDBCCCACBADBBxCCABBABxBBxACACDAAxACBAxDDDBBCBxAADCDDxBxADxBDCDBDxBDxxBABxCADADACCAACADBxBAADAAAxCBBCAABDDABAxBxBBxABBDCxDADBBBDDBDxDxBBABCxDAACABBCACxBABBBDABxABxxxBDAxCDACDDCCABCABABBDADCBDCxCxCxABCBBADBDBDAxxCxACCBxDABDACBCBADAxAACDDABCBADCBDAADCBACADCCACDDBBBDBxBDBDCDAACxDCABDCBABCDCCCCDCDCDxDCCAADABCBCBDxDCCABCCBDxBCADCDBAADxCACDDDDCDBAACBAxxxAADBAxDAAADBABxCDBCAAxDBDCDDCAADCCCBBBCDCBxBxCCCCBxAACDBBDCCBBDCBACxCBDADBBBDDADCxACCBCBBABxDDCADDBBACBCCADBCACDCCCBDABDDCDACBABADCDBDCCxCCCBCCCACADCBDxAABCADACBxDBDDCBCADDBDACBBBxDCxBCxxBDDDADBCCxxBCxABBCCBCDBACBCADDCADABACCCxABACDDxBABxBBDCCBCxAAACACABBDDABxCDBBxDxCADDCACxDACxAAxDCBBDCDBCDDADDxBxxDDCDCCCDDDCAxBDAxBDCACxxCBCADCxCDAxCABAADxBACABAAABBxBCDBCACDBDABABxCABABDDAAACCDBACDAABxAxACADACBBBCACAADDAADBBAACAADBDCCBBDCAACDABBCxBBDxAABCDBADDABACADADADAAADDABCBCxADxBDxBDBxDBCDCDCCxCxCDxxAACBDCDAAxBBDBDDBCxADDADBACAACCADABBDDCCAxDBBCABBBCDBABACCADABBxCxDBBAADBCBDABDDDADBAxAADxBCBCAACADAADCACBABDCDDBAABBBADADDDDDCBCCCDxDDCBxCDACDADCDCDAxAAACAACxBBDDDACCxDCxACAACABDCCABxCCBADxAABAACCBBxxDABBxxDCCxBBAAAxCCACACBBBCBBBBADBBxABABDBAxACCxDxBDDxDBBBDCADACCCCxxBBBCCAAABADADCDCCBBxCCBBAABCCBADAAABDBABBBDDCCxADCDAAABBAxABBAAACDCCAADBCDABBDAAxAACDCxBCDDBACDBCDADCDDCDxCABBBADDABADDCCAAADDCCDCBCCCDDCCCADDCBDBCBCCBBBxCAADxAAABBxCDxABDCCBCCADCDACABDAAACABDABxACCACDCAxxBCAACABBDxxDACxBDxCCDBBBxACADAC");

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

        potions += potions_needed(creature)
    }

    potions
}

fn part_two(input: &str) -> i32 {
    let list = input.split("");

    let mut creatures: Vec<&str> = Vec::new();

    for creature in list {
        if creature == "\n" || creature.is_empty() {
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

    let mut total_potions = 0;

    for pair in pairs {
        let p0 = pair[0].to_string();
        let p1 = pair[1];

        let pstring = p0 + p1;

        let mut potions = 0;

        for creature in pair {
            potions += potions_needed(creature);
        }

        if !pstring.contains('x') {
            potions += 2
        }

        total_potions += potions
    }

    total_potions
}

fn potions_needed(c: &str) -> i32 {
    let mut p = 0;

    if c == "A" {
        p = 0
    }
    if c == "B" {
        p = 1
    }
    if c == "C" {
        p = 3
    }
    if c == "D" {
        p = 5
    }

    p
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        input: String,
        want: i32,
    }

    #[test]
    fn test_part_one() {
        let result = part_one("ABBAC");
        assert_eq!(result, 5);
        let tests = vec![
            Test {
                input: String::from("ABBAC"),
                want: 5,
            },
            Test {
                input: String::from("BACCA"),
                want: 7,
            },
        ];

        for t in tests {
            let got = part_one(&t.input);
            assert_eq!(got, t.want)
        }
    }

    #[test]
    fn test_part_two() {
        let tests = vec![
            Test {
                input: String::from("AxBCDDCAxD"),
                want: 28,
            },
            Test {
                input: String::from("xCDAxxxBAA"),
                want: 13,
            },
        ];

        for t in tests {
            let got = part_two(&t.input);
            assert_eq!(got, t.want)
        }
    }
}
