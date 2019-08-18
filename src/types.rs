//! This file contains common type definitions used in other parts of the project.
pub(crate) type NumTy = u32;
pub(crate) type NodeIx = petgraph::graph::NodeIndex<NumTy>;
pub(crate) type Graph<V, E> = petgraph::Graph<V, E, petgraph::Directed, NumTy>;
pub(crate) type Result<T> = std::result::Result<T, CompileError>;

#[derive(Debug)]
pub(crate) struct CompileError(pub String);

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
