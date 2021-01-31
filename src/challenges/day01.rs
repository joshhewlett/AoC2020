use crate::utilities::file;

static SUM_TO_FIND: i32 = 2020;

pub fn run() {
    let data_as_strings = file::get_data_as_arr("day_01/input.txt");
    let mut data: Vec<i32> = data_as_strings
        .iter()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    println!("Input data size: {}", data.len());

    data.sort();

    let mut result = None;

    // Loop through data elements and try to find a counterpart
    for (i, _) in data.iter().enumerate() {
        result = find_addend_indexes(&data, i);

        if result.is_some() {
            break;
        }
    }

    // Output results
    if result.is_some() {
        println!(
            "Challenge result: {}",
            data[result.unwrap().0] * data[result.unwrap().1]
        );
    } else {
        println!("Challenge result: Result not found");
    }
}

/**
 * Returns a tuple of indexes whose values added together equals SUM_TO_FIND
 */
fn find_addend_indexes(array: &Vec<i32>, index: usize) -> Option<(usize, usize)> {
    let counterpart_addend = SUM_TO_FIND - array[index];

    let search_result = array.binary_search(&counterpart_addend);

    // If there was no result or the indexes are the same, return None
    if search_result.is_err() || search_result.ok().unwrap() == index {
        return None;
    }

    return Some((index, search_result.ok().unwrap()));
}
