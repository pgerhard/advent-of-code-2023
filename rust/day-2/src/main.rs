use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::time::Instant;
use std::cell::RefCell;
use std::thread::LocalKey;
use crate::Color::{BLUE, GREEN, RED};

thread_local!(
    static GLOBAL_PROCESS: RefCell<Vec<u64>> = RefCell::new(Vec::new());
    static GLOBAL_LOAD_FILE_TIMES: RefCell<Vec<u64>> = RefCell::new(Vec::new());
    static GLOBAL_PARSE_GAMES: RefCell<Vec<u64>> = RefCell::new(Vec::new());
    static GLOBAL_FIND_POSSIBLE_GAMES: RefCell<Vec<u64>> = RefCell::new(Vec::new());
    static GLOBAL_ID_SUM: RefCell<Vec<u64>> = RefCell::new(Vec::new());
    static GLOBAL_DETERMINE_MINIMUM_SET: RefCell<Vec<u64>> = RefCell::new(Vec::new());
    static GLOBAL_SET_POWERS: RefCell<Vec<u64>> = RefCell::new(Vec::new());
    static GLOBAL_POWER_SUM: RefCell<Vec<u64>> = RefCell::new(Vec::new());
);

fn main() {
    reset_global_times();
    let result = timed(
        &GLOBAL_PROCESS,
        || process_input("./src/input.txt".to_string()),
    );
    report_global_times("Process Input".to_string());
    println!("Sum of possible games: {}", result.id_sum);
    println!("Power Sum of games: {}", result.power_sum);
}

fn process_input(filename: String) -> Result {
    let lines = timed(
        &GLOBAL_LOAD_FILE_TIMES,
        || read_lines(filename).unwrap(),
    );
    let games = timed(
        &GLOBAL_PARSE_GAMES,
        || lines.map(|line| parse_game(line.unwrap())).collect::<Vec<Game>>(),
    );

    let possible_games = timed(
        &GLOBAL_FIND_POSSIBLE_GAMES,
        || games
            .iter()
            .clone()
            .filter(|game| is_game_possible(game))
    );
    let id_sum = timed(
        &GLOBAL_ID_SUM,
        || possible_games
            .fold(
                0,
                |acc, game| acc + game.number
            )
    );

    let minimum_sets = timed(
        &GLOBAL_DETERMINE_MINIMUM_SET,
        || games
            .iter()
            .clone()
            .map(|game| determine_minimum_cubes(game))
    );
    let set_powers = timed(
        &GLOBAL_SET_POWERS,
        || minimum_sets
            .map(|minimum_set| calc_set_power(&minimum_set))
    );
    let power_sum = timed(
        &GLOBAL_POWER_SUM,
        || set_powers
            .fold(
                0,
                |acc, set_power| acc + set_power
            )
    );

    return Result{
        id_sum,
        power_sum
    };
}

fn is_game_possible(game: &Game) -> bool {
    let impossible_sets = game.sets.iter()
        .filter(|&set| is_set_impossible(set))
        .collect::<Vec<&Set>>();
    return impossible_sets.is_empty();
}

fn is_set_impossible(set: &Set) -> bool {
    let impossible_cubes = set.cubes.iter()
        .filter(|&cube| {
            return if cube.color == RED  { cube.count > 12 }
            else if cube.color == GREEN  { cube.count > 13 }
            else if cube.color == BLUE  { cube.count > 14 }
            else { panic!("Unknown color") }
        })
        .collect::<Vec<&Cube>>();
    return !impossible_cubes.is_empty();
}

