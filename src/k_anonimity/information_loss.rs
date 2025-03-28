use crate::k_anonimity::lattice::Node;

fn prec_loss(node: Node) -> u32 {
    let mut loss: u32 = 0;
    for (qi_name, qi_gen) in node.gen_state.iter() {
        loss += *qi_gen as u32 / node.lattice.rules[qi_name].max_level as u32;
    }
    loss
}
