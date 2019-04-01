use std::collections::HashSet;
use std::vec::Vec;

use std::fs::File;
use std::io::{BufRead, BufReader};


pub trait StringExt {
    fn signature(&self) -> HashSet<char> ;
    fn sorted(&self) -> Vec<char>;
}

impl StringExt for String{

    fn signature(&self) -> HashSet<char> {
        self.chars().collect()
    }

    fn sorted(&self) -> Vec<char> {
        let mut result: Vec<char> = self.chars().collect();
        result.sort();

        return result;
    }

}

fn candidates(filename: &String, searching: &String) -> Vec<String> {
    let mut result = Vec::<String>::new();

    let sig = &searching.signature();
    let search_lenght = searching.len();
    println!("signature : {:?} ; lenght : {}", sig, search_lenght);

    let file = File::open(filename);
    for line in BufReader::new(file.unwrap()).lines() {
        let current = line.unwrap();
        if current.len() < search_lenght{
            let current_sig = &current.signature();
            if current_sig.is_subset(&sig) {
                result.push(current);
            }
        }
    }
    return result;
}


fn find_anagrams(searching: &String, to_examined: &Vec<String>){
    let searching_order = &searching.sorted();
    let searching_len = searching.len();

    for index1 in 0..to_examined.len(){
        let part1 = &to_examined[index1];
        for index2 in index1 + 1..to_examined.len() {
            let part2 = &to_examined[index2];

            if part1.len() + part2.len() == searching_len {
                let mut current = part1.clone();
                current.push_str(&part2);

                let order = &current.sorted();
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
