use crate::utilities::file;

static SUM_TO_FIND: i32 = 2020;

pub fn run() {
    let data_as_strings = file::get_data_as_arr("day_01/input.txt");
    let mut data: Vec<i32> = data_as_strings
        .iter()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    data.sort();

    // Get result
    let part_1_result = find_addend_indexes(&data, 2);
    let part_2_result = find_addend_indexes(&data, 3);

    // Output results
    if part_1_result.is_some() {
        println!("=== Challenge Part 1 result: ");

        print!("Addends: ");
        let mut multiplied_value: i32 = 1;
        for i in part_1_result.unwrap() {
            print!("{} ", data[i]);
            multiplied_value *= data[i];
        }
        println!("\nProduct of addends: {}", multiplied_value);
    } else {
        println!("Challenge Part 1 result: Result not found");
    }

    println!("");
    if part_2_result.is_some() {
        println!("=== Challenge Part 2 result: ");

        print!("Addends: ");
        let mut multiplied_value: i32 = 1;
        for i in part_2_result.unwrap() {
            print!("{} ", data[i]);
            multiplied_value *= data[i];
        }
        println!("\nProduct of addends: {}", multiplied_value);
    } else {
        println!("Challenge Part 2 result: Result not found");
    }
}

fn find_addend_indexes(array_to_search: &Vec<i32>, how_many_addends: u8) -> Option<Vec<usize>> {
    return find_addend_indexes_recursive(
        array_to_search,
        how_many_addends - 1,
        &mut Vec::new(),
        0,
    );
}

/**
 * Find (depth + 1) number of indexes who's values in array_to_search are added together, equal
 * SUM_TO_FIND. Algorithm is pretty inefficient/brute-force. Possible permutations of algorithm is
 * "(array_to_search.len()) choose (depth + 1)" or:
 *
 * When n = array_to_search.len() and r = depth + 1:
 * (r + n - 1)!/(r!(n - 1))!
 *
 * (It's a lot)
 */
fn find_addend_indexes_recursive(
    array_to_search: &Vec<i32>,
    depth: u8,
    result: &mut Vec<usize>,
    current_index: usize,
) -> Option<Vec<usize>> {
    result.push(current_index);

    // The first (depth) values in result has exhausted their options. Return None
    if depth == 0 && current_index == array_to_search.len() {
        result.pop();
        return None;
    }
    // Solution was found. Return result
    else if depth == 0 && get_sum_of_arr(array_to_search, result) == SUM_TO_FIND {
        return Some(result.to_vec());
    }
    // Solution was not found in this current iteration. Increment current_index and try again at same depth
    else if depth == 0 && current_index < (array_to_search.len() - 1) {
        result.pop();
        return find_addend_indexes_recursive(array_to_search, depth, result, current_index + 1);
    }
    // Dig deeper and find solution
    else if depth > 0 && current_index < (array_to_search.len() - depth as usize) {
        let sub_result =
            find_addend_indexes_recursive(array_to_search, depth - 1, result, current_index + 1);

        // If sub_result wasn't found, increment current_index at current depth and try again
        if sub_result.is_none() {
            result.pop();
            return find_addend_indexes_recursive(
                array_to_search,
                depth,
                result,
                current_index + 1,
            );
        }

        // Solution was found. Return sub_result
        return sub_result;
    }

    // No solution was found at current depth/current_index
    result.pop();
    return None;
}

// Get sum of values at indexes' positions in array
fn get_sum_of_arr(array: &Vec<i32>, indexes: &Vec<usize>) -> i32 {
    let mut sum = 0;
    for i in indexes {
        sum += array[*i];
    }

    return sum;
}
