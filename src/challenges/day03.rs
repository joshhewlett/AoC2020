use crate::utilities::file;

pub fn run() {
    // Parse map
    let data_as_string = file::get_data_as_arr("day_03/input.txt");
    let map: Vec<Vec<bool>> = parse_map(&data_as_string);

    // Collect results
    let part1_result = survey_map(&map, 3, 1);
    let part2_tree_counts: Vec<usize> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|x| survey_map(&map, x.0, x.1))
        .collect();
    let mut part2_result: usize = 1;
    for i in part2_tree_counts {
        part2_result *= i;
    }

    // Print results
    println!("Day03/Part2 result: {}", part2_result);
    println!("Day03/Part1 result: {}", part1_result);
}

/**
 * Parses the file data into a 2D Vector containing booleans. A value is true if the respective
 * map location contains a tree and false otherwise
 */
fn parse_map(data: &Vec<String>) -> Vec<Vec<bool>> {
    let width = data[0].chars().count();
    let height = data.iter().count();

    let mut result: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for i in 0..(height) {
        for j in 0..(width) {
            if data[i].chars().nth(j).unwrap() == '#' {
                result[i][j] = true;
            }
        }
    }

    return result;
}

/**
 * Traverses the map with the given slope and returns the amount of
 * trees encountered. 'right' is represented as an isize with the sign
 * representing direction (negative is left)
 */
fn survey_map(map: &Vec<Vec<bool>>, right: isize, down: usize) -> usize {
    // Calculate usize representation of 'right'
    let horizontal: usize = if right < 0 {
        map.len() + right.abs() as usize
    } else {
        right.abs() as usize
    };
    // Find how many trees are encountered
    let mut count_of_trees: usize = 0;
    let mut position: (usize, usize) = (0, 0);
    while position.0 < map.len() {
        if map[position.0][position.1] == true {
            count_of_trees += 1;
        }

        position.0 += down;
        position.1 = (position.1 + horizontal) % map[0].len();
    }

    return count_of_trees;
}
