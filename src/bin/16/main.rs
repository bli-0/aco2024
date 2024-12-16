use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/16"));
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start_position: (usize, usize) = (0, 0);
    'outer: for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if grid[j][i] == 'S' {
                start_position = (i, j);
                break 'outer;
            }
        }
    }

    let part1 = search(start_position, &grid);
    println!("part1: {}", part1);
}

// Search for best time using A*.
fn search(start_position: (usize, usize), grid: &[Vec<char>]) -> i64 {
    let score = 0;
    let mut queue = BinaryHeap::new();

    // Every positon + direction pair keeps track of the best score so we can optimise away paths.
    let mut best_scores: HashMap<((usize, usize), Direction), i64> = HashMap::new();
    queue.push(State {
        position: start_position,
        score,
        direction: Direction::East,
    });

    while let Some(current) = queue.pop() {
        if let Some(value) = best_scores.get_mut(&(current.position, current.direction)) {
            if current.score <= *value {
                *value = current.score;
            } else {
                continue;
            }
        } else {
            best_scores.insert((current.position, current.direction), current.score);
        }
        if grid[current.position.1][current.position.0] == 'E' {
            return current.score;
        }

        {
            let new_direction = current.direction.turn_anti_clockwise();
            let new_score = current.score + 1000;
            queue.push(State {
                position: current.position,
                score: new_score,
                direction: new_direction,
            });
        }

        {
            let new_direction = current.direction.turn_clockwise();
            let new_score: i64 = current.score + 1000;
            queue.push(State {
                position: current.position,
                score: new_score,
                direction: new_direction,
            });
        }

        // Go forward
        {
            match current.direction {
                Direction::East => {
                    if grid[current.position.1][current.position.0 + 1] != '#' {
                        let new_position = (current.position.0 + 1, current.position.1);
                        queue.push(State {
                            position: new_position,
                            score: current.score + 1,
                            direction: current.direction,
                        });
                        continue;
                    }
                }
                Direction::North => {
                    if grid[current.position.1 - 1][current.position.0] != '#' {
                        let new_position = (current.position.0, current.position.1 - 1);
                        queue.push(State {
                            position: new_position,
                            score: current.score + 1,
                            direction: current.direction,
                        });
                        continue;
                    }
                }
                Direction::West => {
                    if grid[current.position.1][current.position.0 - 1] != '#' {
                        let new_position = (current.position.0 - 1, current.position.1);
                        queue.push(State {
                            position: new_position,
                            score: current.score + 1,
                            direction: current.direction,
                        });
                        continue;
                    }
                }
                Direction::South => {
                    if grid[current.position.1 + 1][current.position.0] != '#' {
                        let new_position = (current.position.0, current.position.1 + 1);
                        queue.push(State {
                            position: new_position,
                            score: current.score + 1,
                            direction: current.direction,
                        });
                        continue;
                    }
                }
            }
        }
    }

    panic!("not found");
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct State {
    position: (usize, usize),
    score: i64,
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // flip so we are a min heap.
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
enum Direction {
    East,
    North,
    West,
    South,
}

impl Direction {
    fn turn_anti_clockwise(&self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        }
    }

    fn turn_clockwise(&self) -> Self {
        match self {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
        }
    }
}