fn determine_minimum_cubes(game: &Game) -> MinimumSet {
    let max_red_cube = game.sets.iter()
        .flat_map(|set| set.cubes.clone())
        .filter(|cube| cube.color == RED)
        .max_by_key(|cube| cube.count)
        .map(|cube| cube.count)
        .or_else( || Some(0));

    let max_green_cube = game.sets.iter()
        .flat_map(|set| set.cubes.clone())
        .filter(|cube| cube.color == GREEN)
        .max_by_key(|cube| cube.count)
        .map(|cube| cube.count)
        .or_else( || Some(0));

    let max_blue_cube = game.sets.iter()
        .flat_map(|set| set.cubes.clone())
        .filter(|cube| cube.color == BLUE)
        .max_by_key(|cube| cube.count)
        .map(|cube| cube.count)
        .or_else( || Some(0));

    return MinimumSet{
        red: Cube { count: max_red_cube.unwrap(), color: RED },
        green: Cube { count: max_green_cube.unwrap(), color: GREEN },
        blue: Cube { count: max_blue_cube.unwrap(), color: BLUE },
    }
}

fn calc_set_power(minimum_set: &MinimumSet) -> u64 {
    return minimum_set.red.count * minimum_set.green.count * minimum_set.blue.count
}

fn parse_game(line: String) -> Game {
    let splits = line.split(":").collect::<Vec<&str>>();

    return Game {
        number: splits[0].to_string().replace("Game ", "").parse::<u64>().unwrap(),
        sets: splits[1].split(";")
            .map(|set_string| set_string.to_string())
            .map(|set_string| parse_set(set_string))
            .collect(),
    };
}

fn parse_set(set_string: String) -> Set {
    return Set {
        cubes: set_string
            .split(",")
            .map(|cube_string| parse_cub(cube_string.to_string()))
            .collect::<Vec<Cube>>()
    };
}

fn parse_cub(cube_string: String) -> Cube {
    let trimmed_cube_string = cube_string.trim();
    let splits = trimmed_cube_string.split(" ").map(|split| split.to_string()).collect::<Vec<String>>();

    return Cube {
        count: splits[0].parse::<u64>().unwrap(),
        color: if splits[1] == "red" { Color::RED } else if splits[1] == "green" { Color::GREEN } else if splits[1] == "blue" { Color::BLUE } else { panic!("Unknown color") },
    };
}

#[derive(PartialEq, Debug)]
struct Game {
    number: u64,
    sets: Vec<Set>,
}

#[derive(PartialEq, Debug, Clone)]
struct Set {
    cubes: Vec<Cube>,
}

#[derive(PartialEq, Debug)]
struct MinimumSet {
    red : Cube,
    green : Cube,
    blue : Cube
}

#[derive(PartialEq, Debug, Clone)]
struct Cube {
    count: u64,
    color: Color,
}

#[derive(PartialEq, Debug, Clone)]
enum Color {
    BLUE,
    GREEN,
    RED,
}

#[derive(PartialEq, Debug, Clone)]
struct Result {
    id_sum : u64,
    power_sum : u64
}

fn timed<R, F: FnOnce() -> R>(ref_cell_key: &'static LocalKey<RefCell<Vec<u64>>>, func: F) -> R {
    let stopwatch = Instant::now();
    let result = func();
    ref_cell_key.with(|times| {
        let mut borrowed_times = times.borrow_mut();
        borrowed_times.push(stopwatch.elapsed().as_nanos() as u64)
    });
    return result;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    return Ok(BufReader::new(file).lines());
}

fn reset_global_times() {
    GLOBAL_PROCESS.with(|times| {
        times.borrow_mut().clear()
    });
    GLOBAL_LOAD_FILE_TIMES.with(|times| {
        times.borrow_mut().clear()
    });
    GLOBAL_PARSE_GAMES.with(|times| {
        times.borrow_mut().clear()
    });
    GLOBAL_FIND_POSSIBLE_GAMES.with(|times| {
        times.borrow_mut().clear()
    });
    GLOBAL_ID_SUM.with(|times| {
        times.borrow_mut().clear()
    });
    GLOBAL_DETERMINE_MINIMUM_SET.with(|times| {
        times.borrow_mut().clear()
    });
    GLOBAL_SET_POWERS.with(|times| {
        times.borrow_mut().clear()
    });
    GLOBAL_POWER_SUM.with(|times| {
        times.borrow_mut().clear()
    });
}

