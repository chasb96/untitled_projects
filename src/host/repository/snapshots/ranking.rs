pub enum Ranking {
    ViewCount,
}

impl Ranking {
    pub fn as_ordering_clause(&self) -> &'static str {
        match self {
            Ranking::ViewCount => "view_count DESC",
        }
    }
}