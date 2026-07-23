//
// Copyright (c) 2026 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

//! Binding-local conversion for the semantic `std::time::Duration` used by the
//! advanced pub/sub options. `build.rs` declares (via `convert!(Duration)`) that
//! the foreign representation is bounded `u64` milliseconds whose out-of-range
//! values provide the `Option<Duration>` niche, so `Option<Duration>` crosses
//! JNI as a raw primitive `Long` without a boxed-`Long`/`JObject` allocation.
//! The semantic Rust type stays intact in the zenoh-flat API.

use zenoh_flat::Duration;

/// Wire (`u64` milliseconds) → semantic `Duration`.
pub fn duration_from_millis(ms: u64) -> Duration {
    Duration::from_millis(ms)
}

/// Semantic `Duration` → wire (`u64` milliseconds); durations that do not fit in
/// `u64` milliseconds are rejected (routed to the binding error channel).
pub fn duration_to_millis(d: Duration) -> Result<u64, String> {
    u64::try_from(d.as_millis())
        .map_err(|_| "duration does not fit in u64 milliseconds".to_string())
}
