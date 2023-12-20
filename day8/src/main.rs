use std::collections::HashMap;

use clap::Parser;
use good_lp::{default_solver, variable, Expression, ProblemVariables, Solution, SolverModel};

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct GraphState<'a> {
    node: &'a str,
    next_instruction_idx: usize,
}

#[derive(Debug)]
struct LastZNode<'a> {
    node: GraphState<'a>,
    step_idx_observed: usize,
}

#[derive(Debug)]
struct NodeState<'a> {
    current: GraphState<'a>,
    last_znode: Option<LastZNode<'a>>,
    cycle_starts_on_step: Option<usize>,
    first_znode_observations: HashMap<GraphState<'a>, usize>,
}

fn gcd(mut fst: usize, mut snd: usize) -> usize {
    if fst > snd {
        std::mem::swap(&mut fst, &mut snd);
    }

    loop {
        snd %= fst;

        if snd == 0 {
            return fst;
        }

        std::mem::swap(&mut fst, &mut snd);
    }
}

fn lcm(nums: &[usize]) -> usize {
    return nums.iter().fold(1, |acc, &e| acc * (e / gcd(acc, e)));
}

fn walk_graph(graph: &HashMap<String, (String, String)>, instructions: &Vec<Instruction>) -> usize {
    let mut curr = "AAA";

    for (round_idx, round) in std::iter::repeat(instructions).enumerate() {
        for (step_idx, step) in round.iter().enumerate() {
            curr = match *step {
                Instruction::Left => &graph.get(curr).unwrap().0,
                Instruction::Right => &graph.get(curr).unwrap().1,
            };

            if curr == "ZZZ" {
                return round_idx * instructions.len() + step_idx + 1;
            }
        }
    }

    0
}

fn walk_graph_part2(
    graph: &HashMap<String, (String, String)>,
    instructions: &Vec<Instruction>,
) -> usize {
    // (current, (last observed Z-node, steps since last Z-node), cycle_starts_on_step,
    // first_znode_hits)
    let mut nodes: Vec<NodeState> = graph
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|val| NodeState {
            current: GraphState {
                node: val.as_str(),
                next_instruction_idx: 0,
            },
            last_znode: None,
            cycle_starts_on_step: None,
            first_znode_observations: HashMap::new(),
        })
        .collect::<Vec<_>>();

    // value is dest graph state and distance to that graph state
    let mut znode_map: HashMap<GraphState, (GraphState, usize)> = HashMap::new();

    'outer: for (round_idx, round) in std::iter::repeat(instructions).enumerate() {
        for (step_idx, step) in round.iter().enumerate() {
            let steps_so_far = round_idx * instructions.len() + step_idx + 1;

            for node in &mut nodes {
                if node.cycle_starts_on_step.is_some() {
                    continue;
                }

                let next = GraphState {
                    node: match *step {
                        Instruction::Left => &graph.get(node.current.node).unwrap().0,
                        Instruction::Right => &graph.get(node.current.node).unwrap().1,
                    }
                    .as_str(),
                    next_instruction_idx: (step_idx + 1) % instructions.len(),
                };

                node.current = next.clone();

                if next.node.ends_with('Z') {
                    if let Some(ref last_znode) = node.last_znode {
                        znode_map.insert(
                            last_znode.node.clone(),
                            (next.clone(), steps_so_far - last_znode.step_idx_observed),
                        );
                    }

                    node.last_znode = Some(LastZNode {
                        node: next.clone(),
                        step_idx_observed: steps_so_far,
                    });

                    // We discovered a cycle.
                    if znode_map.contains_key(&next) {
                        node.cycle_starts_on_step =
                            Some(*node.first_znode_observations.get(&next).unwrap());

                        continue;
                    }

                    node.first_znode_observations.insert(next, steps_so_far);
                }
            }

            if nodes.iter().all(|node| node.current.node.ends_with('Z'))
                && nodes.iter().any(|node| node.cycle_starts_on_step.is_none())
            {
                eprintln!("nodes: {nodes:#?}");
                eprintln!("znode_map: {znode_map:#?}");

                return steps_so_far;
            }

            // If every ghost hits a cycle.
            if nodes.iter().all(|node| node.cycle_starts_on_step.is_some()) {
                break 'outer;
            }
        }
    }

    // If we're here, it means every ghost hit a cycle.
    eprintln!("nodes: {nodes:#?}");
    eprintln!("znode_map: {znode_map:#?}");

    // For AoC's input, this is sufficient.
    let lcm_vals = nodes
        .iter()
        .map(|node| node.cycle_starts_on_step.unwrap())
        .collect::<Vec<_>>();
    println!("LCM result: {}", lcm(&lcm_vals));

    // This is 100% overkill for this AoC, but I want to try making a general solution using
    // linear programming / constraint solving.
    //
    // For AoC specifically, all you really need is LCM.
    let mut problem = ProblemVariables::new();

    let cycles = nodes
        .iter()
        .map(|node| {
            let mut cycle = vec![node.current.clone()];
            let mut node_vars = Vec::new();
            let mut this_expr = Expression::with_capacity(10);

            loop {
                let (next, distance) = znode_map.get(cycle.last().unwrap()).unwrap();

                let var = problem.add(variable().integer().min(0));
                node_vars.push(var);

                this_expr += var * (*distance as f64);

                if next == cycle.first().unwrap() {
                    break;
                }

                cycle.push(next.clone());
            }

            this_expr += node.cycle_starts_on_step.unwrap() as f64;

            (
                cycle.into_iter().zip(node_vars).collect::<Vec<_>>(),
                this_expr,
            )
        })
        .collect::<Vec<_>>();

    eprintln!("cycles: {cycles:#?}");

    let mut problem = problem.minimise(cycles[0].1.clone()).using(default_solver);

    for (cycle, _) in &cycles {
        for var_window in cycle.iter().map(|val| val.1).collect::<Vec<_>>().windows(2) {
            problem = problem
                .with(Into::<Expression>::into(var_window[0]).leq(var_window[1]))
                .with(Into::<Expression>::into(var_window[1]).leq(var_window[0] + 1));
        }
    }

    for cycle in cycles.iter().skip(1) {
        let expr1 = cycles[0].1.clone();
        let expr2 = cycle.1.clone();

        eprintln!("expr1: {expr1:?}, expr2: {expr2:?}");

        problem = problem.with(expr1.eq(expr2));
    }

    let solution = problem.solve().unwrap();

    solution.eval(&cycles[0].1) as usize
}

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    part2: bool,
}

fn main() {
    let args = Cli::parse();

    let mut lines = std::io::stdin().lines().map(|line| line.unwrap());

    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            other => panic!("unknown instruction {other}"),
        })
        .collect::<Vec<_>>();

    // Skip the empty line.
    lines.next().unwrap();

    let graph = lines
        .map(|line| {
            let (node, pointers) = line.split_once(" = ").unwrap();

            let (left, right) = pointers
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();

            (node.to_string(), (left.to_string(), right.to_string()))
        })
        .collect::<HashMap<_, _>>();

    if args.part2 {
        println!("{}", walk_graph_part2(&graph, &instructions));
    } else {
        println!("{}", walk_graph(&graph, &instructions));
    }
}

#[cfg(test)]
mod tests {
    use crate::gcd;
    use crate::lcm;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6);
        assert_eq!(gcd(7, 3), 1);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(2, 1), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&[2, 3]), 6);
        assert_eq!(lcm(&[2, 3, 1]), 6);
        assert_eq!(lcm(&[1, 2, 4, 8]), 8);
        assert_eq!(lcm(&[12, 18]), 36);
    }
}
