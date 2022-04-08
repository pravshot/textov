extern crate rand;

use std::collections::HashMap;

use rand::Rng;

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
    pub fn new(filepath: String) {
        todo!();
    }
    // splits textfile by sentences and stores into data member(sentences)
    pub fn split_sentences(filepath: String) {

    }
    // counts unique phrases from sentences data member
    pub fn count_unique_phrases(self) {
        todo!();
    }
    // creates phrase->idx maps
    pub fn fill_maps(self) {
        todo!();
    }
    // populates markov matrix 
    pub fn fill_matrix(self) {
        todo!();
    }
    // normalizes a vector with norm p=1
    // normalize vector
    pub fn normalize_vector(self, vect: &mut Vec<f64>) {
        let sum: f64 = vect.iter().sum();
        for i in 0..vect.len() {
            vect[i] /= sum;
        }
    }
    // normalizes markov matrix by row
    pub fn normalize_matrix(self) {
        for i in 0..self.markov_matrix.len() {
            self.normalize_vector(&mut self.markov_matrix[i]);
        }
    }
    // make weighted random choice from a vector
    pub fn make_weighted_choice(self, vect: &Vec<f64>) -> usize {
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
    // generates a sentence from markov matrix
    pub fn generate_sentence(self) -> String {
        todo!();
    }
    // generates text with num_sentences from markov matrix
    pub fn generate_text(self, num_sentences: usize) -> String {
        todo!();
    }

}