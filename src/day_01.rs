mod task_1 {
    fn count_increases(depths: &[i32]) -> usize {
        let pairs = depths.iter().zip(depths.iter().skip(1));
        let changes = pairs.map(|(a, b)| b - a);
        let increases = changes.filter(|c| *c > 0);

        increases.count()
    }

    #[cfg(test)]
    mod test {
        use super::count_increases;
        use std::error::Error;
        use crate::input::read_input;

        #[test]
        fn example() {
            let depths = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
            assert_eq!(count_increases(depths), 7);
        }

        #[test]
        fn task() -> Result<(), Box<dyn Error>> {
            let input = read_input("day_01_input.txt")?;
            let input: Vec<i32> = input.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;
            let increases = count_increases(&input);
            assert_eq!(increases, 1215);

            Ok(())
        }
    }
}
