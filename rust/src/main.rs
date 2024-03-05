use anyhow::Result;
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use std::fs::{self, File};
use std::io::prelude::*;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::time::Instant;

const TESTCASES: [&str; 24] = [
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

fn main() -> Result<()> {
    let testcases_path = Path::new("../testcases");
    let output_path = Path::new("output");
    let mut total_rust_runtime = 0.0;
    let mut total_c_runtime = 0.0;
    let mut output_summary = File::create(output_path.join("summary").with_extension("md"))?;
    writeln!(
        &mut output_summary,
        "|     Testcase     |  C Runtime  | Rust Runtime |  Change  |\n\
         |------------------|-------------|--------------|----------|"
    )?;
    for &testcase in &TESTCASES {
        let c_path = PathBuf::from("../c/output")
            .join(testcase)
            .with_extension("txt");
        let c_output = fs::read_to_string(c_path)?;
        let c_runtime: f64 = if c_output.to_lowercase().trim() == "did not finish" {
            f64::INFINITY
        } else {
            let line = c_output.lines().next().unwrap();
            line.split_ascii_whitespace().nth(8).unwrap().parse()?
        };
        let testcase_path = testcases_path.join(testcase).with_extension("txt");
        let (rust_runtime, rust_output) = run_testcase(&testcase_path)?;
        let mut output_file = File::create(output_path.join(testcase).with_extension("txt"))?;
        output_file.write_all(rust_output.as_bytes())?;
        writeln!(
            &mut output_summary,
            "| {:^16} | {:>11} | {:>12} | {:>7.2}% |",
            testcase,
            format_runtime(c_runtime),
            format_runtime(rust_runtime),
            ((rust_runtime - c_runtime) / c_runtime) * 100.0
        )?;
        total_c_runtime += c_runtime;
        total_rust_runtime += rust_runtime;
    }
    writeln!(
        &mut output_summary,
        "| {:^16} | {:>11} | {:>12} | {:>7.2}% |",
        "Total",
        format_runtime(total_c_runtime),
        format_runtime(total_rust_runtime),
        ((total_rust_runtime - total_c_runtime) / total_c_runtime) * 100.0
    )?;
    Ok(())
}

fn run_testcase(path: &Path) -> Result<(f64, String)> {
    let testcase_name = path.file_name().unwrap().to_string_lossy();
    println!("Running testcase {testcase_name}");
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
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
    let sets_clone = sets.clone();
    let start = Instant::now();
    let mut set_cover =
        find_set_cover(sets_clone, elements).expect("Failed to find valid set cover");
    let runtime = start.elapsed().as_secs_f64();
    set_cover.sort_unstable();
    let mut output = format!(
        "Found minimum set cover containing {} sets in {}.\n\
        Included sets: {:?}\n",
        set_cover.len(),
        format_runtime(runtime),
        set_cover.iter().map(|i| i + 1).collect::<Vec<_>>()
    );
    for i in set_cover {
        let set_elements: Vec<_> = sets[i].elements.iter_ones().map(|i| i + 1).collect();
        let set_number = i + 1;
        output.push_str(&format!("Set #{set_number}: {:?}\n", set_elements));
    }
    println!("{}", output);
    Ok((runtime, output))
}

fn find_set_cover(sets: Vec<Set>, uncovered_elements: BitVec<usize>) -> Option<Vec<usize>> {
    if uncovered_elements.not_any() {
        return Some(Vec::new());
    }
    let mut best_sets: Option<Vec<usize>> = None;
    let set_count = sets.len();
    let mut stack = vec![(sets, uncovered_elements, Vec::with_capacity(set_count))];
    while let Some((mut sets, mut uncovered_elements, mut selected_sets)) = stack.pop() {
        if uncovered_elements.not_any() {
            if let Some(best) = best_sets.as_ref() {
                if selected_sets.len() < best.len() {
                    best_sets = Some(selected_sets);
                }
            } else {
                best_sets = Some(selected_sets);
            }
            continue;
        }
        if let Some(best) = best_sets.as_ref() {
            if selected_sets.len() + 1 >= best.len() {
                continue;
            }
        }
        let mut dominated_sets = Vec::with_capacity(sets.len());
        for (index, set) in sets.iter().enumerate() {
            for other in sets.iter() {
                if set.id != other.id && set.elements.iter_ones().all(|i| other.elements[i]) {
                    dominated_sets.push(index);
                    break;
                }
            }
        }
        for &index in dominated_sets.iter().rev() {
            sets.remove(index);
        }
        dominated_sets.clear();
        let mut required_sets = dominated_sets;
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
            selected_sets.push(required_set.id);
            for element in required_set.elements.iter_ones() {
                uncovered_elements.set(element, false);
                for set in sets.iter_mut() {
                    set.elements.set(element, false);
                }
            }
        }
        if !required_sets.is_empty() {
            if let Some(solution_sets) = best_sets.as_ref() {
                if selected_sets.len() >= solution_sets.len() {
                    continue;
                }
            }
            sets.retain(|set| set.elements.any());
        }
        if sets.is_empty() {
            if uncovered_elements.not_any() {
                if let Some(best) = best_sets.as_ref() {
                    if selected_sets.len() < best.len() {
                        best_sets = Some(selected_sets);
                    }
                } else {
                    best_sets = Some(selected_sets);
                }
            }
            continue;
        }
        let (largest_set_index, _set) = sets
            .iter()
            .enumerate()
            .max_by_key(|(_index, set)| set.elements.count_ones())
            .unwrap();
        let selected_set = sets.remove(largest_set_index);
        let mut new_selected_sets = Vec::with_capacity(set_count);
        new_selected_sets.extend(&selected_sets);
        new_selected_sets.push(selected_set.id);
        let mut new_sets = sets.clone();
        let mut new_uncovered_elements = uncovered_elements.clone();
        for element in selected_set.elements.iter_ones() {
            new_uncovered_elements.set(element, false);
            for set in new_sets.iter_mut() {
                set.elements.set(element, false);
            }
        }
        new_sets.retain(|set| set.elements.any());
        stack.push((new_sets, new_uncovered_elements, new_selected_sets));
        stack.push((sets, uncovered_elements, selected_sets));
    }
    best_sets
}

fn format_runtime(runtime: f64) -> String {
    if runtime < 1e-3 {
        format!("{:.2} us", runtime * 1e6)
    } else if runtime < 1.0 {
        format!("{:.2} ms", runtime * 1e3)
    } else {
        format!("{:.2} s", runtime)
    }
}
