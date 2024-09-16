use crate::bindings::exports;
use crate::bindings::wasi::clocks::monotonic_clock;
use crate::bindings::wasi::clocks::monotonic_clock::Duration;
use crate::bindings::wasi::clocks::monotonic_clock::Instant;
use crate::Passthrough;

impl<T: ?Sized> exports::wasi::clocks::monotonic_clock::Guest for Passthrough<T> {
    fn now() -> Instant {
        monotonic_clock::now()
    }

    fn resolution() -> Duration {
        monotonic_clock::resolution()
    }

    fn subscribe_instant(when: Instant) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(monotonic_clock::subscribe_instant(when))
    }

    fn subscribe_duration(when: Duration) -> exports::wasi::io::poll::Pollable {
        exports::wasi::io::poll::Pollable::new(monotonic_clock::subscribe_duration(when))
    }
}
