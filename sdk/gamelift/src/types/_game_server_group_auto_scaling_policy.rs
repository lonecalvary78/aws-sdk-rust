// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p><b>This data type is used with the Amazon GameLift Servers FleetIQ and game server groups.</b></p>
/// <p>Configuration settings for intelligent automatic scaling that uses target tracking. These settings are used to add an Auto Scaling policy when creating the corresponding Auto Scaling group. After the Auto Scaling group is created, all updates to Auto Scaling policies, including changing this policy and adding or removing other policies, is done directly on the Auto Scaling group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GameServerGroupAutoScalingPolicy {
    /// <p>Length of time, in seconds, it takes for a new instance to start new game server processes and register with Amazon GameLift Servers FleetIQ. Specifying a warm-up time can be useful, particularly with game servers that take a long time to start up, because it avoids prematurely starting new instances.</p>
    pub estimated_instance_warmup: ::std::option::Option<i32>,
    /// <p>Settings for a target-based scaling policy applied to Auto Scaling group. These settings are used to create a target-based policy that tracks the Amazon GameLift Servers FleetIQ metric <code>"PercentUtilizedGameServers"</code> and specifies a target value for the metric. As player usage changes, the policy triggers to adjust the game server group capacity so that the metric returns to the target value.</p>
    pub target_tracking_configuration: ::std::option::Option<crate::types::TargetTrackingConfiguration>,
}
impl GameServerGroupAutoScalingPolicy {
    /// <p>Length of time, in seconds, it takes for a new instance to start new game server processes and register with Amazon GameLift Servers FleetIQ. Specifying a warm-up time can be useful, particularly with game servers that take a long time to start up, because it avoids prematurely starting new instances.</p>
    pub fn estimated_instance_warmup(&self) -> ::std::option::Option<i32> {
        self.estimated_instance_warmup
    }
    /// <p>Settings for a target-based scaling policy applied to Auto Scaling group. These settings are used to create a target-based policy that tracks the Amazon GameLift Servers FleetIQ metric <code>"PercentUtilizedGameServers"</code> and specifies a target value for the metric. As player usage changes, the policy triggers to adjust the game server group capacity so that the metric returns to the target value.</p>
    pub fn target_tracking_configuration(&self) -> ::std::option::Option<&crate::types::TargetTrackingConfiguration> {
        self.target_tracking_configuration.as_ref()
    }
}
impl GameServerGroupAutoScalingPolicy {
    /// Creates a new builder-style object to manufacture [`GameServerGroupAutoScalingPolicy`](crate::types::GameServerGroupAutoScalingPolicy).
    pub fn builder() -> crate::types::builders::GameServerGroupAutoScalingPolicyBuilder {
        crate::types::builders::GameServerGroupAutoScalingPolicyBuilder::default()
    }
}

/// A builder for [`GameServerGroupAutoScalingPolicy`](crate::types::GameServerGroupAutoScalingPolicy).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GameServerGroupAutoScalingPolicyBuilder {
    pub(crate) estimated_instance_warmup: ::std::option::Option<i32>,
    pub(crate) target_tracking_configuration: ::std::option::Option<crate::types::TargetTrackingConfiguration>,
}
impl GameServerGroupAutoScalingPolicyBuilder {
    /// <p>Length of time, in seconds, it takes for a new instance to start new game server processes and register with Amazon GameLift Servers FleetIQ. Specifying a warm-up time can be useful, particularly with game servers that take a long time to start up, because it avoids prematurely starting new instances.</p>
    pub fn estimated_instance_warmup(mut self, input: i32) -> Self {
        self.estimated_instance_warmup = ::std::option::Option::Some(input);
        self
    }
    /// <p>Length of time, in seconds, it takes for a new instance to start new game server processes and register with Amazon GameLift Servers FleetIQ. Specifying a warm-up time can be useful, particularly with game servers that take a long time to start up, because it avoids prematurely starting new instances.</p>
    pub fn set_estimated_instance_warmup(mut self, input: ::std::option::Option<i32>) -> Self {
        self.estimated_instance_warmup = input;
        self
    }
    /// <p>Length of time, in seconds, it takes for a new instance to start new game server processes and register with Amazon GameLift Servers FleetIQ. Specifying a warm-up time can be useful, particularly with game servers that take a long time to start up, because it avoids prematurely starting new instances.</p>
    pub fn get_estimated_instance_warmup(&self) -> &::std::option::Option<i32> {
        &self.estimated_instance_warmup
    }
    /// <p>Settings for a target-based scaling policy applied to Auto Scaling group. These settings are used to create a target-based policy that tracks the Amazon GameLift Servers FleetIQ metric <code>"PercentUtilizedGameServers"</code> and specifies a target value for the metric. As player usage changes, the policy triggers to adjust the game server group capacity so that the metric returns to the target value.</p>
    /// This field is required.
    pub fn target_tracking_configuration(mut self, input: crate::types::TargetTrackingConfiguration) -> Self {
        self.target_tracking_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Settings for a target-based scaling policy applied to Auto Scaling group. These settings are used to create a target-based policy that tracks the Amazon GameLift Servers FleetIQ metric <code>"PercentUtilizedGameServers"</code> and specifies a target value for the metric. As player usage changes, the policy triggers to adjust the game server group capacity so that the metric returns to the target value.</p>
    pub fn set_target_tracking_configuration(mut self, input: ::std::option::Option<crate::types::TargetTrackingConfiguration>) -> Self {
        self.target_tracking_configuration = input;
        self
    }
    /// <p>Settings for a target-based scaling policy applied to Auto Scaling group. These settings are used to create a target-based policy that tracks the Amazon GameLift Servers FleetIQ metric <code>"PercentUtilizedGameServers"</code> and specifies a target value for the metric. As player usage changes, the policy triggers to adjust the game server group capacity so that the metric returns to the target value.</p>
    pub fn get_target_tracking_configuration(&self) -> &::std::option::Option<crate::types::TargetTrackingConfiguration> {
        &self.target_tracking_configuration
    }
    /// Consumes the builder and constructs a [`GameServerGroupAutoScalingPolicy`](crate::types::GameServerGroupAutoScalingPolicy).
    pub fn build(self) -> crate::types::GameServerGroupAutoScalingPolicy {
        crate::types::GameServerGroupAutoScalingPolicy {
            estimated_instance_warmup: self.estimated_instance_warmup,
            target_tracking_configuration: self.target_tracking_configuration,
        }
    }
}
