use anyhow::Result;
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use serde::Serialize;
use std::fs::{self, File};
use std::num::ParseIntError;
use std::path::Path;
use std::time::Instant;

const TESTCASES_PATH: &str = "../testcases";
const TESTCASE_COUNT: usize = 25;
const TESTCASES: [&str; TESTCASE_COUNT] = [
    "s-rg-8-10",
    "s-X-12-6",
    "s-k-20-30",
    "s-k-30-50",
    "s-rg-31-15",
    "s-rg-40-20",
    "s-k-40-60",
    "s-k-20-35",
    "s-rg-63-25",
    "s-k-30-55",
    "s-rg-118-30",
    "s-rg-109-35",
    "s-k-35-65",
    "s-rg-155-40",
    "s-rg-197-45",
    "s-rg-245-50",
    "s-k-40-80",
    "s-k-50-95",
    "s-rg-413-75",
    "s-k-150-225",
    "s-k-50-100",
    "s-rg-733-100",
    "s-k-200-300",
    "s-k-100-175",
    "s-k-150-250",
];

#[derive(Debug, Clone)]
struct Set {
    id: usize,
    elements: BitVec<usize>,
}

#[derive(Debug, Default, Serialize)]
struct TestCaseOutput {
    name: &'static str,
    runtime: f64,
    set_count: usize,
    set_indices: Vec<usize>,
}

#[derive(Debug, Serialize)]
struct Output {
    total_runtime: f64,
    testcase_outputs: [TestCaseOutput; TESTCASE_COUNT],
}

#[derive(Debug)]
struct State {
    sets: Vec<Set>,
    uncovered_elements: BitVec<usize>,
    chosen_sets: Vec<usize>,
}

fn main() -> Result<()> {
    let mut testcase_outputs: [TestCaseOutput; TESTCASE_COUNT] = Default::default();
    let mut total_runtime = 0.0;
    for (&name, output) in TESTCASES.iter().zip(testcase_outputs.iter_mut()) {
        let (sets, elements) = read_testcase(name)?;
        let start = Instant::now();
        let mut set_indices =
            find_set_cover(sets, elements).expect("Failed to find valid set cover");
        let runtime = start.elapsed().as_secs_f64();
        set_indices.sort_unstable();
        *output = TestCaseOutput {
            name,
            runtime,
            set_count: set_indices.len(),
            set_indices,
        };
        total_runtime += runtime;
        println!("{} {} {}", output.name, output.set_count, output.runtime);
    }
    println!("Completed in {total_runtime} s");
    let output = Output {
        total_runtime,
        testcase_outputs,
    };
    let output_file = File::create("output.json")?;
    serde_json::to_writer(output_file, &output)?;
    Ok(())
}

