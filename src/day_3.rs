/**
--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey.
Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to
be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one
of the two compartments. The Elf that did the packing failed to follow this rule for exactly one
item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but
they need your help finding the errors. Every item type is identified by a single lowercase or
uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack
always has the same number of items in each of its two compartments, so the first half of the
characters represent items in the first compartment, while the second half of the characters
represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments of each
rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the
priorities of those item types?
*/
pub fn calculate_sum_of_rucksack_priorities(list_text: &str) -> i32 {
    list_text.lines().fold(0, |score, line| {
        let rucksack = Rucksack::new(line);
        let priority = get_priority_for_item(rucksack.element);

        score + priority
    })
}

pub fn get_priority_for_item(item: char) -> i32 {
    match item.is_ascii_lowercase() {
        true => item as i32 - 96,
        false => item as i32 - 38,
    }
}

pub struct Rucksack<'a> {
    first: &'a str,
    second: &'a str,
    pub element: char,
}

impl<'a> Rucksack<'a> {
    fn find_common_item(&self) -> char {
        for ch in self.first.chars() {
            if self.second.contains(ch) {
                return ch;
            }
        }
        self.element
    }

    pub fn new(contents: &'a str) -> Self {
        let midpoint = contents.len() / 2;
        let mut r = Rucksack {
            first: &contents[0..midpoint],
            second: &contents[midpoint..],
            element: '_',
        };
        r.element = r.find_common_item();
        r
    }
}
