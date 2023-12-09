use std::{collections::HashMap, ops::AddAssign};

use super::Solve;

#[cfg(not(test))]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day7.txt"));
#[cfg(test)]
const INPUT_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day7-test.txt"));

pub struct Solver;

impl Solve for Solver {
    type Value = usize;

    fn solve_part_one(&self) -> Self::Value {
        let mut hands: Vec<_> = INPUT_FILE
            .lines()
            .map(|line| {
                let (cards, bid) = line.split_once(' ').unwrap();

                let mut card_map = HashMap::<Card, usize>::new();
                let cards: Vec<_> = cards
                    .chars()
                    .map(|c| match c {
                        '2' => {
                            card_map
                                .entry(Card::C2)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::C2
                        }
                        '3' => {
                            card_map
                                .entry(Card::C3)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::C3
                        }
                        '4' => {
                            card_map
                                .entry(Card::C4)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::C4
                        }
                        '5' => {
                            card_map
                                .entry(Card::C5)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::C5
                        }
                        '6' => {
                            card_map
                                .entry(Card::C6)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::C6
                        }
                        '7' => {
                            card_map
                                .entry(Card::C7)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::C7
                        }
                        '8' => {
                            card_map
                                .entry(Card::C8)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::C8
                        }
                        '9' => {
                            card_map
                                .entry(Card::C9)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::C9
                        }
                        'T' => {
                            card_map
                                .entry(Card::CT)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::CT
                        }
                        'J' => {
                            card_map
                                .entry(Card::CJ)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::CJ
                        }
                        'Q' => {
                            card_map
                                .entry(Card::CQ)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::CQ
                        }
                        'K' => {
                            card_map
                                .entry(Card::CK)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::CK
                        }
                        'A' => {
                            card_map
                                .entry(Card::CA)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card::CA
                        }
                        c => unreachable!("unexpected char {c}"),
                    })
                    .collect();

                let ty = {
                    let singles = card_map.values().filter(|&&v| v == 1).count();
                    let pairs = card_map.values().filter(|&&v| v == 2).count();
                    let triplets = card_map.values().filter(|&&v| v == 3).count();
                    let quadruplets = card_map.values().filter(|&&v| v == 4).count();
                    let quintuplets = card_map.values().filter(|&&v| v == 5).count();

                    if quintuplets == 1 {
                        HandType::FiveOfAKind
                    } else if quadruplets == 1 {
                        HandType::FourOfAKind
                    } else if triplets == 1 && pairs == 1 {
                        HandType::FullHouse
                    } else if triplets == 1 && pairs == 0 {
                        HandType::ThreeOfAKind
                    } else if pairs == 2 {
                        HandType::TwoPair
                    } else if pairs == 1 && triplets == 0 {
                        HandType::OnePair
                    } else if singles == 5 {
                        HandType::HighCard
                    } else {
                        unreachable!()
                    }
                };

                Hand {
                    cards,
                    ty,
                    bid: bid.parse::<usize>().unwrap(),
                }
            })
            .collect();

        hands.sort();

        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, hand)| acc + hand.bid * (index + 1))
    }

    fn solve_part_two(&self) -> Self::Value {
        let mut hands: Vec<_> = INPUT_FILE
            .lines()
            .map(|line| {
                let (cards, bid) = line.split_once(' ').unwrap();

                let mut card_map = HashMap::<Card2, usize>::new();
                let cards: Vec<_> = cards
                    .chars()
                    .map(|c| match c {
                        '2' => {
                            card_map
                                .entry(Card2::C2)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::C2
                        }
                        '3' => {
                            card_map
                                .entry(Card2::C3)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::C3
                        }
                        '4' => {
                            card_map
                                .entry(Card2::C4)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::C4
                        }
                        '5' => {
                            card_map
                                .entry(Card2::C5)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::C5
                        }
                        '6' => {
                            card_map
                                .entry(Card2::C6)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::C6
                        }
                        '7' => {
                            card_map
                                .entry(Card2::C7)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::C7
                        }
                        '8' => {
                            card_map
                                .entry(Card2::C8)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::C8
                        }
                        '9' => {
                            card_map
                                .entry(Card2::C9)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::C9
                        }
                        'T' => {
                            card_map
                                .entry(Card2::CT)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::CT
                        }
                        'J' => {
                            card_map
                                .entry(Card2::CJ)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::CJ
                        }
                        'Q' => {
                            card_map
                                .entry(Card2::CQ)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::CQ
                        }
                        'K' => {
                            card_map
                                .entry(Card2::CK)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::CK
                        }
                        'A' => {
                            card_map
                                .entry(Card2::CA)
                                .and_modify(|v| v.add_assign(1))
                                .or_insert(1);
                            Card2::CA
                        }
                        c => unreachable!("unexpected char {c}"),
                    })
                    .collect();

                let ty = {
                    let singles = card_map.values().filter(|&&v| v == 1).count();
                    let pairs = card_map.values().filter(|&&v| v == 2).count();
                    let triplets = card_map.values().filter(|&&v| v == 3).count();
                    let quadruplets = card_map.values().filter(|&&v| v == 4).count();
                    let quintuplets = card_map.values().filter(|&&v| v == 5).count();

                    let jokers = card_map
                        .iter()
                        .filter(|(&k, _)| k == Card2::CJ)
                        .map(|(_, v)| v)
                        .sum::<usize>();

                    if quintuplets == 1 {
                        HandType::FiveOfAKind
                    } else if quadruplets == 1 {
                        if jokers > 0 {
                            HandType::FiveOfAKind
                        } else {
                            HandType::FourOfAKind
                        }
                    } else if triplets == 1 && pairs == 1 {
                        if jokers >= 2 {
                            HandType::FiveOfAKind
                        } else {
                            HandType::FullHouse
                        }
                    } else if triplets == 1 && pairs == 0 {
                        if jokers > 0 {
                            HandType::FourOfAKind
                        } else {
                            HandType::ThreeOfAKind
                        }
                    } else if pairs == 2 {
                        if jokers == 1 {
                            HandType::FullHouse
                        } else if jokers == 2 {
                            HandType::FourOfAKind
                        } else {
                            HandType::TwoPair
                        }
                    } else if pairs == 1 && triplets == 0 {
                        if jokers > 0 {
                            HandType::ThreeOfAKind
                        } else {
                            HandType::OnePair
                        }
                    } else if singles == 5 {
                        if jokers > 0 {
                            HandType::OnePair
                        } else {
                            HandType::HighCard
                        }
                    } else {
                        unreachable!()
                    }
                };

                Hand {
                    cards,
                    ty,
                    bid: bid.parse::<usize>().unwrap(),
                }
            })
            .collect();

        hands.sort();

        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, hand)| acc + hand.bid * (index + 1))
    }
}

#[derive(Debug, Eq)]
struct Hand<C: Eq + PartialEq + Ord + PartialOrd> {
    cards: Vec<C>,
    ty: HandType,
    bid: usize,
}

impl<C: Eq + PartialEq + Ord + PartialOrd> PartialEq for Hand<C> {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.cards == other.cards
    }
}

impl<C: Eq + PartialEq + Ord + PartialOrd> Ord for Hand<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.ty.cmp(&other.ty) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl<C: Eq + PartialEq + Ord + PartialOrd> PartialOrd for Hand<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card2 {
    CJ,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(Solver.solve_part_one(), 6440);
        assert_eq!(Solver.solve_part_two(), 5905);
    }
}
