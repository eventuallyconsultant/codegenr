use crate::{
  custom::CustomError, helpers::HelpersError, loaders::LoaderError, processor::ProcessorError, render::RenderError,
  resolver::ResolverError, SaverError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenrError {
  #[error("Error while loading json document: {0}")]
  Loading(#[from] LoaderError),
  #[error("Error while resolving references: `{0}`")]
  Resolving(#[from] ResolverError),
  #[error("Error while saving intermediate files: `{0}`")]
  Saving(#[from] SaverError),
  #[error("Error while using helpers: `{0}`")]
  Helpers(#[from] HelpersError),
  #[error("Error while adding custom helpers: `{0}`")]
  Customizing(#[from] CustomError),
  #[error("Error while rendering: `{0}`")]
  Rendering(#[from] RenderError),
  #[error("Error while post processing output: `{0}`")]
  Processing(#[from] ProcessorError),
  #[error("TemplateRender error: `{0}`.")]
  RenderTemp(#[from] handlebars::RenderError),
}
