// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateImage`](crate::operation::create_image::builders::CreateImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_description):<br>required: **false**<br><p>The description of the image.</p><br>
    ///   - [`display_name(impl Into<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::display_name) / [`set_display_name(Option<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_display_name):<br>required: **false**<br><p>The display name of the image. If not provided, <code>ImageName</code> is displayed.</p><br>
    ///   - [`image_name(impl Into<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::image_name) / [`set_image_name(Option<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_image_name):<br>required: **true**<br><p>The name of the image. Must be unique to your account.</p><br>
    ///   - [`role_arn(impl Into<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_role_arn):<br>required: **true**<br><p>The ARN of an IAM role that enables Amazon SageMaker AI to perform tasks on your behalf.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_image::builders::CreateImageFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_tags):<br>required: **false**<br><p>A list of tags to apply to the image.</p><br>
    /// - On success, responds with [`CreateImageOutput`](crate::operation::create_image::CreateImageOutput) with field(s):
    ///   - [`image_arn(Option<String>)`](crate::operation::create_image::CreateImageOutput::image_arn): <p>The ARN of the image.</p>
    /// - On failure, responds with [`SdkError<CreateImageError>`](crate::operation::create_image::CreateImageError)
    pub fn create_image(&self) -> crate::operation::create_image::builders::CreateImageFluentBuilder {
        crate::operation::create_image::builders::CreateImageFluentBuilder::new(self.handle.clone())
    }
}
