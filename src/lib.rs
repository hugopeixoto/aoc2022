#[macro_export]
macro_rules! day {
    ( $x:ident, $bench:ident ) => {
        use std::fs::read_to_string;

        pub fn main() {
            let input = read_to_string(
                format!("inputs/{}.in", stringify!($x)),
            ).unwrap();

            let (p1, p2) = $x(input);

            println!("{}", p1);
            println!("{}", p2);
        }

        #[bench]
        fn $bench(b: &mut test::Bencher) {
            let input = read_to_string(
                format!("inputs/{}.in", stringify!($x)),
            ).unwrap();

            b.iter(|| $x(input.clone()));
        }
    };
}
