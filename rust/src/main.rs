use anyhow::Result;
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use serde::Serialize;
use std::fs::{self, File};
use std::num::ParseIntError;
use std::path::Path;
use std::time::Instant;

const TESTCASES_PATH: &str = "../testcases";
const TESTCASE_COUNT: usize = 24;
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
    // "s-k-150-250",
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
        println!("{:?}", &output);
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
    let mut stack = vec![(sets, uncovered_elements, Vec::with_capacity(set_count))];
    while let Some((mut sets, mut uncovered_elements, mut chosen_sets)) = stack.pop() {
        if uncovered_elements.not_any() {
            if min_cover.is_none() || chosen_sets.len() < min_cover.as_ref().unwrap().len() {
                min_cover = Some(chosen_sets);
            }
            continue;
        }
        if min_cover.is_some() && chosen_sets.len() + 1 >= min_cover.as_ref().unwrap().len() {
            continue;
        }
        let dominated_sets = &mut sets_vec;
        dominated_sets.clear();
        for (index, set) in sets.iter().enumerate() {
            for other in sets.iter() {
                if set.id != other.id
                    && set.elements.iter_ones().all(|i| other.elements[i])
                    && (set.id < other.id
                        || set.elements.count_ones() < other.elements.count_ones())
                {
                    dominated_sets.push(index);
                    break;
                }
            }
        }
        for &index in dominated_sets.iter().rev() {
            sets.remove(index);
        }
        let required_sets = &mut sets_vec;
        required_sets.clear();
        for element in uncovered_elements.iter_ones() {
            let mut containing_set_count = 0;
            let mut containing_index = 0;
            for (index, set) in sets.iter().enumerate() {
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
        for &index in required_sets.iter().rev() {
            let required_set = sets.remove(index);
            chosen_sets.push(required_set.id);
            for element in required_set.elements.iter_ones() {
                uncovered_elements.set(element, false);
                for set in sets.iter_mut() {
                    set.elements.set(element, false);
                }
            }
        }
        if !required_sets.is_empty() {
            if min_cover.is_some() && chosen_sets.len() >= min_cover.as_ref().unwrap().len() {
                continue;
            }
            sets.retain(|set| set.elements.any());
        }
        if sets.is_empty() {
            if uncovered_elements.not_any()
                && (min_cover.is_none() || chosen_sets.len() < min_cover.as_ref().unwrap().len())
            {
                min_cover = Some(chosen_sets);
            }
            continue;
        }
        let (largest_set_index, _set) = sets
            .iter()
            .enumerate()
            .max_by_key(|(_index, set)| set.elements.count_ones())
            .unwrap();
        let chosen_set = sets.remove(largest_set_index);
        stack.push((
            sets.clone(),
            uncovered_elements.clone(),
            clone_with_capacity(&chosen_sets, set_count),
        ));
        chosen_sets.push(chosen_set.id);
        for element in chosen_set.elements.iter_ones() {
            uncovered_elements.set(element, false);
            for set in sets.iter_mut() {
                set.elements.set(element, false);
            }
        }
        sets.retain(|set| set.elements.any());
        stack.push((sets, uncovered_elements, chosen_sets));
    }
    min_cover
}

fn clone_with_capacity<T: Clone>(slice: &[T], capacity: usize) -> Vec<T> {
    let mut clone = Vec::with_capacity(capacity);
    clone.extend_from_slice(slice);
    clone
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
