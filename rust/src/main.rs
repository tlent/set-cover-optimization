use anyhow::Result;
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::time::Instant;

fn main() -> Result<()> {
    let filename = std::env::args().nth(1).expect("Missing filename arg");
    let mut file = File::open(filename)?;
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
    let start = Instant::now();
    let mut set_cover =
        find_set_cover(sets.clone(), elements).expect("Failed to find valid set cover");
    let end = start.elapsed();
    println!(
        "Found minimum set cover containing {} sets in {:.6} seconds.",
        set_cover.len(),
        end.as_secs_f64()
    );
    set_cover.sort_unstable();
    println!("Included sets: {:?}", set_cover);
    for i in set_cover {
        let set_elements: Vec<_> = sets[i - 1].1.iter_ones().map(|i| i + 1).collect();
        println!("Set #{i}: {:?}", set_elements);
    }
    Ok(())
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
