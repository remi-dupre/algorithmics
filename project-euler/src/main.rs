#[macro_use]
pub mod util;

use colored::Colorize;

macro_rules! run_problem {
    ( $run_all: expr, $pb: ident ) => {{
        use std::time::Instant;

        let start = Instant::now();
        let soluce = $pb::solve();
        let elapsed = start.elapsed();

        println!("{}: {:<20} ({:.0?})", stringify!($pb), soluce, elapsed);
    }};
    ( $run_all: expr, #[skip] $pb: ident ) => {
        if $run_all {
            run_problem!($run_all, $pb);
        } else {
            println!("{}", format!("{}: skipped", stringify!($pb)).dimmed());
        }
    };
}

macro_rules! main {
    ( $( $( #[ $mode: ident ] )? $pb: ident ),* ) => {
        $( mod $pb; )*

        fn main() {
            let args: Vec<_> = std::env::args().skip(1).collect();

            let run_all = match args.as_slice() {
                [] => false,
                [arg] if ["-a", "--all"].contains(&arg.as_str()) => true,
                _ => panic!("the only known parameters are --all"),
            };

            $( run_problem!( run_all, $( #[ $mode ] )? $pb ); )*
        }
    };
}

main! {
    problem_001, problem_002, problem_003, problem_004, problem_005, problem_006, problem_007,
    problem_008, problem_009, problem_010, problem_011, #[skip] problem_012, problem_013, #[skip]
    problem_014, problem_015, problem_017, problem_018, problem_020, problem_021, problem_022,
    #[skip] problem_023, problem_024, problem_025, #[skip] problem_027, problem_029, problem_030,
    problem_031, problem_034, #[skip] problem_035, problem_036, problem_039, problem_067,
    problem_101, problem_102, #[skip] problem_104, #[skip] problem_108, #[skip] problem_201,
    problem_700
}
