use swifttex_plugin_api::{
    Plugin, CommandHandler, SymbolHandler, Node
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn name(&self) -> &str { "physics" }
    fn version(&self) -> &str { "0.1.0" }

    fn symbols(&self) -> Vec<Box<dyn SymbolHandler>> {
        vec![
            Box::new(HbarSymbol),
        ]
    }

    fn commands(&self) -> Vec<Box<dyn CommandHandler>> {
        vec![
            Box::new(BraCommand),
            Box::new(KetCommand),
            Box::new(BraketCommand),
        ]
    }
}

struct HbarSymbol;
impl SymbolHandler for HbarSymbol {
    fn name(&self) -> &str { "hbar" }
    fn resolve(&self) -> char { 'ℏ' }
}

struct BraCommand;
impl CommandHandler for BraCommand {
    fn name(&self) -> &str { "bra" }
    fn arity(&self) -> usize { 1 }
    fn expand(&self, mut args: Vec<Vec<Node>>) -> Node {
        let inner = args.into_iter().next().unwrap_or_default();
        Node::Group(vec![
            Node::Symbol('⟨'),
            Node::Group(inner),
            Node::Symbol('|'),
        ])
    }
}

struct KetCommand;
impl CommandHandler for KetCommand {
    fn name(&self) -> &str { "ket" }
    fn arity(&self) -> usize { 1 }
    fn expand(&self, mut args: Vec<Vec<Node>>) -> Node {
        let inner = args.into_iter().next().unwrap_or_default();
        Node::Group(vec![
            Node::Symbol('|'),
            Node::Group(inner),
            Node::Symbol('⟩'),
        ])
    }
}

struct BraketCommand;
impl CommandHandler for BraketCommand {
    fn name(&self) -> &str { "braket" }
    fn arity(&self) -> usize { 2 }
    fn expand(&self, mut args: Vec<Vec<Node>>) -> Node {
        let bra = if !args.is_empty() { args.remove(0) } else { vec![] };
        let ket = if !args.is_empty() { args.remove(0) } else { vec![] };
        Node::Group(vec![
            Node::Symbol('⟨'),
            Node::Group(bra),
            Node::Symbol('|'),
            Node::Group(ket),
            Node::Symbol('⟩'),
        ])
    }
}

fn main() {
    println!("Physics plugin example implemented.");
}
