use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn get_data_as_arr(filename: &str) -> Vec<String> {

    let mut result: Vec<String> = Vec::new();

    let mut resource_path = String::from("resources/");
    resource_path.push_str(filename);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(resource_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(data_line) = line {
                result.push(data_line);
            }
        }
    }

    return result;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}