// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSubscription`](crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`space_name(impl Into<String>)`](crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder::space_name) / [`set_space_name(Option<String>)`](crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder::set_space_name):<br>required: **true**<br><p>The name of the space.</p><br>
    /// - On success, responds with [`GetSubscriptionOutput`](crate::operation::get_subscription::GetSubscriptionOutput) with field(s):
    ///   - [`subscription_type(Option<String>)`](crate::operation::get_subscription::GetSubscriptionOutput::subscription_type): <p>The type of the billing plan for the space.</p>
    ///   - [`aws_account_name(Option<String>)`](crate::operation::get_subscription::GetSubscriptionOutput::aws_account_name): <p>The display name of the Amazon Web Services account used for billing for the space.</p>
    ///   - [`pending_subscription_type(Option<String>)`](crate::operation::get_subscription::GetSubscriptionOutput::pending_subscription_type): <p>The type of the billing plan that the space will be changed to at the start of the next billing cycle. This applies only to changes that reduce the functionality available for the space. Billing plan changes that increase functionality are applied immediately. For more information, see <a href="https://codecatalyst.aws/explore/pricing">Pricing</a>.</p>
    ///   - [`pending_subscription_start_time(Option<DateTime>)`](crate::operation::get_subscription::GetSubscriptionOutput::pending_subscription_start_time): <p>The day and time the pending change will be applied to the space, in coordinated universal time (UTC) timestamp format as specified in <a href="https://www.rfc-editor.org/rfc/rfc3339#section-5.6">RFC 3339</a>.</p>
    /// - On failure, responds with [`SdkError<GetSubscriptionError>`](crate::operation::get_subscription::GetSubscriptionError)
    pub fn get_subscription(&self) -> crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder {
        crate::operation::get_subscription::builders::GetSubscriptionFluentBuilder::new(self.handle.clone())
    }
}
