// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetEventRule`](crate::operation::get_event_rule::builders::GetEventRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::get_event_rule::builders::GetEventRuleFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::get_event_rule::builders::GetEventRuleFluentBuilder::set_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the <code>EventRule</code> to return.</p><br>
    /// - On success, responds with [`GetEventRuleOutput`](crate::operation::get_event_rule::GetEventRuleOutput) with field(s):
    ///   - [`arn(String)`](crate::operation::get_event_rule::GetEventRuleOutput::arn): <p>The ARN of the resource.</p>
    ///   - [`notification_configuration_arn(String)`](crate::operation::get_event_rule::GetEventRuleOutput::notification_configuration_arn): <p>The ARN of a <code>NotificationConfiguration</code>.</p>
    ///   - [`creation_time(DateTime)`](crate::operation::get_event_rule::GetEventRuleOutput::creation_time): <p>The date when the <code>EventRule</code> was created.</p>
    ///   - [`source(String)`](crate::operation::get_event_rule::GetEventRuleOutput::source): <p>The matched event source.</p> <p>Must match one of the valid EventBridge sources. Only Amazon Web Services service sourced events are supported. For example, <code>aws.ec2</code> and <code>aws.cloudwatch</code>. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-delivery-level">Event delivery from Amazon Web Services services</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    ///   - [`event_type(String)`](crate::operation::get_event_rule::GetEventRuleOutput::event_type): <p>The event type to match.</p> <p>Must match one of the valid Amazon EventBridge event types. For example, EC2 Instance State-change Notification and Amazon CloudWatch Alarm State Change. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-delivery-level">Event delivery from Amazon Web Services services</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    ///   - [`event_pattern(String)`](crate::operation::get_event_rule::GetEventRuleOutput::event_pattern): <p>An additional event pattern used to further filter the events this <code>EventRule</code> receives.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-event-patterns.html">Amazon EventBridge event patterns</a> in the <i>Amazon EventBridge User Guide.</i></p>
    ///   - [`regions(Vec::<String>)`](crate::operation::get_event_rule::GetEventRuleOutput::regions): <p>A list of Amazon Web Services Regions that send events to this <code>EventRule</code>.</p>
    ///   - [`managed_rules(Vec::<String>)`](crate::operation::get_event_rule::GetEventRuleOutput::managed_rules): <p>A list of managed rules from EventBridge that are associated with this <code>EventRule</code>.</p><note>  <p>These are created by User Notifications within your account so this <code>EventRule</code> functions.</p> </note>
    ///   - [`status_summary_by_region(HashMap::<String, EventRuleStatusSummary>)`](crate::operation::get_event_rule::GetEventRuleOutput::status_summary_by_region): <p>A list of an <code>EventRule</code>'s status by Region. Regions are mapped to <code>EventRuleStatusSummary</code>.</p>
    /// - On failure, responds with [`SdkError<GetEventRuleError>`](crate::operation::get_event_rule::GetEventRuleError)
    pub fn get_event_rule(&self) -> crate::operation::get_event_rule::builders::GetEventRuleFluentBuilder {
        crate::operation::get_event_rule::builders::GetEventRuleFluentBuilder::new(self.handle.clone())
    }
}
