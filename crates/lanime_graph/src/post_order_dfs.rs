use std::mem;

use slotmap::{Key, SecondaryMap};

use crate::{EdgeIdx, Graph, NodeIdx};

pub enum StartNode {
    Single(NodeIdx),
    SearchAll,
}

#[derive(PartialEq, Clone, Copy)]
enum Tag {
    Unvisited,
    Discovered,
    Finished,
}

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
    nodes: SecondaryMap<NodeIdx, Tag>,
    search: bool,
}

impl PostOrderDFS {
    pub fn new<N, E>(graph: &Graph<N, E>, start: StartNode) -> Self {
        let mut stack = Vec::with_capacity(graph.node_len());
        let search = if let StartNode::Single(node) = start {
            stack.push((node, EdgeIdx::null(), NodeIdx::null()));
            false
        } else {
            true
        };
        let mut nodes = SecondaryMap::with_capacity(graph.node_len());
        for n in graph.nodes.keys() {
            nodes.insert(n, Tag::Unvisited);
        }
        PostOrderDFS {
            stack,
            nodes,
            search,
        }
    }

    #[inline]
    pub fn next<N, E>(&mut self, graph: &Graph<N, E>) -> Option<(NodeIdx, EdgeIdx, NodeIdx)> {
        while let Some((snx, ex, dnx)) = self.stack.last() {
            let (snx, ex, dnx) = (*snx, *ex, *dnx);
            if self.discover(snx) {
                for (incoming_node, incoming_edge) in graph.incoming_neighbors(snx) {
                    if !self.is_discovered(incoming_node) {
                        self.stack.push((incoming_node, incoming_edge, snx));
                    }
                }
            } else {
                self.stack.pop();
                if self.finish(snx) && !ex.is_null() {
                    return Some((snx, ex, dnx));
                }
            }
        }
        if self.search {
            if let Some((node, _)) = self
                .nodes
                .iter()
                .filter(|(_, &tag)| tag == Tag::Unvisited)
                .next()
            {
                self.stack.push((node, EdgeIdx::null(), NodeIdx::null()));
                self.next(graph)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline]
    fn discover(&mut self, idx: NodeIdx) -> bool {
        mem::replace(self.nodes.get_mut(idx).unwrap(), Tag::Discovered) != Tag::Discovered
    }

    #[inline]
    fn is_discovered(&self, idx: NodeIdx) -> bool {
        *self.nodes.get(idx).unwrap() == Tag::Discovered
    }

    #[inline]
    fn finish(&mut self, idx: NodeIdx) -> bool {
        mem::replace(self.nodes.get_mut(idx).unwrap(), Tag::Finished) != Tag::Finished
    }
}

#[cfg(test)]
mod test {
    use crate::{post_order_dfs::StartNode, Graph};

    use super::PostOrderDFS;

    #[test]
    fn basic_result_test() {
        //
        // a -1-> b -2-> c
        //
        let mut graph = Graph::<(), ()>::new();

        let a = graph.add_node(());
        let b = graph.add_node(());
        let c = graph.add_node(());

        let one = graph.add_edge(a, b, ());
        let two = graph.add_edge(b, c, ());

        let mut dfs = PostOrderDFS::new(&graph, StartNode::Single(c));

        assert_eq!(dfs.next(&graph), Some((a, one, b)));
        assert_eq!(dfs.next(&graph), Some((b, two, c)));
        assert_eq!(dfs.next(&graph), None);
    }

    #[test]
    fn search_all() {
        //
        // a -1-> b
        //
        // c -2-> d
        //
        let mut graph = Graph::<(), ()>::new();

        let a = graph.add_node(());
        let b = graph.add_node(());
        let c = graph.add_node(());
        let d = graph.add_node(());

        let one = graph.add_edge(a, b, ());
        let two = graph.add_edge(c, d, ());

        let mut dfs = PostOrderDFS::new(&graph, StartNode::SearchAll);

        assert_eq!(dfs.next(&graph), Some((a, one, b)));
        assert_eq!(dfs.next(&graph), Some((c, two, d)));
        assert_eq!(dfs.next(&graph), None);
    }
}
