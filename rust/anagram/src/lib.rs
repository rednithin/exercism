use std::collections::{HashMap, HashSet};

pub fn counter(word: &str) -> HashMap<char, u32> {
    let mut letter_freq: HashMap<char, u32> = HashMap::new();
    word.chars().for_each(|c| {
        let freq = letter_freq.get(&c).unwrap_or(&0) + 1;
        letter_freq.insert(c, freq);
    });
    letter_freq
}

pub fn are_counters_equal(counter: &HashMap<char, u32>, candidate: &HashMap<char, u32>) -> bool {
    if counter.keys().len() == candidate.keys().len() {
        counter.keys().all(|c| counter.get(c) == candidate.get(c))
    } else {
        false
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let letter_freq = counter(&word);

    let mut anagrams: HashSet<&'a str> = HashSet::new();
    possible_anagrams
        .iter()
        .for_each(|possible_anagram: &&'a str| {
            let candidate = possible_anagram.to_lowercase();
            let letter_freq_candidate = counter(&candidate);
            if candidate != word && are_counters_equal(&letter_freq, &letter_freq_candidate) {
                anagrams.insert(possible_anagram);
            }
        });
    anagrams
}
