//
// Copyright (c) 2023 ZettaScale Technology
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

@file:JvmName("Serialization")

package io.zenoh.jni.test

import java.lang.reflect.Type
import kotlin.reflect.KType

// TEST-ONLY native serialization oracle. The SDKs (zenoh-java, zenoh-kotlin) are
// NOT meant to import this package — their production path uses the pure-Kotlin
// `io.zenoh.jni.bytes.SerializationCodec`. These per-element JNI externs exist
// only so this crate's `SerializationCorrespondenceTest` can verify the pure
// codec against the real zenoh-ext serializer (and measure the JNI baseline).

// Forces the native library to load before the first JNI call. The standalone
// (de)serialization path has no Session to trigger the load, and this facade
// class's <clinit> runs before either external fn below is invoked. All loading
// logic lives in the shared `io.zenoh.jni.NativeLibrary`.
private val ensureLoaded = run { io.zenoh.jni.NativeLibrary.ensureLoaded() }

// `onError` mirrors the generated wrappers' error callback: on a serialization
// failure the native side invokes it with the message (the binding-error `je`
// arity), and the handler throws — no direct throw from native code.

// `java.lang.reflect.Type` path (Guava TypeToken). Cannot express Kotlin's
// unsigned value classes (they erase) or Pair/Triple.
public external fun serializeViaJNI(any: Any, type: Type, onError: io.zenoh.jni.JniErrorHandler<ByteArray>): ByteArray

public external fun deserializeViaJNI(bytes: ByteArray, type: Type, onError: io.zenoh.jni.JniErrorHandler<Any>): Any

// `kotlin.reflect.KType` path. The native side inspects the KType classifier,
// so it additionally supports UByte/UShort/UInt/ULong and Pair/Triple (see
// Serialization.serializeViaJNIKType in the Rust crate).
public external fun serializeViaJNIKType(any: Any, kType: KType, onError: io.zenoh.jni.JniErrorHandler<ByteArray>): ByteArray

public external fun deserializeViaJNIKType(bytes: ByteArray, kType: KType, onError: io.zenoh.jni.JniErrorHandler<Any>): Any
