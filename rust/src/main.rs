use anyhow::Result;
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use std::fs::{self, File};
use std::io::prelude::*;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::time::Instant;

const TESTCASES: [&str; 24] = [
    "s-k-40-60",
    "s-k-20-30",
    "s-rg-197-45",
    "s-k-20-35",
    "s-rg-733-100",
    "s-k-35-65",
    "s-k-30-55",
    "s-rg-413-75",
    "s-rg-63-25",
    "s-rg-31-15",
    "s-rg-109-35",
    "s-rg-245-50",
    "s-k-30-50",
    "s-k-100-175",
    "s-k-150-225",
    // "s-k-150-250",
    "s-k-200-300",
    "s-rg-8-10",
    "s-rg-40-20",
    "s-k-50-100",
    "s-k-40-80",
    "s-rg-155-40",
    "s-X-12-6",
    "s-k-50-95",
    "s-rg-118-30",
];

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
        let line = c_output.lines().next().unwrap();
        let c_runtime: f64 = line.split_ascii_whitespace().nth(8).unwrap().parse()?;
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
    let sets: Vec<(usize, BitVec<usize>)> = (1..)
        .zip(lines)
        .map(|(i, line)| {
            let mut set = bitvec!(usize, Lsb0; 0; element_count);
            for s in line.split_ascii_whitespace() {
                let element: usize = s.parse()?;
                set.set(element - 1, true);
            }
            Ok((i, set))
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
        set_cover
    );
    for i in set_cover {
        let set_elements: Vec<_> = sets[i - 1].1.iter_ones().map(|i| i + 1).collect();
        output.push_str(&format!("Set #{i}: {:?}\n", set_elements));
    }
    println!("{}", output);
    Ok((runtime, output))
}

fn find_set_cover(
    mut sets: Vec<(usize, BitVec<usize>)>,
    mut uncovered_elements: BitVec<usize>,
) -> Option<Vec<usize>> {
    if uncovered_elements.count_ones() == 0 {
        return Some(Vec::new());
    }
    for (index, (i, set)) in sets.iter().enumerate() {
        for (j, other) in &sets {
            if i != j && set.iter_ones().all(|i| other[i]) {
                let mut sets = sets.to_owned();
                sets.remove(index);
                return find_set_cover(sets, uncovered_elements);
            }
        }
    }
    for element in uncovered_elements.iter_ones() {
        let mut containing_set_count = 0;
        let mut containing_index = 0;
        for (index, (_, set)) in sets.iter().enumerate() {
            if set[element] {
                containing_set_count += 1;
                containing_index = index;
                if containing_set_count > 1 {
                    break;
                }
            }
        }
        if containing_set_count == 0 {
            return None;
        } else if containing_set_count == 1 {
            let mut new_sets = sets.to_owned();
            let (required_set_id, required_set) = new_sets.remove(containing_index);
            for (_, set) in new_sets.iter_mut() {
                for element in required_set.iter_ones() {
                    set.set(element, false);
                }
            }
            new_sets.retain(|(_, set)| !set.is_empty());
            for element in required_set.iter_ones() {
                uncovered_elements.set(element, false);
            }
            let mut cover = find_set_cover(new_sets, uncovered_elements);
            if let Some(set) = cover.as_mut() {
                set.push(required_set_id);
            }
            return cover;
        }
    }
    let mut largest_set_index = 0;
    for (index, (_, set)) in sets.iter().enumerate() {
        if set.count_ones() > sets[largest_set_index].1.count_ones() {
            largest_set_index = index;
        }
    }
    let (selected_set_id, selected_set) = sets.remove(largest_set_index);
    let mut new_sets = sets.clone();
    let mut new_uncovered_elements = uncovered_elements.clone();
    for (_, set) in new_sets.iter_mut() {
        for element in selected_set.iter_ones() {
            set.set(element, false);
        }
    }
    new_sets.retain(|(_, set)| !set.is_empty());
    for element in selected_set.iter_ones() {
        new_uncovered_elements.set(element, false);
    }
    let mut with_selected = find_set_cover(new_sets, new_uncovered_elements);
    if let Some(set) = with_selected.as_mut() {
        set.push(selected_set_id);
    }
    let without_selected = find_set_cover(sets, uncovered_elements);
    match (with_selected, without_selected) {
        (None, None) => None,
        (None, without @ Some(_)) => without,
        (with @ Some(_), None) => with,
        (Some(with), Some(without)) => Some(std::cmp::min_by_key(with, without, Vec::len)),
    }
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
