use aoc_util::get_seed_data;

fn main() {
    let input = get_seed_data().expect("Could not load values");

    let stream: Vec<char> = input[0].chars().collect();
    let result: usize = find_start_of_packet_marker(stream.clone());
    let result2: usize = find_start_of_message_marker(stream);
    println!("part1: {:?}", result);
    println!("part2: {:?}", result2);
}

fn no_duplicates_exists(part: Vec<char>) -> bool {
    let cpy = part.clone();
    for c in part {
        if cpy.iter().filter(|&n| *n == c).count() > 1 {
            return false;
        }
    }
    return true;
}

fn find_marker(stream: Vec<char>, msg_len: usize) -> usize {
    let gap: usize = msg_len - 1;
    for i in gap..stream.len() {
        let part: Vec<char> = stream[(i - gap)..i + 1].iter().cloned().collect();
        if no_duplicates_exists(part) {
            return i + 1; //index + 1 since its zero based
        }
    }
    return 0;
}

fn find_start_of_packet_marker(stream: Vec<char>) -> usize {
    return find_marker(stream, 4);
}

fn find_start_of_message_marker(stream: Vec<char>) -> usize {
    return find_marker(stream, 14);
}
#[cfg(test)]
mod tests {
    // super::* makes sure the methods in the non-test section are available to test
    use super::*;

    #[test]
    fn test_input_parsing_test() {
        let result0 = no_duplicates_exists(['a', 'b', 'c', 'd', 'a'].to_vec());
        assert_eq!(result0, false);

        let result1 = no_duplicates_exists(['a', 'b', 'c', 'd', 'e'].to_vec());
        assert_eq!(result1, true);
    }

    fn run_packet_marker_test(input: String, expected: usize) {
        let stream: Vec<char> = input.chars().collect();

        let result: usize = find_start_of_packet_marker(stream);
        assert_eq!(result, expected);
    }
    fn run_message_marker_test(input: String, expected: usize) {
        let stream: Vec<char> = input.chars().collect();

        let result: usize = find_start_of_message_marker(stream);
        assert_eq!(result, expected);
    }
    #[test]
    fn case_1() {
        run_packet_marker_test("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 5);
    }
    #[test]
    fn case_2() {
        run_packet_marker_test("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 6);
    }

    #[test]
    fn case_3() {
        run_packet_marker_test("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 10);
    }
    #[test]
    fn case_4() {
        run_packet_marker_test("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 11);
    }
    #[test]
    fn case_5() {
        run_message_marker_test("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 19);
    }
}
