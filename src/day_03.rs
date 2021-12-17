mod task_01 {

    fn calculate_power_consumption(input: &str) -> Result<i32, &str> {
        let mut gamma_rate = 0;
        let mut epsilon_rate = 0;

        let digit_count = input.lines().nth(0).expect("input is empty").len();
        let line_count = input.lines().count();

        let mut ones = Vec::with_capacity(digit_count);
        ones.resize(digit_count, 0);

        for line in input.lines() {
            for (i, _) in line.chars().enumerate().filter(|(_, c)| *c == '1') {
                if let Some(count) = ones.get_mut(i) {
                    *count += 1;
                }
            }
        }

        for ones_count in ones {
            gamma_rate <<= 1;
            epsilon_rate <<= 1;
            if ones_count > (line_count - ones_count) {
                gamma_rate += 1;
            } else {
                epsilon_rate += 1;
            }
        }

        Ok(gamma_rate * epsilon_rate)
    }

    #[cfg(test)]
    mod test {
        use super::calculate_power_consumption;
        use crate::input::read_input;

        #[test]
        fn example() {
            let input = read_input("inputs/day_03/example.txt").unwrap();
            let pc = calculate_power_consumption(&input).unwrap();
            assert_eq!(pc, 198);
        }

        #[test]
        fn task() {
            let input = read_input("inputs/day_03/input.txt").unwrap();
            let pc = calculate_power_consumption(&input).unwrap();
            assert_eq!(pc, 1071734);
        }
    }
}

mod task_02 {
    // implementation goes here

    #[cfg(test)]
    mod test {
        #[test]
        #[ignore]
        fn example() {}

        #[test]
        #[ignore]
        fn task() {
            // let solution = /* ... */

            // output result for input to AoC website
            // println!("result: {}", result);

            // once the solution is correct, assert it in test results
            // assert_eq!(solution, /* expected result */);
        }
    }
}
