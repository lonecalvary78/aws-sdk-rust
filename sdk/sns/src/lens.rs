// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_endpoints_by_platform_application_output_output_next_token(
    input: &crate::operation::list_endpoints_by_platform_application::ListEndpointsByPlatformApplicationOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_origination_numbers_output_output_next_token(
    input: &crate::operation::list_origination_numbers::ListOriginationNumbersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_phone_numbers_opted_out_output_output_next_token(
    input: &crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_platform_applications_output_output_next_token(
    input: &crate::operation::list_platform_applications::ListPlatformApplicationsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_sms_sandbox_phone_numbers_output_output_next_token(
    input: &crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_subscriptions_output_output_next_token(
    input: &crate::operation::list_subscriptions::ListSubscriptionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_subscriptions_by_topic_output_output_next_token(
    input: &crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_topics_output_output_next_token(
    input: &crate::operation::list_topics::ListTopicsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_endpoints_by_platform_application_output_output_endpoints(
    input: crate::operation::list_endpoints_by_platform_application::ListEndpointsByPlatformApplicationOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Endpoint>> {
    let input = input.endpoints?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_origination_numbers_output_output_phone_numbers(
    input: crate::operation::list_origination_numbers::ListOriginationNumbersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberInformation>> {
    let input = input.phone_numbers?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_phone_numbers_opted_out_output_output_phone_numbers(
    input: crate::operation::list_phone_numbers_opted_out::ListPhoneNumbersOptedOutOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = input.phone_numbers?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_platform_applications_output_output_platform_applications(
    input: crate::operation::list_platform_applications::ListPlatformApplicationsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::PlatformApplication>> {
    let input = input.platform_applications?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_sms_sandbox_phone_numbers_output_output_phone_numbers(
    input: crate::operation::list_sms_sandbox_phone_numbers::ListSmsSandboxPhoneNumbersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::SmsSandboxPhoneNumber>> {
    let input = input.phone_numbers;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_subscriptions_output_output_subscriptions(
    input: crate::operation::list_subscriptions::ListSubscriptionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Subscription>> {
    let input = input.subscriptions?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_subscriptions_by_topic_output_output_subscriptions(
    input: crate::operation::list_subscriptions_by_topic::ListSubscriptionsByTopicOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Subscription>> {
    let input = input.subscriptions?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_topics_output_output_topics(
    input: crate::operation::list_topics::ListTopicsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Topic>> {
    let input = input.topics?;
    ::std::option::Option::Some(input)
}
