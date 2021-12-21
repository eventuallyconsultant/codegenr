use crate::{
  custom::CustomError, helpers::HelpersError, loader::LoaderError, processor::ProcessorError, render::RenderError, resolver::ResolverError,
  SaverError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenrError {
  #[error("Error while loading json document: {0}")]
  Loading(#[from] LoaderError),
  #[error("Error while resolving ...")]
  Resolving(#[from] ResolverError),
  #[error("Error while saving ...")]
  Saving(#[from] SaverError),
  #[error("Error while using helpers ...")]
  Helpers(#[from] HelpersError),
  #[error("Error while customizing ...")]
  Customizing(#[from] CustomError),
  #[error("Error while rendering ...")]
  Rendering(#[from] RenderError),
  #[error("Error while processing ...")]
  Processing(#[from] ProcessorError),
}
