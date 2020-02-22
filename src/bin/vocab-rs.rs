use vocab_js::VocabStore;

fn main() {
    let vocab_store = VocabStore::init("vocab.sqlite")
        .expect("Could not init store");
}
