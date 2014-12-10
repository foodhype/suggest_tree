use std::collections::HashMap;
use std::io::BufferedReader;
use std::io::File;
use std::io;
struct SuggestTree<'a> {
    root: CompletionTrie<'a>,
    completion_table: Vec<String>,
    //inverted_index: HashMap<&'a str, uint>,
    last_index: uint
}


struct CompletionTrie<'a> {
    children: HashMap<char, Box<CompletionTrie<'a>>>,
    completion_weights: HashMap<uint, uint>
}

impl<'a> CompletionTrie<'a> {
    fn new() -> CompletionTrie<'a> {
        CompletionTrie {
            children: HashMap::new(),
            completion_weights: HashMap::new(),
        }
    }

    fn add(&mut self, word: &str, index: uint) {
        if word.len() == 0 {
            return;
        }

        let letter = word.char_at(0);
        if !self.children.contains_key(&letter) {
            self.children.insert(letter, box CompletionTrie::new());
        }
      
        let mut y = match self.children.get_mut(&letter) {
            Some(n) => n,
            None => {
                return;
            }
        };

        self.completion_weights.insert(index, 1); 
        y.add(word.slice(1, word.len()), index);
    }

    fn get_indexes(& mut self, prefix: &str) -> Option<& HashMap<uint, uint>> {
        if prefix.len() == 0 {
            return Some(&self.completion_weights);
        }
        
        let letter = prefix.char_at(0);

        match self.children.get_mut(&letter) {
            Some(n) => return n.get_indexes(prefix.slice(1, prefix.len())),
            None => {
                return None
            }
        };

    }

}   

impl<'a> SuggestTree<'a> {
    fn new() -> SuggestTree<'a> {
        SuggestTree {
            root: CompletionTrie::new(),
            completion_table: Vec::new(),
            //inverted_index: HashMap::new(),
            last_index: 0
        }
    }

    fn add(&mut self, word: &'a str) {
        self.root.add(word, self.last_index);
        self.completion_table.push(String::from_str(word));
        self.last_index += 1;
    }

    fn get_weights(& mut self, prefix: &str) -> HashMap<&String, uint> {
        let mut map = HashMap::new();
        match self.root.get_indexes(prefix) {
            Some(n) => {
                for (k, v) in n.iter() {
                    map.insert(&self.completion_table[*k], *v);
                }
            },
            None => {
                println!("No words for that prefix");
            }
        };
        
        return map;
    }
}

fn main() {
    let mut y = SuggestTree::new();
    let path = Path::new("/usr/share/dict/words");
    let mut file = BufferedReader::new(File::open(&path));
    println!("Starting to build dictionary.");
    for line in file.lines().filter_map(|result| result.ok()) {
        y.add(line.as_slice().trim());
    }
    println!("Finished building dictionary");
    /*
    loop {
        println!("\nEnter a prefix:");
        let input = io::stdin().read_line().ok().expect("Failed to readline");
        let t_input = input.as_slice().trim();
        let d = y.get_weights(t_input);
        println!("Words for prefix {}:", t_input);
        for k in d.keys() {
            println!("{}", k);
        }
    }*/
}
