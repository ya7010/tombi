use itertools::Itertools;
use syntax::SyntaxNode;
use text_size::TextSize;

use crate::AstNode;

pub fn find_node_at_offset<N: AstNode>(syntax: &SyntaxNode, offset: TextSize) -> Option<N> {
    ancestors_at_offset(syntax, offset).find_map(N::cast)
}

pub fn ancestors_at_offset(
    node: &SyntaxNode,
    offset: TextSize,
) -> impl Iterator<Item = SyntaxNode> {
    node.token_at_offset(offset)
        .map(|token| token.parent_ancestors())
        .kmerge_by(|node1, node2| node1.text_range().len() < node2.text_range().len())
}