fn find_set_cover(sets: Vec<Set>, uncovered_elements: BitVec<usize>) -> Option<Vec<usize>> {
    if uncovered_elements.not_any() {
        return Some(Vec::new());
    }
    let set_count = sets.len();
    let mut min_cover: Option<Vec<usize>> = None;
    let mut sets_vec = Vec::with_capacity(set_count);
    let initial_state = State {
        sets,
        uncovered_elements,
        chosen_sets: Vec::with_capacity(set_count),
    };
    let mut stack = vec![initial_state];
    while let Some(mut state) = stack.pop() {
        let dominated_sets = &mut sets_vec;
        dominated_sets.clear();
        for (index, set) in state.sets.iter().enumerate() {
            for (other_index, other) in state.sets.iter().enumerate() {
                if set.id != other.id
                    && is_subset(&set.elements, &other.elements)
                    && !dominated_sets.contains(&other_index)
                {
                    dominated_sets.push(index);
                    break;
                }
            }
        }
        for &index in dominated_sets.iter().rev() {
            state.sets.swap_remove(index);
        }
        let required_sets = &mut sets_vec;
        required_sets.clear();
        for element in state.uncovered_elements.iter_ones() {
            let mut containing_set_count = 0;
            let mut containing_index = 0;
            for (index, set) in state.sets.iter().enumerate() {
                if set.elements[element] {
                    containing_set_count += 1;
                    containing_index = index;
                    if containing_set_count > 1 {
                        break;
                    }
                }
            }
            if containing_set_count == 0 {
                continue;
            } else if containing_set_count == 1 {
                required_sets.push(containing_index);
            }
        }
        required_sets.sort_unstable();
        required_sets.dedup();
        if !required_sets.is_empty()
            && min_cover.is_some()
            && state.chosen_sets.len() + required_sets.len() >= min_cover.as_ref().unwrap().len()
        {
            continue;
        }
        for &index in required_sets.iter().rev() {
            let required_set = state.sets.swap_remove(index);
            state.chosen_sets.push(required_set.id);
            assign_difference(&mut state.uncovered_elements, &required_set.elements);
            for set in state.sets.iter_mut() {
                assign_difference(&mut set.elements, &required_set.elements);
            }
        }
        if !required_sets.is_empty() {
            state.sets.retain(|set| set.elements.any());
        }
        if state.sets.is_empty() {
            if state.uncovered_elements.not_any()
                && (min_cover.is_none()
                    || state.chosen_sets.len() < min_cover.as_ref().unwrap().len())
            {
                min_cover = Some(state.chosen_sets);
            }
            continue;
        }
        let (largest_set_index, _set) = state
            .sets
            .iter()
            .enumerate()
            .max_by_key(|(_index, set)| set.elements.count_ones())
            .unwrap();
        let largest_set = state.sets.swap_remove(largest_set_index);
        stack.push(state.clone());
        state.chosen_sets.push(largest_set.id);
        assign_difference(&mut state.uncovered_elements, &largest_set.elements);
        if state.uncovered_elements.not_any() {
            if min_cover.is_none() || state.chosen_sets.len() < min_cover.as_ref().unwrap().len() {
                min_cover = Some(state.chosen_sets);
            }
            continue;
        }
        if min_cover.is_some() && state.chosen_sets.len() + 1 >= min_cover.as_ref().unwrap().len() {
            continue;
        }
        for set in state.sets.iter_mut() {
            assign_difference(&mut set.elements, &largest_set.elements);
        }
        state.sets.retain(|set| set.elements.any());
        stack.push(state);
    }
    min_cover
}

fn is_subset(set: &BitVec<usize>, other: &BitVec<usize>) -> bool {
    set.as_raw_slice()
        .iter()
        .zip(other.as_raw_slice())
        .all(|(&s, &o)| s & o == s)
}

fn assign_difference(set: &mut BitVec<usize>, other: &BitVec<usize>) {
    let iter = set.as_raw_mut_slice().iter_mut().zip(other.as_raw_slice());
    for (s, &o) in iter {
        *s &= !o;
    }
}

fn read_testcase(name: &str) -> Result<(Vec<Set>, BitVec<usize>)> {
    let content = fs::read_to_string(Path::new(TESTCASES_PATH).join(name).with_extension("txt"))?;
    let mut lines = content.lines();
    let element_count: usize = lines.next().expect("Invalid file content").parse()?;
    let set_count: usize = lines.next().expect("Invalid file content").parse()?;
    let elements = bitvec!(usize, Lsb0; 1; element_count);
    let sets: Vec<Set> = lines
        .enumerate()
        .map(|(id, line)| {
            let mut elements = bitvec!(usize, Lsb0; 0; element_count);
            for s in line.split_ascii_whitespace() {
                let element: usize = s.parse()?;
                elements.set(element - 1, true);
            }
            Ok(Set { id, elements })
        })
        .collect::<Result<_, ParseIntError>>()?;
    assert_eq!(set_count, sets.len());
    Ok((sets, elements))
}

impl Clone for State {
    fn clone(&self) -> Self {
        let mut chosen_sets_clone = Vec::with_capacity(self.chosen_sets.capacity());
        chosen_sets_clone.extend(&self.chosen_sets);
        Self {
            sets: self.sets.clone(),
            uncovered_elements: self.uncovered_elements.clone(),
            chosen_sets: chosen_sets_clone,
        }
    }
}
