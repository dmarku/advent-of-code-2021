mod task_01 {
    fn count_increases(depths: &[i32]) -> usize {
        let pairs = depths.iter().zip(depths.iter().skip(1));
        let changes = pairs.map(|(a, b)| b - a);
        let increases = changes.filter(|c| *c > 0);

        increases.count()
    }

    #[cfg(test)]
    mod test {
        use super::count_increases;

        #[test]
        fn example() {
            let depths = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
            assert_eq!(count_increases(depths), 7);
        }

        use std::error::Error;
        #[test]
        fn task() -> Result<(), Box<dyn Error>> {
            use std::fs;
            use std::path::PathBuf;

            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("day_01_input.txt");
            let input = fs::read_to_string(d)?;
            let input: Vec<i32> = input.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;
            let increases = count_increases(&input);
            assert_eq!(increases, 1215);

            Ok(())
        }
    }
}
