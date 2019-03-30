use std::collections::HashSet;
use std::vec::Vec;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn signature(word: &String) -> HashSet<char> {
    let mut sig = HashSet::new();
    let char_vec: Vec<char> = word.chars().collect();
    for current in  char_vec {
        sig.insert(current);
    }

    return sig;
}

fn candidates(filename: &String, searching: &String) -> Vec<String> {
    let mut result = Vec::<String>::new();

    let sig = signature(&searching);
    let search_lenght = searching.len();
    println!("signature : {:?} ; lenght : {}", sig, search_lenght);

    let file = File::open(filename);
    for line in BufReader::new(file.unwrap()).lines() {
        let current = line.unwrap();
        if current.len() < search_lenght{
            let current_sig = signature(&current);
            if current_sig.is_subset(&sig) {
                result.push(current);
            }
        }
    }
    return result;
}

fn sort(word: &String) -> Vec<char> {
    let mut result: Vec<char> = word.chars().collect();
    result.sort();

    return result;
}

fn find_anagrams(searching: &String, to_examined: &Vec<String>){
    let searching_order = sort(&searching);
    let searching_len = searching.len();

    for part1 in to_examined {
        for part2 in to_examined {
            let mut current = part1.clone();
            current.push_str(&part2);
            if part1.len() + part2.len() == searching_len {
                let order = sort(&current);
                if order == searching_order{
                    println!("{} = {} + {}", searching, part1, part2);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let dict = args[0].clone();
    let searching = args[1].clone();

    println!("searching {} in {}", searching, dict);

    let to_examined = candidates(&dict, &searching);
    println!("{} candidates", to_examined.len());

    find_anagrams(&searching, &to_examined);
}
