use rayon::prelude::*;
use regex::Regex;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct OpState {
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    ores: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
}

impl PartialEq for OpState {
    fn eq(&self, other: &Self) -> bool {
        self.ore_robots == other.ore_robots
            && self.clay_robots == other.clay_robots
            && self.obsidian_robots == other.obsidian_robots
            && self.geode_robots == other.geode_robots
            && self.ores == other.ores
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geodes == other.geodes
    }
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    obsidian_robot_ore_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
}

struct BluePrintAnalysis {
    quality_level: i32,
    geodes: i32,
}

enum Choices {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
    Nothing,
}

fn main() {
    let blueprints = parse_blueprints();

    let part_1: i32 = blueprints
        .par_iter()
        .map(|blueprint| get_max_geodes(&blueprint, 24).quality_level)
        .sum();

    println!("part 1: {}", part_1);

    let part_2: i32 = blueprints[..3]
        .par_iter()
        .map(|blueprint| get_max_geodes(&blueprint, 32).geodes)
        .product();

    println!("part 2: {}", part_2);
}

fn get_max_geodes(blueprint: &Blueprint, minutes: usize) -> BluePrintAnalysis {
    let initial_state = OpState {
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
        ores: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
    };

    let mut level = 0;

    let mut choices = Vec::new();

    let mut current_geode_count = Vec::new();

    let mut states: Vec<OpState> = Vec::new();

    current_geode_count.push(0);

    choices.push(find_choices(&blueprint, &initial_state));

    states.push(initial_state);

    loop {
        if choices[level].len() == 0 {
            if level == 0 {
                break;
            }
            level -= 1;
            states.pop();
            choices.pop();
            continue;
        }

        let choice = choices[level].pop().unwrap();

        let state = states[level];

        let new_state = process_choice(&blueprint, &state, &choice);

        if current_geode_count.len() <= level + 1 {
            current_geode_count.push(new_state.geodes);
        } else if new_state.geodes < current_geode_count[level + 1] {
            if level == 0 {
                break;
            }
            level -= 1;
            states.pop();
            choices.pop();
            continue;
        } else if new_state.geodes > current_geode_count[level + 1] {
            current_geode_count[level + 1] = new_state.geodes;
        }

        if level + 1 == minutes {
            level -= 1;
            states.pop();
            choices.pop();
        } else if current_geode_count[level + 1] > new_state.geodes {
            if level == 0 {
                break;
            }
            level -= 1;
            states.pop();
            choices.pop();
        } else {
            let new_choices = find_choices(&blueprint, &new_state);
            level += 1;
            choices.push(new_choices);
            states.push(new_state);
        }
    }
    let mut max_geodes = 0;

    for geodes in current_geode_count {
        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    return BluePrintAnalysis {
        quality_level: blueprint.id * max_geodes,
        geodes: max_geodes,
    };
}

fn process_choice(blueprint: &Blueprint, state: &OpState, choice: &Choices) -> OpState {
    let mut new_state = state.clone();

    new_state.ores += state.ore_robots;
    new_state.clay += state.clay_robots;
    new_state.obsidian += state.obsidian_robots;
    new_state.geodes += state.geode_robots;

    match choice {
        Choices::OreRobot => {
            new_state.ore_robots += 1;
            new_state.ores -= blueprint.ore_robot_ore_cost;
        }
        Choices::ClayRobot => {
            new_state.clay_robots += 1;
            new_state.ores -= blueprint.clay_robot_ore_cost;
        }
        Choices::ObsidianRobot => {
            new_state.obsidian_robots += 1;
            new_state.ores -= blueprint.obsidian_robot_ore_cost;
            new_state.clay -= blueprint.obsidian_robot_clay_cost;
        }
        Choices::GeodeRobot => {
            new_state.geode_robots += 1;
            new_state.ores -= blueprint.geode_robot_ore_cost;
            new_state.obsidian -= blueprint.geode_robot_obsidian_cost;
        }
        Choices::Nothing => {}
    }

    return new_state;
}

fn find_choices(blueprint: &Blueprint, state: &OpState) -> Vec<Choices> {
    let mut choices: Vec<Choices> = Vec::new();

    if state.obsidian >= blueprint.geode_robot_obsidian_cost
        && state.ores >= blueprint.geode_robot_ore_cost
    {
        choices.push(Choices::GeodeRobot);
        return choices;
    }

    if state.ores >= blueprint.obsidian_robot_ore_cost
        && state.clay >= blueprint.obsidian_robot_clay_cost
    {
        choices.push(Choices::ObsidianRobot);
    }

    if blueprint.obsidian_robot_clay_cost > state.clay_robots
        && state.ores >= blueprint.clay_robot_ore_cost
    {
        choices.push(Choices::ClayRobot);
    }

    if blueprint
        .ore_robot_ore_cost
        .max(blueprint.geode_robot_ore_cost)
        .max(blueprint.obsidian_robot_ore_cost)
        .max(blueprint.clay_robot_ore_cost)
        > state.ore_robots
        && state.ores >= blueprint.ore_robot_ore_cost
    {
        choices.push(Choices::OreRobot);
    }

    choices.push(Choices::Nothing);

    return choices;
}

fn parse_blueprints() -> Vec<Blueprint> {
    let content = fs::read_to_string("./input.txt").expect("failed to read file");

    let blueprint_lines = content.split("\n");

    let mut blueprints = Vec::new();

    for blueprint_line in blueprint_lines {
        let re = Regex::new("Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();

        let id = re
            .captures(blueprint_line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let ore_robot_cost = re
            .captures(blueprint_line)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let clay_robot_cost = re
            .captures(blueprint_line)
            .unwrap()
            .get(3)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let obsidian_robot_ore_cost = re
            .captures(blueprint_line)
            .unwrap()
            .get(4)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let obsidian_robot_clay_cost = re
            .captures(blueprint_line)
            .unwrap()
            .get(5)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let geode_robot_ore_cost = re
            .captures(blueprint_line)
            .unwrap()
            .get(6)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let geode_robot_obsidian_cost = re
            .captures(blueprint_line)
            .unwrap()
            .get(7)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();

        let blueprint = Blueprint {
            id,
            ore_robot_ore_cost: ore_robot_cost,
            clay_robot_ore_cost: clay_robot_cost,
            obsidian_robot_clay_cost,
            obsidian_robot_ore_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        };

        blueprints.push(blueprint);
    }

    return blueprints;
}
