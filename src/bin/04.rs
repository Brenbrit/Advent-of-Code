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