fn report_global_times(prefix: String) {
    println!(
        "{} - Overall: Executions {}, Time {:.2}ns",
        prefix,
        GLOBAL_PROCESS.with(|times| {
            times.borrow().len() as f64
        }),
        GLOBAL_PROCESS.with(|times| {
            let borrowed_times = times.borrow();
            borrowed_times.iter().sum::<u64>() as f64 / borrowed_times.len() as f64
        })
    );
    println!(
        "{} - Load File: Executions {}, Time {:.2}ns",
        prefix,
        GLOBAL_LOAD_FILE_TIMES.with(|times| {
            times.borrow().len() as f64
        }),
        GLOBAL_LOAD_FILE_TIMES.with(|times| {
            let borrowed_times = times.borrow();
            borrowed_times.iter().sum::<u64>() as f64 / borrowed_times.len() as f64
        })
    );
    println!(
        "{} - Parse Game: Executions {}, Time {:.2}ns",
        prefix,
        GLOBAL_PARSE_GAMES.with(|times| {
            times.borrow().len() as f64
        }),
        GLOBAL_PARSE_GAMES.with(|times| {
            let borrowed_times = times.borrow();
            borrowed_times.iter().sum::<u64>() as f64 / borrowed_times.len() as f64
        })
    );
    println!(
        "{} - Find Possible Games: Executions {}, Time {:.2}ns",
        prefix,
        GLOBAL_FIND_POSSIBLE_GAMES.with(|times| {
            times.borrow().len() as f64
        }),
        GLOBAL_FIND_POSSIBLE_GAMES.with(|times| {
            let borrowed_times = times.borrow();
            borrowed_times.iter().sum::<u64>() as f64 / borrowed_times.len() as f64
        })
    );
    println!(
        "{} - Sum ID: Executions {}, Time {:.2}ns",
        prefix,
        GLOBAL_ID_SUM.with(|times| {
            times.borrow().len() as f64
        }),
        GLOBAL_ID_SUM.with(|times| {
            let borrowed_times = times.borrow();
            borrowed_times.iter().sum::<u64>() as f64 / borrowed_times.len() as f64
        })
    );
    println!(
        "{} - Determine Minimum Set: Executions {}, Time {:.2}ns",
        prefix,
        GLOBAL_DETERMINE_MINIMUM_SET.with(|times| {
            times.borrow().len() as f64
        }),
        GLOBAL_DETERMINE_MINIMUM_SET.with(|times| {
            let borrowed_times = times.borrow();
            borrowed_times.iter().sum::<u64>() as f64 / borrowed_times.len() as f64
        })
    );
    println!(
        "{} - Calculate Set Powers: Executions {}, Time {:.2}ns",
        prefix,
        GLOBAL_SET_POWERS.with(|times| {
            times.borrow().len() as f64
        }),
        GLOBAL_SET_POWERS.with(|times| {
            let borrowed_times = times.borrow();
            borrowed_times.iter().sum::<u64>() as f64 / borrowed_times.len() as f64
        })
    );
    println!(
        "{} - Calculate Power Sum: Executions {}, Time {:.2}ns",
        prefix,
        GLOBAL_POWER_SUM.with(|times| {
            times.borrow().len() as f64
        }),
        GLOBAL_POWER_SUM.with(|times| {
            let borrowed_times = times.borrow();
            borrowed_times.iter().sum::<u64>() as f64 / borrowed_times.len() as f64
        })
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        reset_global_times();
        let actual = timed(
            &GLOBAL_PROCESS,
            || process_input("./src/test-input.txt".to_string()),
        );
        report_global_times("Test Process Input".to_string());
        assert_eq!(
            actual,
            Result {
                id_sum : 8,
                power_sum : 2286
            }
        )
    }

    #[test]
    fn test_parse_game() {
        let actual = parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string());
        assert_eq!(
            actual,
            Game {
                number: 3,
                sets: vec![
                    Set {
                        cubes: vec![
                            Cube { count: 8, color: GREEN },
                            Cube { count: 6, color: BLUE },
                            Cube { count: 20, color: RED },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { count: 5, color: BLUE },
                            Cube { count: 4, color: RED },
                            Cube { count: 13, color: GREEN },
                        ]
                    },
                    Set {
                        cubes: vec![
                            Cube { count: 5, color: GREEN },
                            Cube { count: 1, color: RED },
                        ]
                    },
                ],
            }
        )
    }

    #[test]
    fn test_is_red_game_possible_true () {
        let actual = is_game_possible(
            &Game {
                number: 1,
                sets : vec![
                    Set {
                        cubes: vec![
                            Cube { count: 12, color: RED },
                        ]
                    }
                ]
            }
        );
        assert_eq!(
            actual,
            true
        )
    }

    #[test]
    fn test_is_red_game_possible_false () {
        let actual = is_game_possible(
            &Game {
                number: 1,
                sets : vec![
                    Set {
                        cubes: vec![
                            Cube { count: 13, color: RED },
                        ]
                    }
                ]
            }
        );
        assert_eq!(
            actual,
            false
        )
    }

    #[test]
    fn test_is_green_game_possible_true () {
        let actual = is_game_possible(
            &Game {
                number: 1,
                sets : vec![
                    Set {
                        cubes: vec![
                            Cube { count: 13, color: GREEN },
                        ]
                    }
                ]
            }
        );
        assert_eq!(
            actual,
            true
        )
    }

    #[test]
    fn test_is_green_game_possible_false () {
        let actual = is_game_possible(
            &Game {
                number: 1,
                sets : vec![
                    Set {
                        cubes: vec![
                            Cube { count: 14, color: GREEN },
                        ]
                    }
                ]
            }
        );
        assert_eq!(
            actual,
            false
        )
    }


    #[test]
    fn test_is_blue_game_possible_true () {
        let actual = is_game_possible(
            &Game {
                number: 1,
                sets : vec![
                    Set {
                        cubes: vec![
                            Cube { count: 14, color: BLUE },
                        ]
                    }
                ]
            }
        );
        assert_eq!(
            actual,
            true
        )
    }

    #[test]
    fn test_is_blue_game_possible_false () {
        let actual = is_game_possible(
            &Game {
                number: 1,
                sets : vec![
                    Set {
                        cubes: vec![
                            Cube { count: 15, color: BLUE },
                        ]
                    }
                ]
            }
        );
        assert_eq!(
            actual,
            false
        )
    }

    #[test]
    fn test_determine_minimum_cubes_just_red () {
        let actual = determine_minimum_cubes(
            &Game {
                number : 1,
                sets : vec![
                    Set {
                        cubes: vec![
                            Cube { count: 10, color: RED },
                        ]
                    }
                ]
            }
        );
        assert_eq!(
            actual,
            MinimumSet {
                red : Cube { count : 10, color : RED},
                green : Cube { count : 0, color : GREEN},
                blue : Cube { count : 0, color : BLUE}
            }
        )
    }

    #[test]
    fn test_determine_minimum_cubes_just_green () {
        let actual = determine_minimum_cubes(
            &Game {
                number : 1,
                sets : vec![
                    Set {
                        cubes: vec![
                            Cube { count: 15, color: GREEN },
                        ]
                    }
                ]
            }
        );
        assert_eq!(
            actual,
            MinimumSet {
                red : Cube { count : 0, color : RED},
                green : Cube { count : 15, color : GREEN},
                blue : Cube { count : 0, color : BLUE}
            }
        )
    }

    #[test]
    fn test_determine_minimum_cubes_just_blue () {
        let actual = determine_minimum_cubes(
            &Game {
                number : 1,
                sets : vec![
                    Set {
                        cubes: vec![
                            Cube { count: 20, color: BLUE },
                        ]
                    }
                ]
            }
        );
        assert_eq!(
            actual,
            MinimumSet {
                red : Cube { count : 0, color : RED},
                green : Cube { count : 0, color : GREEN},
                blue : Cube { count : 20, color : BLUE}
            }
        )
    }

    #[test]
    fn test_determine_minimum_cubes_game_one () {
        let actual = determine_minimum_cubes(
            &parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string())
        );
        assert_eq!(
            actual,
            MinimumSet {
                red : Cube { count : 4, color : RED},
                green : Cube { count : 2, color : GREEN},
                blue : Cube { count : 6, color : BLUE}
            }
        )
    }

    #[test]
    fn test_determine_minimum_cubes_game_two () {
        let actual = determine_minimum_cubes(
            &parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string())
        );
        assert_eq!(
            actual,
            MinimumSet {
                red : Cube { count : 1, color : RED},
                green : Cube { count : 3, color : GREEN},
                blue : Cube { count : 4, color : BLUE}
            }
        )
    }

    #[test]
    fn test_determine_minimum_cubes_game_three () {
        let actual = determine_minimum_cubes(
            &parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string())
        );
        assert_eq!(
            actual,
            MinimumSet {
                red : Cube { count : 20, color : RED},
                green : Cube { count : 13, color : GREEN},
                blue : Cube { count : 6, color : BLUE}
            }
        )
    }

    #[test]
    fn test_determine_minimum_cubes_game_four () {
        let actual = determine_minimum_cubes(
            &parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string())
        );
        assert_eq!(
            actual,
            MinimumSet {
                red : Cube { count : 14, color : RED},
                green : Cube { count : 3, color : GREEN},
                blue : Cube { count : 15, color : BLUE}
            }
        )
    }

    #[test]
    fn test_determine_minimum_cubes_game_five () {
        let actual = determine_minimum_cubes(
            &parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string())
        );
        assert_eq!(
            actual,
            MinimumSet {
                red : Cube { count : 6, color : RED},
                green : Cube { count : 3, color : GREEN},
                blue : Cube { count : 2, color : BLUE}
            }
        )
    }

    #[test]
    fn test_calc_set_power_game_one() {
        let actual = calc_set_power(
            &MinimumSet {
                red : Cube { count : 4, color : RED},
                green : Cube { count : 2, color : GREEN},
                blue : Cube { count : 6, color : BLUE}
            }
        );
        assert_eq!(
            actual,
            48
        )
    }

    #[test]
    fn test_calc_set_power_game_two() {
        let actual = calc_set_power(
            &MinimumSet {
                red : Cube { count : 1, color : RED},
                green : Cube { count : 3, color : GREEN},
                blue : Cube { count : 4, color : BLUE}
            }
        );
        assert_eq!(
            actual,
            12
        )
    }

    #[test]
    fn test_calc_set_power_game_three() {
        let actual = calc_set_power(
            &MinimumSet {
                red : Cube { count : 20, color : RED},
                green : Cube { count : 13, color : GREEN},
                blue : Cube { count : 6, color : BLUE}
            }
        );
        assert_eq!(
            actual,
            1560
        )
    }

    #[test]
    fn test_calc_set_power_game_four() {
        let actual = calc_set_power(
            &MinimumSet {
                red : Cube { count : 14, color : RED},
                green : Cube { count : 3, color : GREEN},
                blue : Cube { count : 15, color : BLUE}
            }
        );
        assert_eq!(
            actual,
            630
        )
    }

    #[test]
    fn test_calc_set_power_game_five() {
        let actual = calc_set_power(
            &MinimumSet {
                red : Cube { count : 6, color : RED},
                green : Cube { count : 3, color : GREEN},
                blue : Cube { count : 2, color : BLUE}
            }
        );
        assert_eq!(
            actual,
            36
        )
    }
}