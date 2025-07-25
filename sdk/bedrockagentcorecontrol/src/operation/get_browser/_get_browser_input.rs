// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBrowserInput {
    /// <p>The unique identifier of the browser to retrieve.</p>
    pub browser_id: ::std::option::Option<::std::string::String>,
}
impl GetBrowserInput {
    /// <p>The unique identifier of the browser to retrieve.</p>
    pub fn browser_id(&self) -> ::std::option::Option<&str> {
        self.browser_id.as_deref()
    }
}
impl GetBrowserInput {
    /// Creates a new builder-style object to manufacture [`GetBrowserInput`](crate::operation::get_browser::GetBrowserInput).
    pub fn builder() -> crate::operation::get_browser::builders::GetBrowserInputBuilder {
        crate::operation::get_browser::builders::GetBrowserInputBuilder::default()
    }
}

/// A builder for [`GetBrowserInput`](crate::operation::get_browser::GetBrowserInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetBrowserInputBuilder {
    pub(crate) browser_id: ::std::option::Option<::std::string::String>,
}
impl GetBrowserInputBuilder {
    /// <p>The unique identifier of the browser to retrieve.</p>
    /// This field is required.
    pub fn browser_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.browser_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the browser to retrieve.</p>
    pub fn set_browser_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.browser_id = input;
        self
    }
    /// <p>The unique identifier of the browser to retrieve.</p>
    pub fn get_browser_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.browser_id
    }
    /// Consumes the builder and constructs a [`GetBrowserInput`](crate::operation::get_browser::GetBrowserInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_browser::GetBrowserInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_browser::GetBrowserInput { browser_id: self.browser_id })
    }
}
