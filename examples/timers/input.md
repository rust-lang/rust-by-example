Rust provides timers in two flavors: `oneshot` for a single use and `periodic`
to receive notifications over time. These timers provide the `Receiver` part of
a channel and will block the task until they timeout.

{timers.rs}
