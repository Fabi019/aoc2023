use std::collections::HashMap;

aoc2023::main!("../../assets/day19.txt");

fn part1(input: &str) -> u32 {
    let (rules, parts) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (id, rules) = line.split_once('{').unwrap();

            let rules = &rules[..rules.len() - 1]
                .split(',')
                .map(|rule| {
                    if let Some((eq, target)) = rule.split_once(':') {
                        let part = &eq[0..1];
                        let sign = &eq[1..2];
                        let num = &eq[2..].parse::<u32>().unwrap();
                        (part, sign, *num, target)
                    } else {
                        ("", "", 0u32, rule)
                    }
                })
                .collect::<Vec<_>>();

            (id.to_owned(), rules.to_owned())
        })
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(|line| {
            let line = &line[1..line.len() - 1]; // remove { and }
            line.split(',')
                .map(|part| {
                    let (id, num) = part.split_once('=').unwrap();
                    (id, num.parse::<u32>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut accepted = vec![];

    for partlist in parts {
        let mut current_ruleset = &rules["in"];

        'outer: loop {
            'ruleset: for &(target, sign, num, next) in current_ruleset {
                // No rule matched, default case
                if target.is_empty() {
                    if next == "A" {
                        accepted.push(partlist.clone());
                        break 'outer;
                    } else if next == "R" {
                        break 'outer;
                    }
                    current_ruleset = &rules[next];
                    break;
                }

                for (part, rating) in &partlist {
                    if part != &target {
                        continue;
                    }

                    let next = match sign {
                        "<" if rating < &num => next,
                        ">" if rating > &num => next,
                        _ => continue,
                    };

                    if next == "A" {
                        accepted.push(partlist.clone());
                        break 'outer;
                    } else if next == "R" {
                        break 'outer;
                    }
                    current_ruleset = &rules[next];
                    break 'ruleset;
                }
            }
        }
    }

    accepted
        .into_iter()
        .flat_map(|parts| parts.into_iter().map(|(_, num)| num))
        .sum()
}

fn part2(input: &str) -> u64 {
    let (rules, _) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (id, rules) = line.split_once('{').unwrap();

            let rules = &rules[..rules.len() - 1]
                .split(',')
                .map(|rule| {
                    if let Some((eq, next)) = rule.split_once(':') {
                        let part = eq[0..1].chars().next().unwrap();
                        let sign = eq[1..2].to_owned();
                        let num = &eq[2..].parse::<u32>().unwrap();
                        (part, sign, *num, next.to_owned())
                    } else {
                        (' ', "".to_owned(), 0u32, rule.to_owned())
                    }
                })
                .collect::<Vec<_>>();

            (id.to_owned(), rules.to_owned())
        })
        .collect::<HashMap<_, _>>();

    let initial: Possibilities = HashMap::from_iter([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);

    split_recursive(rules, "in", initial)
}

type Rule = (char, String, u32, String);
type Possibilities = HashMap<char, (u32, u32)>;

fn split_range(
    current: Possibilities,
    rule: Rule,
) -> (Option<Possibilities>, Option<Possibilities>) {
    let (mut matching, mut failing) = (current.clone(), current.clone());
    let (target, sign, num, _) = rule;

    match sign.as_str() {
        "<" => {
            let (min, max) = current[&target];
            if max < num {
                return (Some(current), None);
            }
            if min >= num {
                return (None, Some(current));
            }
            matching.insert(target, (min, num - 1));
            failing.insert(target, (num, max));
        }
        ">" => {
            let (min, max) = current[&target];
            if min > num {
                return (Some(current), None);
            }
            if max <= num {
                return (None, Some(current));
            }
            matching.insert(target, (num + 1, max));
            failing.insert(target, (min, num));
        }
        _ => return (None, None),
    }

    (Some(matching), Some(failing))
}

fn split_recursive(rules: HashMap<String, Vec<Rule>>, rule: &str, current: Possibilities) -> u64 {
    if rule == "A" {
        // Calculate the number of possibilities
        return current
            .values()
            .map(|(min, max)| (max - min + 1) as u64)
            .product();
    } else if rule == "R" {
        return 0;
    }

    let mut possibilities = 0;
    let mut current = Some(current);

    for r @ (target, _, _, next) in &rules[rule] {
        if let Some(c) = current {
            // Default case
            if target == &' ' {
                possibilities += split_recursive(rules.clone(), next, c);
                break;
            }

            let (matching, failed) = split_range(c, r.clone());

            if let Some(m) = matching {
                possibilities += split_recursive(rules.clone(), next, m);
            }

            current = failed;
        }
    }

    possibilities
}

aoc2023::test!(
    "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
",
    19114,
    167409079868000
);
