use crate::block::BlockNode;

pub(crate) fn render_blocks(output: &mut String, blocks: &[BlockNode], depth: usize) {
    for block in blocks {
        block.render(output);
    }

    if depth > 0 {
        output.push('\n');
    }
}
