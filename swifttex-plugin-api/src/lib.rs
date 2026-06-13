pub mod ast;
pub use ast::Node;
use std::collections::HashMap;

/// A single command expansion registered by a plugin.
/// When the parser encounters \{name}, it calls expand()
/// with the raw argument tokens and returns a Node.
pub trait CommandHandler: Send + Sync {
    fn name(&self) -> &str;
    fn arity(&self) -> usize;  // number of {} arguments expected
    fn expand(&self, args: Vec<Vec<Node>>) -> Node;
}

/// A symbol mapping registered by a plugin.
/// When parser encounters \{name} with arity 0,
/// it resolves to this char.
pub trait SymbolHandler: Send + Sync {
    fn name(&self) -> &str;
    fn resolve(&self) -> char;
}

/// An environment handler registered by a plugin.
/// When parser encounters \begin{name}...\end{name},
/// it calls render() with the inner nodes.
pub trait EnvironmentHandler: Send + Sync {
    fn name(&self) -> &str;
    fn render(&self, inner: Vec<Node>) -> Node;
}

/// The plugin interface. Implement this to create a plugin.
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str { "0.1.0" }

    fn commands(&self) -> Vec<Box<dyn CommandHandler>> { vec![] }
    fn symbols(&self) -> Vec<Box<dyn SymbolHandler>> { vec![] }
    fn environments(&self) -> Vec<Box<dyn EnvironmentHandler>> { vec![] }
}

/// Registry that holds all registered plugins and their handlers.
pub struct PluginRegistry {
    commands: HashMap<String, Box<dyn CommandHandler>>,
    symbols: HashMap<String, Box<dyn SymbolHandler>>,
    environments: HashMap<String, Box<dyn EnvironmentHandler>>,
    plugin_names: Vec<String>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            symbols: HashMap::new(),
            environments: HashMap::new(),
            plugin_names: vec![],
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        for cmd in plugin.commands() {
            self.commands.insert(cmd.name().to_string(), cmd);
        }
        for sym in plugin.symbols() {
            self.symbols.insert(sym.name().to_string(), sym);
        }
        for env in plugin.environments() {
            self.environments.insert(env.name().to_string(), env);
        }
        self.plugin_names.push(plugin.name().to_string());
    }

    pub fn resolve_command(&self, name: &str, args: Vec<Vec<Node>>) -> Option<Node> {
        self.commands.get(name).map(|h| h.expand(args))
    }

    pub fn resolve_symbol(&self, name: &str) -> Option<char> {
        self.symbols.get(name).map(|h| h.resolve())
    }

    pub fn resolve_environment(&self, name: &str, inner: Vec<Node>) -> Option<Node> {
        self.environments.get(name).map(|h| h.render(inner))
    }

    pub fn has_command(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    pub fn has_symbol(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }
    
    pub fn command_arity(&self, name: &str) -> Option<usize> {
        self.commands.get(name).map(|h| h.arity())
    }

    pub fn has_environment(&self, name: &str) -> bool {
        self.environments.contains_key(name)
    }

    pub fn registered_plugins(&self) -> &[String] {
        &self.plugin_names
    }

    pub fn register_symbol_direct(&mut self, name: String, ch: char) {
        struct DirectSymbol { name: String, ch: char }
        impl SymbolHandler for DirectSymbol {
            fn name(&self) -> &str { &self.name }
            fn resolve(&self) -> char { self.ch }
        }
        self.symbols.insert(name.clone(), Box::new(DirectSymbol { name, ch }));
    }

    pub fn register_command_direct(
        &mut self,
        name: String,
        arity: usize,
        expand: Box<dyn Fn(Vec<Vec<Node>>) -> Node + Send + Sync>,
    ) {
        struct DirectCommand {
            name: String,
            arity: usize,
            expand: Box<dyn Fn(Vec<Vec<Node>>) -> Node + Send + Sync>,
        }
        impl CommandHandler for DirectCommand {
            fn name(&self) -> &str { &self.name }
            fn arity(&self) -> usize { self.arity }
            fn expand(&self, args: Vec<Vec<Node>>) -> Node { (self.expand)(args) }
        }
        self.commands.insert(name.clone(),
            Box::new(DirectCommand { name, arity, expand }));
    }
}

impl Default for PluginRegistry {
    fn default() -> Self { Self::new() }
}
