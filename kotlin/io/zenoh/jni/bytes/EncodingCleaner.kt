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

package io.zenoh.jni.bytes

import java.lang.ref.Cleaner

/**
 * GC-managed release of owned [Encoding] handles held inside SDK encoding
 * values. Shared by every SDK built on these bindings (zenoh-java,
 * zenoh-kotlin): an SDK encoding value is a plain non-closeable value, so a
 * handle it owns — created with a custom encoding or delivered by a receive
 * decomposition — is freed by a [Cleaner] instead of an explicit `close()`.
 *
 * Registration is keyed to the *handle's* reachability and captures only its
 * raw pointer: the owning SDK value's strong reference keeps the handle alive,
 * and an in-flight native call keeps it alive past the owner's death, so the
 * cleaner can never free a handle a running call is using. The handle must
 * never be closed by any other path (send paths only borrow it) — the raw
 * free is unconditional.
 */
public object EncodingCleaner {

    private val CLEANER: Cleaner = Cleaner.create()

    /** Register [handle] for release when it becomes unreachable. */
    public fun register(handle: Encoding) {
        val rawPtr = handle.ptr
        CLEANER.register(handle) { Encoding.freePtr(rawPtr) }
    }
}
