use std::path::PathBuf;

use super::PluginConfig;
use crate::types::Dependency;
use crate::types::JSONObject;
use crate::types::Priority;

// TODO Diagnostics and invalidations

pub struct ResolveContext {
  pub specifier: String,
  pub dependency: Dependency,
  pub pipeline: Option<String>,
}

#[derive(Debug, Default)]
pub struct Resolution {
  /// Whether this dependency can be deferred by Parcel itself
  pub can_defer: bool,

  /// The code of the resolved asset
  ///
  /// If provided, this is used rather than reading the file from disk.
  ///
  pub code: Option<String>,

  /// An absolute path to the resolved file
  pub file_path: PathBuf,

  /// Whether the resolved file should be excluded from the build
  pub is_excluded: bool,

  /// Is spread (shallowly merged) onto the request's dependency.meta
  pub meta: JSONObject,

  /// An optional named pipeline to use to compile the resolved file
  pub pipeline: Option<String>,

  /// Overrides the priority set on the dependency
  pub priority: Option<Priority>,

  /// Corresponds to the asset side effects
  pub side_effects: bool,

  /// Query parameters to be used by transformers when compiling the resolved file
  pub query: Option<String>,
}

/// Converts a dependency specifier into a file path that will be processed by transformers
///
/// Resolvers run in a pipeline until one of them return a result.
///
pub trait ResolverPlugin: Send + Sync {
  /// A hook designed to setup any config needed to resolve dependencies
  ///
  /// This function will run once, shortly after the plugin is initialised.
  ///
  fn load_config(&mut self, config: &PluginConfig) -> Result<(), anyhow::Error>;

  /// Determines what the dependency specifier resolves to
  fn resolve(
    &mut self,
    config: &PluginConfig,
    ctx: &ResolveContext,
  ) -> Result<Resolution, anyhow::Error>;
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestResolverPlugin {}

  impl ResolverPlugin for TestResolverPlugin {
    fn load_config(&mut self, _config: &PluginConfig) -> Result<(), anyhow::Error> {
      todo!()
    }

    fn resolve(
      &mut self,
      _config: &PluginConfig,
      _ctx: &ResolveContext,
    ) -> Result<Resolution, anyhow::Error> {
      todo!()
    }
  }

  #[test]
  fn can_be_defined_in_dyn_vec() {
    let mut resolvers = Vec::<Box<dyn ResolverPlugin>>::new();

    resolvers.push(Box::new(TestResolverPlugin {}));

    assert_eq!(resolvers.len(), 1);
  }
}
