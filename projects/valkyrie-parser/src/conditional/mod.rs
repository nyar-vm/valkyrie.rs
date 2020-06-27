use std::ops::Range;

// if a {1}
// if a {1} else {2}
// if a {1} else if b {2}
// if a {1} else if b {2} else {3}
// if a {1} else if b {2} else if c {3}
// if a {1} else if b {2} else if c {3} else {4}
pub struct IfStatementNode<E, B> {
    pub branches: Vec<ConditionBranchNode<E, B>>,
    pub else_branch: Option<B>,
    pub range: Range<usize>,
}

pub struct ConditionBranchNode<E, B> {
    pub condition: E,
    pub body: B,
    pub range: Range<usize>,
}
