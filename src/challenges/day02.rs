use crate::utilities::file;

#[derive(Debug)]
struct Policy {
    min: u8,
    max: u8,
    character: char,
}

#[derive(Debug)]
struct Entry {
    policy: Policy,
    password: String,
}

impl Entry {
    fn new(policy_min: u8, policy_max: u8, policy_char: char, password: String) -> Entry {
        return Entry {
            policy: Policy {
                min: policy_min,
                max: policy_max,
                character: policy_char,
            },
            password: password,
        };
    }

    fn is_valid_per_policy_1(&self) -> bool {
        // Count the occurrences
        let policy_char_occurence_count = self.password.matches(self.policy.character).count();

        return self.policy.min as usize <= policy_char_occurence_count
            && policy_char_occurence_count <= self.policy.max as usize;
    }

    fn is_valid_per_policy_2(&self) -> bool {
        let pos_1_char: char = self
            .password
            .chars()
            .nth(self.policy.min as usize - 1)
            .unwrap();
        let pos_2_char: char = self
            .password
            .chars()
            .nth(self.policy.max as usize - 1)
            .unwrap();

        return (pos_1_char == self.policy.character && pos_2_char != self.policy.character)
            || (pos_1_char != self.policy.character && pos_2_char == self.policy.character);
    }
}

pub fn run() {
    let data = file::get_data_as_arr("day_02/input.txt");

    let entries: Vec<Entry> = data.iter().map(self::parse_entry).collect();

    // for i in entries.iter() {
    //     println!("{:#?}", i);
    // }

    let policy_1_valid_count: usize = entries
        .iter()
        .map(Entry::is_valid_per_policy_1)
        .filter(|x| *x)
        .count();
    let policy_2_valid_count: usize = entries
        .iter()
        .map(Entry::is_valid_per_policy_2)
        .filter(|x| *x)
        .count();

    println!("=== Challenge Day02/Part1 Result: {}", policy_1_valid_count);
    println!("=== Challenge Day02/Part2 Result: {}", policy_2_valid_count);
}

fn parse_entry(entry: &String) -> Entry {
    let policy_password: Vec<String> = entry.split(": ").map(String::from).collect::<Vec<String>>();

    // Parse policy values
    let min: u8 = policy_password[0].split("-").collect::<Vec<&str>>()[0]
        .parse::<u8>()
        .unwrap();
    let max: u8 = policy_password[0].split("-").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()[0]
        .parse::<u8>()
        .unwrap();
    let character: char = policy_password[0].split("-").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .parse::<char>()
        .unwrap();

    // Parse password
    let password: String = String::from(&policy_password[1]);

    return Entry::new(min, max, character, password);
}
