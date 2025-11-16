//! Plugin Registry - Manages all plugins in the system

use crate::plugin::*;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Plugin registry that manages all plugins
pub struct PluginRegistry {
    analyzers: HashMap<String, Arc<dyn AnalyzerPlugin>>,
    reporters: HashMap<String, Arc<dyn ReporterPlugin>>,
    executors: HashMap<String, Arc<dyn ExecutorPlugin>>,
    transformers: HashMap<String, Arc<dyn TransformerPlugin>>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            analyzers: HashMap::new(),
            reporters: HashMap::new(),
            executors: HashMap::new(),
            transformers: HashMap::new(),
        }
    }

    /// Register an analyzer plugin
    pub fn register_analyzer(&mut self, plugin: Arc<dyn AnalyzerPlugin>) -> Result<()> {
        let name = plugin.name();
        if self.analyzers.contains_key(&name) {
            return Err(anyhow!("Analyzer plugin '{}' is already registered", name));
        }
        self.analyzers.insert(name, plugin);
        Ok(())
    }

    /// Register a reporter plugin
    pub fn register_reporter(&mut self, plugin: Arc<dyn ReporterPlugin>) -> Result<()> {
        let name = plugin.name();
        if self.reporters.contains_key(&name) {
            return Err(anyhow!("Reporter plugin '{}' is already registered", name));
        }
        self.reporters.insert(name, plugin);
        Ok(())
    }

    /// Register an executor plugin
    pub fn register_executor(&mut self, plugin: Arc<dyn ExecutorPlugin>) -> Result<()> {
        let name = plugin.name();
        if self.executors.contains_key(&name) {
            return Err(anyhow!("Executor plugin '{}' is already registered", name));
        }
        self.executors.insert(name, plugin);
        Ok(())
    }

    /// Register a transformer plugin
    pub fn register_transformer(&mut self, plugin: Arc<dyn TransformerPlugin>) -> Result<()> {
        let name = plugin.name();
        if self.transformers.contains_key(&name) {
            return Err(anyhow!(
                "Transformer plugin '{}' is already registered",
                name
            ));
        }
        self.transformers.insert(name, plugin);
        Ok(())
    }

    /// Get an analyzer plugin by name
    pub fn get_analyzer(&self, name: &str) -> Option<Arc<dyn AnalyzerPlugin>> {
        self.analyzers.get(name).cloned()
    }

    /// Get a reporter plugin by name
    pub fn get_reporter(&self, name: &str) -> Option<Arc<dyn ReporterPlugin>> {
        self.reporters.get(name).cloned()
    }

    /// Get an executor plugin by name
    pub fn get_executor(&self, name: &str) -> Option<Arc<dyn ExecutorPlugin>> {
        self.executors.get(name).cloned()
    }

    /// Get a transformer plugin by name
    pub fn get_transformer(&self, name: &str) -> Option<Arc<dyn TransformerPlugin>> {
        self.transformers.get(name).cloned()
    }

    /// Find an analyzer that can handle the input
    pub fn find_analyzer(&self, input: &AnalyzerInput) -> Option<Arc<dyn AnalyzerPlugin>> {
        self.analyzers
            .values()
            .find(|p| p.is_enabled() && p.can_handle(input))
            .cloned()
    }

    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<(PluginType, PluginMetadata)> {
        let mut plugins = Vec::new();

        for plugin in self.analyzers.values() {
            plugins.push((PluginType::Analyzer, plugin.metadata()));
        }
        for plugin in self.reporters.values() {
            plugins.push((PluginType::Reporter, plugin.metadata()));
        }
        for plugin in self.executors.values() {
            plugins.push((PluginType::Executor, plugin.metadata()));
        }
        for plugin in self.transformers.values() {
            plugins.push((PluginType::Transformer, plugin.metadata()));
        }

        plugins
    }

    /// Get all enabled analyzers
    pub fn enabled_analyzers(&self) -> Vec<Arc<dyn AnalyzerPlugin>> {
        self.analyzers
            .values()
            .filter(|p| p.is_enabled())
            .cloned()
            .collect()
    }

    /// Get all enabled reporters
    pub fn enabled_reporters(&self) -> Vec<Arc<dyn ReporterPlugin>> {
        self.reporters
            .values()
            .filter(|p| p.is_enabled())
            .cloned()
            .collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global plugin registry singleton
static GLOBAL_REGISTRY: once_cell::sync::Lazy<RwLock<PluginRegistry>> =
    once_cell::sync::Lazy::new(|| RwLock::new(PluginRegistry::new()));

/// Get the global plugin registry
pub fn global_registry() -> &'static RwLock<PluginRegistry> {
    &GLOBAL_REGISTRY
}

/// Register a plugin in the global registry
pub fn register_analyzer_plugin(plugin: Arc<dyn AnalyzerPlugin>) -> Result<()> {
    global_registry().write().unwrap().register_analyzer(plugin)
}

pub fn register_reporter_plugin(plugin: Arc<dyn ReporterPlugin>) -> Result<()> {
    global_registry().write().unwrap().register_reporter(plugin)
}

pub fn register_executor_plugin(plugin: Arc<dyn ExecutorPlugin>) -> Result<()> {
    global_registry().write().unwrap().register_executor(plugin)
}

pub fn register_transformer_plugin(plugin: Arc<dyn TransformerPlugin>) -> Result<()> {
    global_registry()
        .write()
        .unwrap()
        .register_transformer(plugin)
}
