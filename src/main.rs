use std::collections::HashMap;


struct SuggestTree<'a> {
    root: CompletionTrie<'a>,
    completion_table: Vec<&'a str>,
    inverted_index: HashMap<&'a str, uint>,
    last_index: uint
}


struct CompletionTrie<'a> {
    children: HashMap<&'a str, Option<&'a CompletionTrie<'a>>>,
}


impl<'a> SuggestTree<'a> {
/*    fn new() -> SuggestTree<'a> {
        SuggestTree<'a> {
            root: CompletionTrie<'a>,
        }
    }
*/
    fn insert(&self, word: &str, index: uint) {

    }

}


fn main() {
    println!("Hello, world!")
}
