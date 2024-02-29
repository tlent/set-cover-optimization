use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() -> Result<()> {
    let filename = std::env::args().nth(1).expect("Missing filename arg");
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let mut lines = content.lines();
    let element_count: u32 = lines.next().expect("Invalid file content").parse()?;
    let set_count: usize = lines.next().expect("Invalid file content").parse()?;
    let elements: HashSet<u32> = (1..=element_count).collect();
    let sets: Vec<(usize, HashSet<u32>)> = (1..)
        .zip(lines)
        .map(|(i, line)| {
            line.split_ascii_whitespace()
                .map(|word| word.parse())
                .collect::<Result<_, _>>()
                .map(|set| (i, set))
        })
        .collect::<Result<_, _>>()?;
    assert_eq!(set_count, sets.len());
    let start = Instant::now();
    let set_cover = find_set_cover(sets.clone(), elements).expect("Failed to find valid set cover");
    let end = start.elapsed();
    println!(
        "Found minimum set cover containing {} sets in {:.6} seconds.",
        set_cover.len(),
        end.as_secs_f64()
    );
    let mut sorted_set_cover: Vec<_> = set_cover.into_iter().collect();
    sorted_set_cover.sort_unstable();
    println!("Included sets: {:?}", sorted_set_cover);
    for i in sorted_set_cover {
        let mut sorted_set_elements: Vec<_> = sets[i - 1].1.iter().collect();
        sorted_set_elements.sort_unstable();
        println!("Set #{i}: {:?}", sorted_set_elements);
    }
    Ok(())
}

fn find_set_cover(
    mut sets: Vec<(usize, HashSet<u32>)>,
    mut uncovered_elements: HashSet<u32>,
) -> Option<HashSet<usize>> {
    if uncovered_elements.is_empty() {
        return Some(HashSet::new());
    }
    for (index, (i, set)) in sets.iter().enumerate() {
        for (j, other) in &sets {
            if i != j && set.is_subset(other) {
                let mut sets = sets.to_owned();
                sets.remove(index);
                return find_set_cover(sets, uncovered_elements);
            }
        }
    }
    for &element in &uncovered_elements {
        let mut containing_set_count = 0;
        let mut containing_index = 0;
        for (index, (_, set)) in sets.iter().enumerate() {
            if set.contains(&element) {
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
                set.retain(|element| !required_set.contains(element));
            }
            new_sets.retain(|(_, set)| !set.is_empty());
            for element in &required_set {
                uncovered_elements.remove(element);
            }
            let mut cover = find_set_cover(new_sets, uncovered_elements);
            if let Some(set) = cover.as_mut() {
                set.insert(required_set_id);
            }
            return cover;
        }
    }
    let (selected_set_id, selected_set) = sets.pop().unwrap();
    let mut new_sets = sets.clone();
    let mut new_uncovered_elements = uncovered_elements.clone();
    for (_, set) in new_sets.iter_mut() {
        set.retain(|element| !selected_set.contains(element));
    }
    new_sets.retain(|(_, set)| !set.is_empty());
    for element in &selected_set {
        new_uncovered_elements.remove(element);
    }
    let mut with_selected = find_set_cover(new_sets, new_uncovered_elements);
    if let Some(set) = with_selected.as_mut() {
        set.insert(selected_set_id);
    }
    let without_selected = find_set_cover(sets, uncovered_elements);
    match (with_selected, without_selected) {
        (None, None) => None,
        (None, without @ Some(_)) => without,
        (with @ Some(_), None) => with,
        (Some(with), Some(without)) => Some(std::cmp::min_by_key(with, without, HashSet::len)),
    }
}
