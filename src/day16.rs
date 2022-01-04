use std::str::FromStr;

use scan_fmt::{parse::ScanError, scan_fmt};

#[derive(Debug)]
struct Sue {
    number: usize,
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

impl Sue {
    fn empty(number: usize) -> Sue {
        Sue {
            number,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }

    fn matches(&self, template: &Sue) -> bool {
        (self.children.is_none() || self.children == template.children)
            && (self.cats.is_none() || self.cats == template.cats)
            && (self.samoyeds.is_none() || self.samoyeds == template.samoyeds)
            && (self.pomeranians.is_none() || self.pomeranians == template.pomeranians)
            && (self.akitas.is_none() || self.akitas == template.akitas)
            && (self.vizslas.is_none() || self.vizslas == template.vizslas)
            && (self.goldfish.is_none() || self.goldfish == template.goldfish)
            && (self.trees.is_none() || self.trees == template.trees)
            && (self.cars.is_none() || self.cars == template.cars)
            && (self.perfumes.is_none() || self.perfumes == template.perfumes)
    }

    fn matches_ranged(&self, template: &Sue) -> bool {
        (self.children.is_none() || self.children == template.children)
            && (self.cats.is_none() || self.cats.unwrap() > template.cats.unwrap())
            && (self.samoyeds.is_none() || self.samoyeds == template.samoyeds)
            && (self.pomeranians.is_none()
                || self.pomeranians.unwrap() < template.pomeranians.unwrap())
            && (self.akitas.is_none() || self.akitas == template.akitas)
            && (self.vizslas.is_none() || self.vizslas == template.vizslas)
            && (self.goldfish.is_none() || self.goldfish.unwrap() < template.goldfish.unwrap())
            && (self.trees.is_none() || self.trees.unwrap() > template.trees.unwrap())
            && (self.cars.is_none() || self.cars == template.cars)
            && (self.perfumes.is_none() || self.perfumes == template.perfumes)
    }
}

impl FromStr for Sue {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        scan_fmt!(s, "Sue {d}: {[^\n]}", usize, String).map(|(number, sue_props)| {
            sue_props
                .split(", ")
                .fold(Sue::empty(number), |mut sue, sue_prop| {
                    // todo: really this should return the error
                    let (prop, n) = scan_fmt!(sue_prop, "{}: {d}", String, u8).unwrap();
                    match prop.as_str() {
                        "children" => sue.children = Some(n),
                        "cats" => sue.cats = Some(n),
                        "samoyeds" => sue.samoyeds = Some(n),
                        "pomeranians" => sue.pomeranians = Some(n),
                        "akitas" => sue.akitas = Some(n),
                        "vizslas" => sue.vizslas = Some(n),
                        "goldfish" => sue.goldfish = Some(n),
                        "trees" => sue.trees = Some(n),
                        "cars" => sue.cars = Some(n),
                        "perfumes" => sue.perfumes = Some(n),
                        _ => panic!("invalid sue prop"),
                    }
                    sue
                })
        })
    }
}

// This is not part of the input for some reason
const TEMPLATE_SUE: Sue = Sue {
    number: 0,
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .find_map(|line| {
            let sue: Sue = line.parse().unwrap();
            if sue.matches(&TEMPLATE_SUE) {
                Some(sue.number)
            } else {
                None
            }
        })
        .unwrap() as i64
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .find_map(|line| {
            let sue: Sue = line.parse().unwrap();
            if sue.matches_ranged(&TEMPLATE_SUE) {
                Some(sue.number)
            } else {
                None
            }
        })
        .unwrap() as i64
}
