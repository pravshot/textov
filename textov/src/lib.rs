extern crate rand;
extern crate rayon;

use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

use rand::Rng;
use rayon::prelude::*;

pub struct Textov {
    num_unique_phrases: usize,
    markov_matrix: Vec<Vec<f64>>,
    phrase_idx_map: HashMap<String, usize>,
    idx_phrase_map: HashMap<usize, String>,
    sentences: Vec<String>
    //k_order: enum maybe usize <-- future add-on
}

impl Textov {
    // constructor, will fill up all data members with helper functions
    pub fn new(filepath: String) -> Self {
        let sentences = split_sentences(filepath);
        let (phrase_idx_map, idx_phrase_map, num_unique_phrases) = create_maps(&sentences);
        let mut markov_matrix = create_markov_matrix(&sentences, &phrase_idx_map, num_unique_phrases);
        normalize_matrix(&mut markov_matrix);
        Textov {
            num_unique_phrases,
            markov_matrix,
            phrase_idx_map,
            idx_phrase_map,
            sentences
        }
    }
    // generates a sentence from markov matrix
    pub fn generate_sentence(&self) -> String {
        // O is START, 1 is END
        let mut cur_idx = make_weighted_choice(&self.markov_matrix[0]);
        let mut res = self.idx_phrase_map[&cur_idx].clone();
        cur_idx = make_weighted_choice(&self.markov_matrix[cur_idx]);
        while cur_idx != 1 {
            res.push(' ');
            res.push_str(&self.idx_phrase_map[&cur_idx]);
            cur_idx = make_weighted_choice(&self.markov_matrix[cur_idx]);
        }
        res
    }
    // generates text with num_sentences from markov matrix
    pub fn generate_text(&self, num_sentences: usize) -> String {
        let mut res = String::new();
        for _ in 0..num_sentences {
            res.push_str(self.generate_sentence().as_str());
            res.push(' ');
        }
        res
    }
    // prints all data members
    pub fn print_all(&self) {
        println!("num_unique_phrase {:?} \n", self.num_unique_phrases);
        println!("phrase_idx_map {:?} \n", self.phrase_idx_map);
        println!("idx_phrase_map {:?} \n", self.idx_phrase_map);
        println!("markov_matrix {:?} \n ", self.markov_matrix);
        println!("sentences {:?} \n", self.sentences);
    }
}

// returns vector of sentences from filepath
pub fn split_sentences(filepath: String) -> Vec<String> {
    let mut sentences: Vec<String> = Vec::new();
    let cleaned_contents: String = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let mut curr : String = String::new();
    for i in 0..cleaned_contents.len() {
        curr = curr + &cleaned_contents[i..i+1];
        if &cleaned_contents[i..i+1] == "." || &cleaned_contents[i..i+1] == "!" || &cleaned_contents[i..i+1] == "?" {
            sentences.push(curr);
            curr = String::new();
        }
    }
    sentences
}

// creates phrase/idx maps and counts unique phrases
pub fn create_maps(sentences: &Vec<String>) -> (HashMap<String, usize>, HashMap<usize, String>, usize) {
    let mut phrase_idx_map: HashMap<String, usize> = HashMap::new();
    let mut idx_phrase_map: HashMap<usize, String> = HashMap::new();
    let mut num_unique_phrases: usize = 0;
    let mut idx: usize = 2;
    for sentence in sentences {
        for phrase in sentence.split_whitespace() {
            if !phrase_idx_map.contains_key(phrase) {
                phrase_idx_map.insert(phrase.to_string(), idx);
                idx_phrase_map.insert(idx, phrase.to_string());
                idx+=1;
                num_unique_phrases+=1;
            }
        }
    }
    (phrase_idx_map, idx_phrase_map, num_unique_phrases)
}

// creates un-normalized markov matrix 
pub fn create_markov_matrix(sentences: &Vec<String>, phrase_idx_map: &HashMap<String, usize>, num_unique_phrases: usize) -> Vec<Vec<f64>> {
    // 0 is START, 1 is END
    let mut markov_matrix = vec![vec![0.0; num_unique_phrases + 2]; num_unique_phrases + 2];
    for sentence in sentences {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        for i in 0..words.len() {
            // start word 
            if i == 0 {
                markov_matrix[0][phrase_idx_map[words[i]]] += 1.0;
            }
            // end word
            if i == words.len() - 1 {
                markov_matrix[phrase_idx_map[words[i - 1]]][phrase_idx_map[words[i]]] += 1.0;
                markov_matrix[phrase_idx_map[words[i]]][1] += 1.0;
            }
            // middle word
            if i != 0 && i != words.len() - 1 {
                markov_matrix[phrase_idx_map[words[i - 1]]][phrase_idx_map[words[i]]] += 1.0;
            }
        }
    }
    markov_matrix
}

// create un-normalized markov matrix using parallelism/concurrency with Arc/Mutex
pub fn create_markov_matrix_with_concurrency(sentences: &Vec<String>, phrase_idx_map: &HashMap<String, usize>, num_unique_phrases: usize) -> Vec<Vec<f64>> {
    // 0 is START, 1 is END
    let markov_matrix = vec![vec![0.0; num_unique_phrases + 2]; num_unique_phrases + 2];
    let markov_matrix_arc = Arc::new(Mutex::new(markov_matrix));
    sentences.par_iter().for_each(|sentence| {
        let markov_matrix_arced = markov_matrix_arc.clone();
        let mut markov_matrix= markov_matrix_arced.lock().unwrap();
        let words: Vec<&str> = sentence.split_whitespace().collect();
        for i in 0..words.len() {
            // start word 
            if i == 0 {
                (*markov_matrix)[0][phrase_idx_map[words[i]]] += 1.0;
            }
            // end word
            if i == words.len() - 1 {
                (*markov_matrix)[phrase_idx_map[words[i]]][1] += 1.0;
                (*markov_matrix)[phrase_idx_map[words[i - 1]]][phrase_idx_map[words[i]]] += 1.0;
            }
            // middle word
            if i != 0 && i != words.len() - 1 {
                (*markov_matrix)[phrase_idx_map[words[i - 1]]][phrase_idx_map[words[i]]] += 1.0;
            }
        }
    });
    let result = markov_matrix_arc.lock().unwrap();
    result.clone()
}

// normalizes matrix by row
pub fn normalize_matrix(matrix: &mut Vec<Vec<f64>>) {
    for row in matrix.iter_mut() {
        let sum: f64 = row.iter().sum();
        if sum != 0.0 {
            for i in 0..row.len() {
                row[i] /= sum;
            }
        }
    }
}

// normalizes matrix by row using parallelism/concurrency with rayon crate
pub fn normalize_matrix_with_concurrency(matrix: &mut Vec<Vec<f64>>) {
    matrix.par_iter_mut().for_each(|row| {
        let sum: f64 = row.iter().sum();
        if sum != 0.0 {
            for i in 0..row.len() {
                row[i] /= sum;
            }
        }
    });
}

// make weighted random choice from a vector of probabiliities, returns index of choice
pub fn make_weighted_choice(vect: &Vec<f64>) -> usize {
    let mut rng = rand::thread_rng();
    let mut cum_prob: f64 = 0.0;
    let mut choice: usize = 0;
    let rand_prob: f64 = rng.gen();
    for i in 0..vect.len() {
        cum_prob += vect[i];
        if rand_prob < cum_prob {
            choice = i;
            break;
        }
    }
    choice
}
