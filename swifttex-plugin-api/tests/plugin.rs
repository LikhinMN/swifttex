use swifttex_plugin_api::{Plugin, CommandHandler, SymbolHandler, Node, PluginRegistry};
use swifttex_parser::Parser;
use std::sync::{Arc, Mutex};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn name(&self) -> &str { "physics" }
    fn version(&self) -> &str { "0.1.0" }

    fn symbols(&self) -> Vec<Box<dyn SymbolHandler>> {
        vec![Box::new(HbarSymbol)]
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

#[test]
fn test_plugin_registration() {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(PhysicsPlugin));
    
    assert!(registry.registered_plugins().contains(&"physics".to_string()));
    assert_eq!(registry.resolve_symbol("hbar"), Some('ℏ'));
}

#[test]
fn test_plugin_command() {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(PhysicsPlugin));
    
    let args = vec![vec![Node::Text('x')]];
    let node = registry.resolve_command("bra", args).unwrap();
    if let Node::Group(inner) = node {
        assert_eq!(inner.len(), 3);
        assert_eq!(inner[0], Node::Symbol('⟨'));
    } else {
        panic!("Expected Group");
    }
}

#[test]
fn test_direct_registration() {
    let mut registry = PluginRegistry::new();
    registry.register_symbol_direct("mypi".to_string(), 'π');
    assert_eq!(registry.resolve_symbol("mypi"), Some('π'));
    assert_eq!(registry.resolve_symbol("notregistered"), None);
}

#[test]
fn test_parser_with_plugin() {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(PhysicsPlugin));
    let reg_arc = Arc::new(Mutex::new(registry));
    
    let mut parser = Parser::with_registry(r"\hbar", Some(reg_arc.clone()));
    let nodes = parser.parse();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0], Node::Symbol('ℏ'));
    
    let mut parser = Parser::with_registry(r"\ket{x}", Some(reg_arc));
    let nodes = parser.parse();
    assert_eq!(nodes.len(), 1);
    if let Node::Group(inner) = &nodes[0] {
        assert_eq!(inner.len(), 3);
        assert_eq!(inner[0], Node::Symbol('|'));
        assert_eq!(inner[2], Node::Symbol('⟩'));
    } else {
        panic!("Expected Group");
    }
}
