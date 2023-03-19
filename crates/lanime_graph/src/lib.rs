pub mod post_order_dfs;

use std::ops::{Index, IndexMut};

use hashbrown::HashMap;
use slotmap::{new_key_type, HopSlotMap, SecondaryMap};

new_key_type! {
    pub struct NodeIdx;
    pub struct EdgeIdx;
}

/// Simple Directed Graph
pub struct Graph<N, E> {
    nodes: HopSlotMap<NodeIdx, N>,
    edges: HopSlotMap<EdgeIdx, E>,
    incoming_adjacencies: SecondaryMap<NodeIdx, HashMap<NodeIdx, EdgeIdx>>,
    outgoing_adjacencies: SecondaryMap<NodeIdx, HashMap<NodeIdx, EdgeIdx>>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: HopSlotMap::with_key(),
            edges: HopSlotMap::with_key(),
            incoming_adjacencies: SecondaryMap::new(),
            outgoing_adjacencies: SecondaryMap::new(),
        }
    }

    #[inline]
    #[rustfmt::skip]
    pub fn node_len(&self) -> usize { self.nodes.len() }

    pub fn add_node(&mut self, value: N) -> NodeIdx {
        let idx = self.nodes.insert(value);
        self.incoming_adjacencies.insert(idx, HashMap::new());
        self.outgoing_adjacencies.insert(idx, HashMap::new());
        idx
    }

    pub fn add_edge(&mut self, src: NodeIdx, dst: NodeIdx, value: E) -> EdgeIdx {
        let idx = self.edges.insert(value);
        self.outgoing_adjacencies[src].insert(dst, idx);
        self.incoming_adjacencies[dst].insert(src, idx);
        idx
    }

    pub fn incoming_neighbors(
        &self,
        dst: NodeIdx,
    ) -> impl Iterator<Item = (NodeIdx, EdgeIdx)> + '_ {
        self.incoming_adjacencies[dst]
            .iter()
            .map(|(node, edge)| (*node, *edge))
    }

    #[inline]
    pub fn get_disjoint_mut<const NI: usize, const EI: usize>(
        &mut self,
        nodes: [NodeIdx; NI],
        edges: [EdgeIdx; EI],
    ) -> (Option<[&mut N; NI]>, Option<[&mut E; EI]>) {
        (
            self.nodes.get_disjoint_mut(nodes),
            self.edges.get_disjoint_mut(edges),
        )
    }
}

impl<N, E> Index<NodeIdx> for Graph<N, E> {
    type Output = N;

    fn index(&self, index: NodeIdx) -> &Self::Output {
        self.nodes.get(index).unwrap()
    }
}

impl<N, E> IndexMut<NodeIdx> for Graph<N, E> {
    fn index_mut(&mut self, index: NodeIdx) -> &mut Self::Output {
        self.nodes.get_mut(index).unwrap()
    }
}

impl<N, E> Index<EdgeIdx> for Graph<N, E> {
    type Output = E;

    fn index(&self, index: EdgeIdx) -> &Self::Output {
        self.edges.get(index).unwrap()
    }
}

impl<N, E> IndexMut<EdgeIdx> for Graph<N, E> {
    fn index_mut(&mut self, index: EdgeIdx) -> &mut Self::Output {
        self.edges.get_mut(index).unwrap()
    }
}
