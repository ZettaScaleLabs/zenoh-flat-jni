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

package io.zenoh.jni.config

/**
 * The standard string form of a Zenoh id, shared by every SDK built on these
 * bindings: the id bytes read as a little-endian integer, rendered as
 * lowercase hex without leading zeros (`"0"` for the zero id) — Zenoh's own
 * rule (uhlc `ID`'s `Debug`), in pure Kotlin over the value-class bytes; the
 * correspondence with the native formatter is verified by the consuming SDK's
 * test suite (`ZenohIdCorrespondenceTest` in zenoh-java). No JNI crossing.
 */
fun ZenohId.zidString(): String {
    val hex = "0123456789abcdef"
    val sb = StringBuilder(bytes.size * 2)
    for (i in bytes.indices.reversed()) {
        val b = bytes[i].toInt() and 0xFF
        sb.append(hex[b ushr 4]).append(hex[b and 0x0F])
    }
    val s = sb.trimStart('0').toString()
    return s.ifEmpty { "0" }
}
