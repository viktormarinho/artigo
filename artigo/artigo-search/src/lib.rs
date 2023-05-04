

struct Document {
    id: usize,
    path: String
}

struct DocumentReference;

struct IndexTerm {
    term: String,
    refs: Vec<DocumentReference>,
}

pub struct SearchGraph {
    indexes: Vec<IndexTerm>,
}

impl SearchGraph {
    pub fn from_dir(dir: String) -> Self {
        todo!();
    }

    pub fn text_query(text: String) {

    }
}