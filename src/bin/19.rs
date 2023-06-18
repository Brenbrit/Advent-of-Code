const PARSE_ERROR: &str = "Unable to parse input.";

struct Blueprint {
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost_ore: u32,
    obsidian_cost_clay: u32,
    geode_cost_ore: u32,
    geode_cost_obsidian: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = read_input(input)?;

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn read_input(input: &str) -> Option<Vec<Blueprint>> {
    let mut blueprints: Vec<Blueprint> = vec![];

    fn text_in_middle<'a>(line: &'a str, before: &'a str, after: &'a str) -> Option<&'a str> {
        Some(line
            .split(before)
            .collect::<Vec<&str>>()
            .get(1)?
            .split(after)
            .collect::<Vec<&str>>()
            .get(0)?)
    }

    fn numbers_on_outside(text: &str) -> (u32, u32) {
        let text_split = text
        .split(' ')
        .collect::<Vec<&str>>();

        let first_number = text_split
            .get(0)
            .expect(PARSE_ERROR)
            .parse::<u32>()
            .expect(PARSE_ERROR);

        let second_number = text_split
            .last()
            .expect(PARSE_ERROR)
            .parse::<u32>()
            .expect(PARSE_ERROR);

        (first_number, second_number)
    }

    for line in input.lines() {
        let ore_cost = text_in_middle(line, "Each ore robot costs ", " ore.")?
            .parse::<u32>()
            .expect(PARSE_ERROR);
        let clay_cost = text_in_middle(line, "Each clay robot costs ", " ore.")?
            .parse::<u32>()
            .expect(PARSE_ERROR);

        let obsidian_cost = text_in_middle(line, "Each obsidian robot costs ", " clay")?;
        let (obsidian_cost_ore, obsidian_cost_clay) = numbers_on_outside(obsidian_cost);

        let geode_cost = text_in_middle(line, "Each geode robot costs ", " obsidian")?;
        let (geode_cost_ore, geode_cost_obsidian) = numbers_on_outside(geode_cost);

        blueprints.push(Blueprint {
            ore_cost,
            clay_cost,
            obsidian_cost_ore,
            obsidian_cost_clay,
            geode_cost_ore,
            geode_cost_obsidian
        });
    }

    Some(blueprints)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(12_u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
