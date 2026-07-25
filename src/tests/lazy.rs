use super::*;

regex! {
    pub LazyExpr = r"[a-z]+?"
}

regex! {
    pub LazyBoundedExpr = r"[a-z]+?a"
}