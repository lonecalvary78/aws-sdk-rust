// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdatePipelineOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated pipeline.</p>
    pub pipeline_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the pipeline version.</p>
    pub pipeline_version_id: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl UpdatePipelineOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated pipeline.</p>
    pub fn pipeline_arn(&self) -> ::std::option::Option<&str> {
        self.pipeline_arn.as_deref()
    }
    /// <p>The ID of the pipeline version.</p>
    pub fn pipeline_version_id(&self) -> ::std::option::Option<i64> {
        self.pipeline_version_id
    }
}
impl ::aws_types::request_id::RequestId for UpdatePipelineOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdatePipelineOutput {
    /// Creates a new builder-style object to manufacture [`UpdatePipelineOutput`](crate::operation::update_pipeline::UpdatePipelineOutput).
    pub fn builder() -> crate::operation::update_pipeline::builders::UpdatePipelineOutputBuilder {
        crate::operation::update_pipeline::builders::UpdatePipelineOutputBuilder::default()
    }
}

/// A builder for [`UpdatePipelineOutput`](crate::operation::update_pipeline::UpdatePipelineOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdatePipelineOutputBuilder {
    pub(crate) pipeline_arn: ::std::option::Option<::std::string::String>,
    pub(crate) pipeline_version_id: ::std::option::Option<i64>,
    _request_id: Option<String>,
}
impl UpdatePipelineOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the updated pipeline.</p>
    pub fn pipeline_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pipeline_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the updated pipeline.</p>
    pub fn set_pipeline_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pipeline_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the updated pipeline.</p>
    pub fn get_pipeline_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.pipeline_arn
    }
    /// <p>The ID of the pipeline version.</p>
    pub fn pipeline_version_id(mut self, input: i64) -> Self {
        self.pipeline_version_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ID of the pipeline version.</p>
    pub fn set_pipeline_version_id(mut self, input: ::std::option::Option<i64>) -> Self {
        self.pipeline_version_id = input;
        self
    }
    /// <p>The ID of the pipeline version.</p>
    pub fn get_pipeline_version_id(&self) -> &::std::option::Option<i64> {
        &self.pipeline_version_id
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdatePipelineOutput`](crate::operation::update_pipeline::UpdatePipelineOutput).
    pub fn build(self) -> crate::operation::update_pipeline::UpdatePipelineOutput {
        crate::operation::update_pipeline::UpdatePipelineOutput {
            pipeline_arn: self.pipeline_arn,
            pipeline_version_id: self.pipeline_version_id,
            _request_id: self._request_id,
        }
    }
}
