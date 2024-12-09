use std::{collections::HashSet, fs::File, path::Display};

#[derive(PartialEq, Clone)]
enum State {
    FreeSpace,
    FileBlock,
}

impl State {
    fn flip(&mut self) -> Self {
        match self {
            State::FreeSpace => State::FileBlock,
            State::FileBlock => State::FreeSpace,
        }
    }
}

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/9"));
    let mut total_length = 0;
    let mut state = State::FileBlock;
    let mut original_filesystem: Vec<FileSystem> = vec![];
    let mut current_id = 0;
    for c in input.chars() {
        let block_size = c.to_digit(10).unwrap();

        original_filesystem.push(FileSystem::new(
            current_id, // ignored if space.
            total_length,
            total_length + block_size,
            state.clone(),
        ));

        if state == State::FileBlock {
            current_id += 1;
        }

        total_length += block_size;
        state = state.flip()
    }

    // Have two scanners left and right.
    let mut left_scanner_idx = 0;
    let mut right_scanner_idx  = total_length;
}

struct FileSystem {
    id: u32,
    state: State,
    // Inclusive of Start
    orig_start_idx: u32,
    // Not inclusive of end.
    orig_end_idx: u32,
    consumed_idx: HashSet<u32>,
}

impl FileSystem {
    fn new(id: u32, start: u32, end: u32, state: State) -> Self {
        Self {
            id,
            state,
            orig_start_idx: start,
            orig_end_idx: end,
            consumed_idx: HashSet::new(),
        }
    }
    fn in_bounds(&self, idx: u32) -> bool {
        idx >= self.orig_start_idx && idx < self.orig_end_idx
    }
}

impl std::fmt::Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            State::FreeSpace => for ,
            State::FileBlock => todo!(),
        }
    }
}
