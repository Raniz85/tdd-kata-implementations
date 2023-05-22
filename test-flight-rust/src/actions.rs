use std::ptr::copy_nonoverlapping;
use crate::alpha::Alpha;

pub enum Action {
    A,
    B,
    C,
    D,
    E,
    F,
}

fn reverse(input: [Alpha; 16]) -> [Alpha; 16] {
    let mut transformed = input.clone();
    transformed.reverse();
    transformed
}

fn consonant_rot13(input: [Alpha; 16]) -> [Alpha; 16] {
    input.map(|a| if !a.is_vowel() {
        a.rot13()
    } else {
        a
    })
}

fn swap_vowels(input: [Alpha; 16]) -> [Alpha; 16] {
    let mut transformed = input.clone();
    for i in 1..16 {
        if transformed[i].is_vowel() {
            let tmp = transformed[i-1];
            transformed[i-1] = transformed[i];
            transformed[i] = tmp;
        }
    }
    transformed
}

fn combine_positions(input: [Alpha; 16]) -> [Alpha; 16] {
    let mut transformed = input.clone();
    for i in (0..16).step_by(2) {
        let a = transformed[i];
        let b = transformed[i+1];
        let c = &a + &b;
        transformed[i] = c.clone();
        transformed[i+1] = c;
    }
    transformed
}

fn swap_back_front(input: [Alpha; 16]) -> [Alpha; 16] {
    input[8..].iter().chain(input[..8].iter()).cloned().collect::<Vec<_>>().try_into().unwrap()
}

fn even_rot13(input: [Alpha; 16]) -> [Alpha; 16] {
    input.iter().enumerate().map(|(index, c)| {
        if (index + 1) % 2 == 0 {
            c.rot13()
        } else {
            c.clone()
        }
    }).collect::<Vec<_>>().try_into().unwrap()
}

impl Action {
    pub fn new(action: Alpha) -> Option<Action> {
        match action.0 {
            1 => Some(Action::A),
            2 => Some(Action::B),
            3 => Some(Action::C),
            4 => Some(Action::D),
            5 => Some(Action::E),
            6 => Some(Action::F),
            _ => None,
        }
    }

    pub fn transform(&self, input: [Alpha; 16]) -> [Alpha; 16] {
        match self {
            Action::A => swap_vowels(consonant_rot13(reverse(input))),
            Action::B => swap_back_front(even_rot13(combine_positions(input))),
            Action::C => swap_vowels(combine_positions(consonant_rot13(input))),
            Action::D => combine_positions(reverse(swap_back_front(input))),
            Action::E => reverse(even_rot13(swap_vowels(input))),
            Action::F => consonant_rot13(swap_vowels(even_rot13(input))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::identity;
    use crate::actions::{combine_positions, consonant_rot13, reverse, swap_back_front, swap_vowels, even_rot13};
    use crate::alpha::Alpha;

    fn to_alphas(source: &str) -> [Alpha; 16] {
        source.chars().map(Alpha::new)
            .filter_map(identity)
            .collect::<Vec<_>>()
            .try_into().unwrap()
    }

    #[test]
    pub fn test_transforms() {
        let items: &[(fn([Alpha; 16]) -> [Alpha; 16], &str)] = &[
            (reverse, "PONMLKJIHGFEDCBA"),
            (consonant_rot13, "AOPQESTUIWXYZAOC"),
            (swap_vowels, "ABCEDFGIHJKLMONP"),
            (combine_positions, "CCGGKKOOSSWWAAEE"),
            (swap_back_front, "IJKLMNOPABCDEFGH"),
            (even_rot13, "AOCQESGUIWKYMAOC"),
        ];
        for (transform, expected) in items {
            // Given an input string
            let input: [Alpha; 16] = to_alphas("ABCDEFGHIJKLMNOP");

            // When the string is transformed
            let result = transform(input);

            // Then it should match the expected output
            assert_eq!(to_alphas(expected), result);
        }
    }
}
