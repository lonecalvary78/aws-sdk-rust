// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InvokeCodeInterpreterInput {
    /// <p>The unique identifier of the code interpreter associated with the session. This must match the identifier used when creating the session with <code>StartCodeInterpreterSession</code>.</p>
    pub code_interpreter_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the code interpreter session to use. This must be an active session created with <code>StartCodeInterpreterSession</code>. If the session has expired or been stopped, the request will fail.</p>
    pub session_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the code interpreter to invoke.</p>
    pub name: ::std::option::Option<crate::types::ToolName>,
    /// <p>The arguments for the code interpreter. This includes the code to execute and any additional parameters such as the programming language, whether to clear the execution context, and other execution options. The structure of this parameter depends on the specific code interpreter being used.</p>
    pub arguments: ::std::option::Option<crate::types::ToolArguments>,
}
impl InvokeCodeInterpreterInput {
    /// <p>The unique identifier of the code interpreter associated with the session. This must match the identifier used when creating the session with <code>StartCodeInterpreterSession</code>.</p>
    pub fn code_interpreter_identifier(&self) -> ::std::option::Option<&str> {
        self.code_interpreter_identifier.as_deref()
    }
    /// <p>The unique identifier of the code interpreter session to use. This must be an active session created with <code>StartCodeInterpreterSession</code>. If the session has expired or been stopped, the request will fail.</p>
    pub fn session_id(&self) -> ::std::option::Option<&str> {
        self.session_id.as_deref()
    }
    /// <p>The name of the code interpreter to invoke.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::ToolName> {
        self.name.as_ref()
    }
    /// <p>The arguments for the code interpreter. This includes the code to execute and any additional parameters such as the programming language, whether to clear the execution context, and other execution options. The structure of this parameter depends on the specific code interpreter being used.</p>
    pub fn arguments(&self) -> ::std::option::Option<&crate::types::ToolArguments> {
        self.arguments.as_ref()
    }
}
impl InvokeCodeInterpreterInput {
    /// Creates a new builder-style object to manufacture [`InvokeCodeInterpreterInput`](crate::operation::invoke_code_interpreter::InvokeCodeInterpreterInput).
    pub fn builder() -> crate::operation::invoke_code_interpreter::builders::InvokeCodeInterpreterInputBuilder {
        crate::operation::invoke_code_interpreter::builders::InvokeCodeInterpreterInputBuilder::default()
    }
}

/// A builder for [`InvokeCodeInterpreterInput`](crate::operation::invoke_code_interpreter::InvokeCodeInterpreterInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InvokeCodeInterpreterInputBuilder {
    pub(crate) code_interpreter_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) session_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<crate::types::ToolName>,
    pub(crate) arguments: ::std::option::Option<crate::types::ToolArguments>,
}
impl InvokeCodeInterpreterInputBuilder {
    /// <p>The unique identifier of the code interpreter associated with the session. This must match the identifier used when creating the session with <code>StartCodeInterpreterSession</code>.</p>
    /// This field is required.
    pub fn code_interpreter_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code_interpreter_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the code interpreter associated with the session. This must match the identifier used when creating the session with <code>StartCodeInterpreterSession</code>.</p>
    pub fn set_code_interpreter_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code_interpreter_identifier = input;
        self
    }
    /// <p>The unique identifier of the code interpreter associated with the session. This must match the identifier used when creating the session with <code>StartCodeInterpreterSession</code>.</p>
    pub fn get_code_interpreter_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.code_interpreter_identifier
    }
    /// <p>The unique identifier of the code interpreter session to use. This must be an active session created with <code>StartCodeInterpreterSession</code>. If the session has expired or been stopped, the request will fail.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.session_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the code interpreter session to use. This must be an active session created with <code>StartCodeInterpreterSession</code>. If the session has expired or been stopped, the request will fail.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.session_id = input;
        self
    }
    /// <p>The unique identifier of the code interpreter session to use. This must be an active session created with <code>StartCodeInterpreterSession</code>. If the session has expired or been stopped, the request will fail.</p>
    pub fn get_session_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.session_id
    }
    /// <p>The name of the code interpreter to invoke.</p>
    /// This field is required.
    pub fn name(mut self, input: crate::types::ToolName) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the code interpreter to invoke.</p>
    pub fn set_name(mut self, input: ::std::option::Option<crate::types::ToolName>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the code interpreter to invoke.</p>
    pub fn get_name(&self) -> &::std::option::Option<crate::types::ToolName> {
        &self.name
    }
    /// <p>The arguments for the code interpreter. This includes the code to execute and any additional parameters such as the programming language, whether to clear the execution context, and other execution options. The structure of this parameter depends on the specific code interpreter being used.</p>
    pub fn arguments(mut self, input: crate::types::ToolArguments) -> Self {
        self.arguments = ::std::option::Option::Some(input);
        self
    }
    /// <p>The arguments for the code interpreter. This includes the code to execute and any additional parameters such as the programming language, whether to clear the execution context, and other execution options. The structure of this parameter depends on the specific code interpreter being used.</p>
    pub fn set_arguments(mut self, input: ::std::option::Option<crate::types::ToolArguments>) -> Self {
        self.arguments = input;
        self
    }
    /// <p>The arguments for the code interpreter. This includes the code to execute and any additional parameters such as the programming language, whether to clear the execution context, and other execution options. The structure of this parameter depends on the specific code interpreter being used.</p>
    pub fn get_arguments(&self) -> &::std::option::Option<crate::types::ToolArguments> {
        &self.arguments
    }
    /// Consumes the builder and constructs a [`InvokeCodeInterpreterInput`](crate::operation::invoke_code_interpreter::InvokeCodeInterpreterInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::invoke_code_interpreter::InvokeCodeInterpreterInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::invoke_code_interpreter::InvokeCodeInterpreterInput {
            code_interpreter_identifier: self.code_interpreter_identifier,
            session_id: self.session_id,
            name: self.name,
            arguments: self.arguments,
        })
    }
}
