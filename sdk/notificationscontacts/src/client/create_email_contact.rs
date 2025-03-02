// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateEmailContact`](crate::operation::create_email_contact::builders::CreateEmailContactFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_email_contact::builders::CreateEmailContactFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_email_contact::builders::CreateEmailContactFluentBuilder::set_name):<br>required: **true**<br><p>The name of the email contact.</p><br>
    ///   - [`email_address(impl Into<String>)`](crate::operation::create_email_contact::builders::CreateEmailContactFluentBuilder::email_address) / [`set_email_address(Option<String>)`](crate::operation::create_email_contact::builders::CreateEmailContactFluentBuilder::set_email_address):<br>required: **true**<br><p>The email address this email contact points to. The activation email and any subscribed emails are sent here.</p><note>  <p>This email address can't receive emails until it's activated.</p> </note><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_email_contact::builders::CreateEmailContactFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_email_contact::builders::CreateEmailContactFluentBuilder::set_tags):<br>required: **false**<br><p>A map of tags assigned to a resource. A tag is a string-to-string map of key-value pairs.</p><br>
    /// - On success, responds with [`CreateEmailContactOutput`](crate::operation::create_email_contact::CreateEmailContactOutput) with field(s):
    ///   - [`arn(String)`](crate::operation::create_email_contact::CreateEmailContactOutput::arn): <p>The Amazon Resource Name (ARN) of the resource.</p>
    /// - On failure, responds with [`SdkError<CreateEmailContactError>`](crate::operation::create_email_contact::CreateEmailContactError)
    pub fn create_email_contact(&self) -> crate::operation::create_email_contact::builders::CreateEmailContactFluentBuilder {
        crate::operation::create_email_contact::builders::CreateEmailContactFluentBuilder::new(self.handle.clone())
    }
}
