use std::collections::{HashMap, VecDeque};

aoc2023::main!("../../assets/day20.txt");

fn part1(input: &str) -> u32 {
    let mut modules = HashMap::new();

    for line in input.lines() {
        let (module, connections) = line.split_once(" -> ").unwrap();
        let connections = connections.split(", ").collect::<Vec<_>>();

        let (typ, name) = module.split_at(1);
        modules.insert(name, (name, typ, connections));
    }

    let (mut lows, mut highs) = (0, 0);

    let mut flip_states = HashMap::new();
    let mut conj_states = HashMap::new();

    // Fill conjunction memory state
    let conjuctions = modules.values().filter(|(_, typ, _)| *typ == "&");
    for (name, _, _) in conjuctions {
        let sources = modules
            .iter()
            .filter(|(_, (_, _, cons))| cons.contains(name));
        for (src, _) in sources {
            conj_states
                .entry(name)
                .or_insert(HashMap::new())
                .insert(*src, false);
        }
    }

    // Run simulation
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back((("", "roadcaster"), false));

        while let Some(((src, dest), pulse)) = queue.pop_front() {
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
                        let current_state = flip_states.entry(name).or_insert(false);
                        
                        // Flip state on low pulse
                        // High pulses are ignored
                        if !pulse {
                            *current_state = !*current_state;
                            for connection in connections {
                                println!("{} -{current_state}> {}", name, connection);
                                queue.push_back(((name, connection), *current_state));
                            }
                        }
                    }
                    "&" => {
                        let states = conj_states.entry(name).or_insert(HashMap::new());
                        states.insert(src, pulse);

                        // Forward high pulse if any connection is false
                        // Otherwise forward low pulse
                        let not_same = states.values().any(|p| !*p);

                        for connection in connections {
                            println!("{} -{not_same}> {}", name, connection);
                            queue.push_back(((name, connection), not_same));
                        }
                    }
                    "b" => {
                        // Forward pulse to all connections (broadcast)
                        for connection in connections {
                            println!("{} -{pulse}> {}", name, connection);
                            queue.push_back(((name, connection), pulse));
                        }
                    }
                    _ => unreachable!("Unknown type: {}", typ),
                }
            }
        }
    }

    println!("{} {}", lows, highs);

    highs * lows
}

fn part2(input: &str) -> u32 {
    0
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
