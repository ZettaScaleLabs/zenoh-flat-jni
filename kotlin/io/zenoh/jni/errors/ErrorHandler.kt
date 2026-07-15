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

package io.zenoh.jni.errors

/**
 * Compatibility name for consumers of the pre-rust-side-only `Error` binding.
 *
 * JniGen now emits the domain-error callback in the root package because the
 * Rust `Error` value is decomposed to its message and never becomes a Kotlin
 * handle. Keep the former package path source-compatible while consumers move
 * to [io.zenoh.jni.ErrorHandler].
 */
public typealias ErrorHandler<R> = io.zenoh.jni.ErrorHandler<R>
