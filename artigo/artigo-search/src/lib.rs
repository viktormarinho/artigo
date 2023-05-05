
struct Document {
    path: String
}

struct DocumentReference {
    document: Document,
    ocurrences: usize,
}

struct IndexTerm {
    term: String,
    refs: Vec<DocumentReference>,
}

pub struct QueryResult;

pub enum IngestionType {
    FileSystem(String),
}

pub struct SearchGraph {
    indexes: Vec<IndexTerm>,
    ingestion_type: IngestionType
}

impl SearchGraph {
    pub fn new(ingestion_type: IngestionType) -> Self {
        todo!();
    }

    pub fn text_query(text: String) -> QueryResult {
        todo!();
    }
}