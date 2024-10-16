use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

const HAND_STRENGTH: [usize; 7] = [5, 7, 9, 11, 13, 17, 25];
const CARD_STRENGTH: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lines = file.split("\n");

    let mut hands: Vec<(&str, usize, usize)> = lines.map(|line| {
        let mut line = line.split_whitespace();
        let hand = line.next().unwrap();
        let strength = get_strength(hand);
        let bid: usize = line.next().unwrap().parse().unwrap();

        (hand, strength, bid)
    }).collect();

    hands.sort_by(order_by_strength);

    let mut total_winnings = 0;

    for (i, (_, _, bid)) in hands.iter().enumerate() {
        total_winnings += (i + 1) * bid
    }

    println!("{}", total_winnings);
}

fn order_by_strength((hand_a, hand_strength_a, _): &(&str, usize, usize), (hand_b, hand_strength_b, _): &(&str, usize, usize)) -> Ordering {
    if hand_strength_a != hand_strength_b { return hand_strength_a.cmp(hand_strength_b); }

    for (char_a, char_b) in hand_a.chars().zip(hand_b.chars()) {
        if char_a == char_b { continue; }

        let strength_a = CARD_STRENGTH.iter().position(|&char| char == char_a).unwrap();
        let strength_b = CARD_STRENGTH.iter().position(|&char| char == char_b).unwrap();

        return strength_a.cmp(&strength_b);
    }

    Ordering::Equal
}

fn get_strength(hand: &str) -> usize {
    let chars: Vec<_> = hand.chars().collect();
    let mut hash = count_chars(&chars);

    apply_joker(&mut hash);

    hash.into_values().fold(0, |acc, count| acc + count.pow(2))
}

fn apply_joker(hash: &mut HashMap<char, usize>) {
    let j = *hash.get(&'J').unwrap_or(&0);

    if j == 0 || j == 5 { return; }

    let max = hash.iter().filter(|(&key, _)| key != 'J').max_by_key(|(_, &strength)| strength);

    if let Some((&key, _)) = max {
        hash.entry(key).and_modify(|count| *count += j);
        hash.remove_entry(&'J');
        return;
    }

    let first_key = *hash.keys().find(|&key| key != &'J').unwrap();
    hash.entry(first_key).and_modify(|count| *count += j);
    hash.remove_entry(&'J');
}

fn count_chars(chars: &[char]) -> HashMap<char, usize> {
    let mut hash = HashMap::new();

    for char in chars {
        *hash.entry(*char).or_insert(0) += 1;
    }

    hash
}
