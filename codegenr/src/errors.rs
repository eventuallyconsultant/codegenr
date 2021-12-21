use crate::{loader::LoaderError, resolver::ResolverError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenrError {
  #[error("Error while loading json document: {0}")]
  Loading(#[from] LoaderError),
  #[error("Error while resolving ...")]
  Resolving(#[from] ResolverError),
  // #[error("Error while saving ...")]
  // Saving(#[from] SaverError),
  // #[error("Error while processing ...")]
  // Helpers(#[from] CustomError),
  // #[error("Error while processing ...")]
  // Customizing(#[from] CustomError),
  // #[error("Error while rendering ...")]
  // Rendering(#[from] RenderError),
  // #[error("Error while processing ...")]
  // Processing(#[from] ProcessorError),
}
