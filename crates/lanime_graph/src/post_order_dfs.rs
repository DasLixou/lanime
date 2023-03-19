use hashbrown::HashSet;
use slotmap::Key;

use crate::{EdgeIdx, Graph, NodeIdx};

/// A special kind of DFS which starts at a point and goes backwards using incoming nodes.
/// Returns the edge data with the source and destination NodeIdx.
/// **Skipps the deepest (zero incoming) nodes**
///
/// # Logic representation
///
/// Imagine this graph structure:
/// ```ignore
/// // a -1-> b -2-> c
/// ```
/// This will result in those results when using `c` as the startnode:
/// ```ìgnore
/// (a, 1, b)
/// (b, 2, c)
/// ```
pub struct PostOrderDFS {
    stack: Vec<(NodeIdx, EdgeIdx, NodeIdx)>,
    discovered: HashSet<NodeIdx>,
    finished: HashSet<NodeIdx>,
}

impl PostOrderDFS {
    pub fn new<N, E>(graph: &Graph<N, E>, start: NodeIdx) -> Self {
        let mut stack = Vec::with_capacity(graph.node_len());
        stack.push((start, EdgeIdx::null(), NodeIdx::null()));
        PostOrderDFS {
            stack,
            discovered: HashSet::with_capacity(graph.node_len()),
            finished: HashSet::with_capacity(graph.node_len()),
        }
    }

    #[inline]
    pub fn next<N, E>(&mut self, graph: &Graph<N, E>) -> Option<(NodeIdx, EdgeIdx, NodeIdx)> {
        while let Some((snx, ex, dnx)) = self.stack.last() {
            let (snx, ex, dnx) = (*snx, *ex, *dnx);
            if self.discovered.insert(snx) {
                for (incoming_node, incoming_edge) in graph.incoming_neighbors(snx) {
                    if !self.discovered.contains(&incoming_node) {
                        self.stack.push((incoming_node, incoming_edge, snx));
                    }
                }
            } else {
                self.stack.pop();
                if self.finished.insert(snx) && !ex.is_null() {
                    return Some((snx, ex, dnx));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::Graph;

    use super::PostOrderDFS;

    #[test]
    fn basic_result_test() {
        let mut graph = Graph::<(), ()>::new();

        let a = graph.add_node(());
        let b = graph.add_node(());
        let c = graph.add_node(());

        let one = graph.add_edge(a, b, ());
        let two = graph.add_edge(b, c, ());

        let mut dfs = PostOrderDFS::new(&graph, c);

        assert_eq!(dfs.next(&graph), Some((a, one, b)));
        assert_eq!(dfs.next(&graph), Some((b, two, c)));
        assert_eq!(dfs.next(&graph), None);
    }
}
