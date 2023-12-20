use std::collections::{HashMap, VecDeque};

aoc2023::main!("../../assets/day20.txt");

type Pulses = Vec<((String, String), bool)>;
type Modules<'a> = HashMap<&'a str, (&'a str, &'a str, Vec<&'a str>)>;
type FlipStates<'a> = HashMap<&'a str, bool>;
type ConjStates<'a> = HashMap<&'a str, HashMap<&'a str, bool>>;

fn prepare(input: &str) -> (Modules, FlipStates, ConjStates) {
    let mut modules: Modules = HashMap::new();

    // Parse input
    for line in input.lines() {
        let (module, connections) = line.split_once(" -> ").unwrap();
        let connections = connections.split(", ").collect::<Vec<_>>();

        let (typ, name) = module.split_at(1);
        modules.insert(name, (name, typ, connections));
    }

    let mut flip_states: FlipStates = HashMap::new();
    let mut conj_states: ConjStates = HashMap::new();

    // Fill flip states
    let flipflops = modules.values().filter(|(_, typ, _)| *typ == "%");
    for (name, _, _) in flipflops {
        flip_states.insert(*name, false);
    }

    // Fill conjunction memory state
    let conjuctions = modules.values().filter(|(_, typ, _)| *typ == "&");
    for (name, _, _) in conjuctions {
        let sources = modules
            .iter()
            .filter(|(_, (_, _, cons))| cons.contains(name));
        for (src, _) in sources {
            conj_states.entry(*name).or_default().insert(*src, false);
        }
    }

    (modules, flip_states, conj_states)
}

fn part1(input: &str) -> u32 {
    let (modules, mut flip_states, mut conj_states) = prepare(input);
    let (mut lows, mut highs) = (0, 0);

    // Run simulation
    for _ in 0..1000 {
        let ((nh, nl), _) = press_button(&modules, &mut flip_states, &mut conj_states);
        highs += nh;
        lows += nl;
    }

    highs * lows
}

fn part2(input: &str) -> u64 {
    if cfg!(test) {
        return 0;
    }

    let (modules, mut flip_states, mut conj_states) = prepare(input);

    // Should be only one conjuction with rx as connection
    let rx_parent = modules
        .iter()
        .find(|(_, (_, _, cons))| cons.contains(&"rx"))
        .unwrap();

    println!("rx parent: {}", rx_parent.0);

    // Should be exactly 4 conjunctions, used for lcm calculation
    let parent_parents = modules
        .values()
        .filter(|(_, _, c)| c.contains(rx_parent.0))
        .map(|(n, _, _)| *n)
        .collect::<Vec<_>>();

    println!("parent parents: {:?}", parent_parents);

    // Contains the cycles until high for each parent
    let mut first_highs = [0; 4];

    'outer: for n in 1.. {
        let (_, pulses) = press_button(&modules, &mut flip_states, &mut conj_states);

        for ((src, _), puls) in pulses {
            if puls && parent_parents.contains(&src.as_str()) {
                for (i, name) in parent_parents.iter().enumerate() {
                    if *name == src && first_highs[i] == 0 {
                        println!("{} is high after {} cycles", name, n);
                        first_highs[i] = n;

                        // Check if all are set
                        if first_highs.iter().all(|n| *n != 0) {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    // Calculate lcm
    first_highs.into_iter().fold(1, aoc2023::lcm)
}

fn press_button(
    modules: &Modules,
    flip_states: &mut FlipStates,
    conj_states: &mut ConjStates,
) -> ((u32, u32), Pulses) {
    let (mut lows, mut highs) = (0, 0);

    let mut pulses = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((("", "roadcaster"), false));

    while let Some(((src, dest), pulse)) = queue.pop_front() {
        // Only used for part 2, not needed for part 1
        pulses.push(((src.to_string(), dest.to_string()), pulse));
        //println!("{src} -{pulse}> {dest}");

        // Count pulse
        if pulse {
            highs += 1;
        } else {
            lows += 1;
        }

        // Forward pulse
        if let Some((name, typ, connections)) = modules.get(dest) {
            match *typ {
                "%" => {
                    let current_state = flip_states.get_mut(*name).unwrap();

                    // Flip state on low pulse
                    // High pulses are ignored
                    if !pulse {
                        *current_state = !*current_state;
                        for connection in connections {
                            queue.push_back(((name, connection), *current_state));
                        }
                    }
                }
                "&" => {
                    let states = conj_states.get_mut(*name).unwrap();
                    *states.get_mut(src).unwrap() = pulse;

                    // Forward high pulse if any connection is false
                    // Otherwise forward low pulse
                    let not_same = states.values().any(|p| !*p);

                    // if matches!(*name, "mp" | "ng" | "qt" | "qb" if not_same) {
                    //     println!("{} is high", name);
                    // }

                    for connection in connections {
                        queue.push_back(((name, connection), not_same));
                    }
                }
                "b" => {
                    // Forward pulse to all connections (broadcast)
                    for connection in connections {
                        queue.push_back(((name, connection), pulse));
                    }
                }
                _ => unreachable!("Unknown type: {}", typ),
            }
        }
    }

    ((lows, highs), pulses)
}

aoc2023::test!(
    "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
",
    11687500,
    0
);
