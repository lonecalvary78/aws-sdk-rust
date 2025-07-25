// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RetrieveMemoryRecordsInput {
    /// <p>The identifier of the memory store from which to retrieve memory records.</p>
    pub memory_id: ::std::option::Option<::std::string::String>,
    /// <p>The namespace to filter memory records by. If specified, only memory records in this namespace are searched.</p>
    pub namespace: ::std::option::Option<::std::string::String>,
    /// <p>The search criteria to use for finding relevant memory records. This includes the search query, memory strategy ID, and other search parameters.</p>
    pub search_criteria: ::std::option::Option<crate::types::SearchCriteria>,
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return in a single call. Minimum value of 1, maximum value of 100. Default is 20.</p>
    pub max_results: ::std::option::Option<i32>,
}
impl RetrieveMemoryRecordsInput {
    /// <p>The identifier of the memory store from which to retrieve memory records.</p>
    pub fn memory_id(&self) -> ::std::option::Option<&str> {
        self.memory_id.as_deref()
    }
    /// <p>The namespace to filter memory records by. If specified, only memory records in this namespace are searched.</p>
    pub fn namespace(&self) -> ::std::option::Option<&str> {
        self.namespace.as_deref()
    }
    /// <p>The search criteria to use for finding relevant memory records. This includes the search query, memory strategy ID, and other search parameters.</p>
    pub fn search_criteria(&self) -> ::std::option::Option<&crate::types::SearchCriteria> {
        self.search_criteria.as_ref()
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return in a single call. Minimum value of 1, maximum value of 100. Default is 20.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl RetrieveMemoryRecordsInput {
    /// Creates a new builder-style object to manufacture [`RetrieveMemoryRecordsInput`](crate::operation::retrieve_memory_records::RetrieveMemoryRecordsInput).
    pub fn builder() -> crate::operation::retrieve_memory_records::builders::RetrieveMemoryRecordsInputBuilder {
        crate::operation::retrieve_memory_records::builders::RetrieveMemoryRecordsInputBuilder::default()
    }
}

/// A builder for [`RetrieveMemoryRecordsInput`](crate::operation::retrieve_memory_records::RetrieveMemoryRecordsInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RetrieveMemoryRecordsInputBuilder {
    pub(crate) memory_id: ::std::option::Option<::std::string::String>,
    pub(crate) namespace: ::std::option::Option<::std::string::String>,
    pub(crate) search_criteria: ::std::option::Option<crate::types::SearchCriteria>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl RetrieveMemoryRecordsInputBuilder {
    /// <p>The identifier of the memory store from which to retrieve memory records.</p>
    /// This field is required.
    pub fn memory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.memory_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the memory store from which to retrieve memory records.</p>
    pub fn set_memory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.memory_id = input;
        self
    }
    /// <p>The identifier of the memory store from which to retrieve memory records.</p>
    pub fn get_memory_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.memory_id
    }
    /// <p>The namespace to filter memory records by. If specified, only memory records in this namespace are searched.</p>
    /// This field is required.
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.namespace = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The namespace to filter memory records by. If specified, only memory records in this namespace are searched.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.namespace = input;
        self
    }
    /// <p>The namespace to filter memory records by. If specified, only memory records in this namespace are searched.</p>
    pub fn get_namespace(&self) -> &::std::option::Option<::std::string::String> {
        &self.namespace
    }
    /// <p>The search criteria to use for finding relevant memory records. This includes the search query, memory strategy ID, and other search parameters.</p>
    /// This field is required.
    pub fn search_criteria(mut self, input: crate::types::SearchCriteria) -> Self {
        self.search_criteria = ::std::option::Option::Some(input);
        self
    }
    /// <p>The search criteria to use for finding relevant memory records. This includes the search query, memory strategy ID, and other search parameters.</p>
    pub fn set_search_criteria(mut self, input: ::std::option::Option<crate::types::SearchCriteria>) -> Self {
        self.search_criteria = input;
        self
    }
    /// <p>The search criteria to use for finding relevant memory records. This includes the search query, memory strategy ID, and other search parameters.</p>
    pub fn get_search_criteria(&self) -> &::std::option::Option<crate::types::SearchCriteria> {
        &self.search_criteria
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The maximum number of results to return in a single call. Minimum value of 1, maximum value of 100. Default is 20.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. Minimum value of 1, maximum value of 100. Default is 20.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of results to return in a single call. Minimum value of 1, maximum value of 100. Default is 20.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// Consumes the builder and constructs a [`RetrieveMemoryRecordsInput`](crate::operation::retrieve_memory_records::RetrieveMemoryRecordsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::retrieve_memory_records::RetrieveMemoryRecordsInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::retrieve_memory_records::RetrieveMemoryRecordsInput {
            memory_id: self.memory_id,
            namespace: self.namespace,
            search_criteria: self.search_criteria,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
