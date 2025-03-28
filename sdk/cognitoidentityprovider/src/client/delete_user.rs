// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteUser`](crate::operation::delete_user::builders::DeleteUserFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`access_token(impl Into<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::access_token) / [`set_access_token(Option<String>)`](crate::operation::delete_user::builders::DeleteUserFluentBuilder::set_access_token):<br>required: **true**<br><p>A valid access token that Amazon Cognito issued to the currently signed-in user. Must include a scope claim for <code>aws.cognito.signin.user.admin</code>.</p><br>
    /// - On success, responds with [`DeleteUserOutput`](crate::operation::delete_user::DeleteUserOutput)
    /// - On failure, responds with [`SdkError<DeleteUserError>`](crate::operation::delete_user::DeleteUserError)
    pub fn delete_user(&self) -> crate::operation::delete_user::builders::DeleteUserFluentBuilder {
        crate::operation::delete_user::builders::DeleteUserFluentBuilder::new(self.handle.clone())
    }
}
