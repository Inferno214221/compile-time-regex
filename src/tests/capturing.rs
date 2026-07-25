use super::*;

regex! {
    pub CapturingExpr = r"<(?<tag>[a-z\-]+)( (?<attribute>[a-z\-]+))*>" / "i"
}