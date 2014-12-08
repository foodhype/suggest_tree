use std::collections::HashMap;


struct SuggestTree<'a> {
    root: CompletionTrie<'a>,
    completion_table: Vec<&'a str>,
    inverted_index: HashMap<&'a str, uint>,
    last_index: uint
}


struct CompletionTrie<'a> {
    children: HashMap<char, Box<CompletionTrie<'a>>>,
}

impl<'a> CompletionTrie<'a> {
    fn new() -> CompletionTrie<'a> {
        CompletionTrie {
            children: HashMap::new(),
        }
    }

    fn add(& mut self, word: &str, index: uint) {
        let mut x = self;
        for letter in word.chars() {
            if !x.children.contains_key(&letter) {
                x.children.insert(letter, box CompletionTrie::new());
            }
            
            println!("{}", letter);
        }
    }
}   

impl<'a> SuggestTree<'a> {
    fn new() -> SuggestTree<'a> {
        SuggestTree {
            root: CompletionTrie::new(),
            completion_table: Vec::new(),
            inverted_index: HashMap::new(),
            last_index: 0
        }
    }

    fn insert(&self, word: &str, index: uint) {
    
    }
}

fn main() {
    let mut x = CompletionTrie::new();
    let y = CompletionTrie::new();
    //x.children.insert("YOLO", Some(&y));
    x.add("hello", 0);
}
