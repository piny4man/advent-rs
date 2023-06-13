use std::{fs, path::Path};

fn main() {
    let parsed_sections =
        fs::read_to_string(Path::new("section-pairs.txt")).expect("Should contain section pairs");

    println!(
        "Total contained sections: {:?}",
        compare_section_pairs(parsed_sections)
    );
}

fn compare_section_pairs(sections: String) -> i32 {
    let mut fully_contained_section_count: i32 = 0;
    for pair in sections.lines() {
        let pair_sections = pair.split(',').collect::<Vec<_>>();
        let first_section_range = pair_sections[0].split('-').collect::<Vec<_>>();
        let second_section_range = pair_sections[1].split('-').collect::<Vec<_>>();
        let first_section_start = first_section_range[0].parse::<i32>().unwrap();
        let first_section_end = first_section_range[1].parse::<i32>().unwrap();
        let second_section_start = second_section_range[0].parse::<i32>().unwrap();
        let second_section_end = second_section_range[1].parse::<i32>().unwrap();
        if (first_section_start <= second_section_start && first_section_end >= second_section_end)
            || (second_section_start <= first_section_start
                && second_section_end >= first_section_end)
        {
            fully_contained_section_count += 1;
        }
    }
    fully_contained_section_count
}
