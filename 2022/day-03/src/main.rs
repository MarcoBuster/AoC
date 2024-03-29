use std::fs;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq)]
struct Item(char);

impl Item {
    fn new(c: char) -> Self {
        assert!(c.is_ascii_alphabetic());
        Item(c)
    }

    fn priority(&self) -> u32 {
        if self.0.is_ascii_uppercase() {
            self.0 as u32 - 65 + 27
        } else {
            self.0 as u32 - 97 + 1
        }
    }
}

type Compartment = Vec<Item>;

struct Rucksack(Compartment, Compartment);

impl Rucksack {
    fn new(items: &str) -> Self {
        let (compartment_1, compartment_2) = items.split_at(items.len() / 2);
        Rucksack(
            compartment_1.chars().map(Item::new).collect(),
            compartment_2.chars().map(Item::new).collect(),
        )
    }

    fn common_item(&self) -> Option<&Item> {
        self.0
            .iter()
            .find(|c| self.1.contains(c))
    }

    fn contents(&self) -> Vec<&Item> {
        self.0
            .iter()
            .chain(self.1.iter())
            .collect()
    }
}

struct Group(Rucksack, Rucksack, Rucksack);

impl Group {
    fn new(items_1: &str, items_2: &str, items_3: &str) -> Self {
        Group(
            Rucksack::new(items_1),
            Rucksack::new(items_2),
            Rucksack::new(items_3),
        )
    }

    fn common_item(&self) -> Option<&Item> {
        self.0
            .contents()
            .iter()
            .find(|c| self.1.contents().contains(c) && self.2.contents().contains(c))
            .copied()
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lines = contents.lines();

    let mut part_1 = 0;
    let mut part_2 = 0;

    while let (Some(line_1), Some(line_2), Some(line_3)) =
        (lines.next(), lines.next(), lines.next())
    {
        let group = Group::new(line_1, line_2, line_3);

        [&group.0, &group.1, &group.2].map(|r| part_1 += r.common_item().unwrap().priority());
        part_2 += group.common_item().unwrap().priority();
    }

    println!("Part 1: {}", part_1);
    assert_eq!(part_1, 7824);

    println!("Part 2: {}", part_2);
    assert_eq!(part_2, 2798);
}
