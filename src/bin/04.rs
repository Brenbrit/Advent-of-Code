/*
--- Day 4: Camp Cleanup ---

Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.

However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

For example, consider the following list of section assignment pairs:

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8

For the first few pairs, this list means:

    Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
    The Elves in the second pair were each assigned two sections.
    The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.

This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:

.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8

Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

In how many assignment pairs does one range fully contain the other?

 */

/*
--- Part Two ---

It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.

In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:

    5-7,7-9 overlaps in a single section, 7.
    2-8,3-7 overlaps all of the sections 3 through 7.
    6-6,4-6 overlaps in a single section, 6.
    2-6,4-8 overlaps in sections 4, 5, and 6.

So, in this example, the number of overlapping assignment pairs is 4.

In how many assignment pairs do the ranges overlap?

 */

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut fully_contained_counter: u32 = 0;

    for line in lines {
        let (job_1, job_2) = parse_line(line);

        let mut bigger_job = job_1;
        let mut smaller_job = job_2;
        if (job_2[1] - job_2[0]) > (job_1[1] - job_1[0]) {
            bigger_job = job_2;
            smaller_job = job_1;
        }
        
        if (bigger_job[0] <= smaller_job[0]) && (bigger_job[1] >= smaller_job[1]) {
            fully_contained_counter += 1;
        }

    }

    Some(fully_contained_counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut overlap_counter: u32 = 0;

    for line in lines {
        let (job_1, job_2) = parse_line(line);

        let mut first_job = job_1;
        let mut second_job = job_2;
        if second_job[0] < first_job[0] {
            first_job = job_2;
            second_job = job_1;
        }
        
        if ((second_job[0] <= first_job[1]) && second_job[0] >= first_job[0]) || ((second_job[1] <= first_job[1]) && second_job[1] >= first_job[0]) {
            overlap_counter += 1;
        }

    }

    Some(overlap_counter)
}

fn parse_line(line: &str) -> ([u32; 2], [u32; 2]) {
    let split: Vec<&str> = line.split(',').collect();
    let first_job: Vec<&str> = split.get(0).unwrap().split('-').collect();
    let second_job: Vec<&str> = split.get(1).unwrap().split('-').collect();
    
    let first_job: [u32; 2] = [
        first_job.get(0).unwrap().parse::<u32>().unwrap(),
        first_job.get(1).unwrap().parse::<u32>().unwrap()
    ];
    let second_job: [u32; 2] = [
        second_job.get(0).unwrap().parse::<u32>().unwrap(),
        second_job.get(1).unwrap().parse::<u32>().unwrap()
    ];

    (first_job, second_job)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4 as u32));
    }
}
