// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListStreamsInput {
    /// <p>The name of the keyspace for which to list streams. If specified, only streams associated with tables in this keyspace are returned. If omitted, streams from all keyspaces are included in the results.</p>
    pub keyspace_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the table for which to list streams. Must be used together with <code>keyspaceName</code>. If specified, only streams associated with this specific table are returned.</p>
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of streams to return in a single <code>ListStreams</code> request. Default value is 100. The minimum value is 1 and the maximum value is 100.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>An optional pagination token provided by a previous <code>ListStreams</code> operation. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>maxResults</code>.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListStreamsInput {
    /// <p>The name of the keyspace for which to list streams. If specified, only streams associated with tables in this keyspace are returned. If omitted, streams from all keyspaces are included in the results.</p>
    pub fn keyspace_name(&self) -> ::std::option::Option<&str> {
        self.keyspace_name.as_deref()
    }
    /// <p>The name of the table for which to list streams. Must be used together with <code>keyspaceName</code>. If specified, only streams associated with this specific table are returned.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>The maximum number of streams to return in a single <code>ListStreams</code> request. Default value is 100. The minimum value is 1 and the maximum value is 100.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>An optional pagination token provided by a previous <code>ListStreams</code> operation. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>maxResults</code>.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListStreamsInput {
    /// Creates a new builder-style object to manufacture [`ListStreamsInput`](crate::operation::list_streams::ListStreamsInput).
    pub fn builder() -> crate::operation::list_streams::builders::ListStreamsInputBuilder {
        crate::operation::list_streams::builders::ListStreamsInputBuilder::default()
    }
}

/// A builder for [`ListStreamsInput`](crate::operation::list_streams::ListStreamsInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListStreamsInputBuilder {
    pub(crate) keyspace_name: ::std::option::Option<::std::string::String>,
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListStreamsInputBuilder {
    /// <p>The name of the keyspace for which to list streams. If specified, only streams associated with tables in this keyspace are returned. If omitted, streams from all keyspaces are included in the results.</p>
    pub fn keyspace_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.keyspace_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the keyspace for which to list streams. If specified, only streams associated with tables in this keyspace are returned. If omitted, streams from all keyspaces are included in the results.</p>
    pub fn set_keyspace_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.keyspace_name = input;
        self
    }
    /// <p>The name of the keyspace for which to list streams. If specified, only streams associated with tables in this keyspace are returned. If omitted, streams from all keyspaces are included in the results.</p>
    pub fn get_keyspace_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.keyspace_name
    }
    /// <p>The name of the table for which to list streams. Must be used together with <code>keyspaceName</code>. If specified, only streams associated with this specific table are returned.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the table for which to list streams. Must be used together with <code>keyspaceName</code>. If specified, only streams associated with this specific table are returned.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>The name of the table for which to list streams. Must be used together with <code>keyspaceName</code>. If specified, only streams associated with this specific table are returned.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// <p>The maximum number of streams to return in a single <code>ListStreams</code> request. Default value is 100. The minimum value is 1 and the maximum value is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of streams to return in a single <code>ListStreams</code> request. Default value is 100. The minimum value is 1 and the maximum value is 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of streams to return in a single <code>ListStreams</code> request. Default value is 100. The minimum value is 1 and the maximum value is 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>An optional pagination token provided by a previous <code>ListStreams</code> operation. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>maxResults</code>.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional pagination token provided by a previous <code>ListStreams</code> operation. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>maxResults</code>.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>An optional pagination token provided by a previous <code>ListStreams</code> operation. If this parameter is specified, the response includes only records beyond the token, up to the value specified by <code>maxResults</code>.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`ListStreamsInput`](crate::operation::list_streams::ListStreamsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_streams::ListStreamsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_streams::ListStreamsInput {
            keyspace_name: self.keyspace_name,
            table_name: self.table_name,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
