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
import java.util.concurrent.atomic.AtomicReference

/**
 * A lazily materialized, GC-managed native [Encoding] handle for one immutable
 * `(id, schema)` encoding value. Shared by every SDK built on these bindings
 * (zenoh-java, zenoh-kotlin): an SDK-tier encoding value embeds one of these
 * and passes [getOrCreate]'s handle to the typed `encoding: Encoding?` send
 * overloads (`put`/`get`/`declarePublisher`/`replySuccess`/…), so a saved
 * encoding crosses JNI as a bare `jlong` on every send instead of re-decoding
 * its schema string each call. Every native encoding parameter is borrowed
 * (`Option<&Encoding>`, cloned Rust-side), so the cached handle is never
 * consumed and stays valid for unlimited sends.
 *
 * The first [getOrCreate] pays one `encoding_new_from_id` crossing; concurrent
 * first calls race benignly (CAS — the loser closes its fresh handle). The
 * SDK value stays a plain non-closeable value: the native handle is released
 * by a [Cleaner] once the handle is unreachable, which — since this cache
 * holds the only long-lived reference — is when the owning SDK value dies and
 * any in-flight native call holding the handle has returned.
 */
public class CachedEncodingHandle {

    private val ref = AtomicReference<Encoding?>()

    /** Whether a native handle has been materialized (test/diagnostics). */
    public val isMaterialized: Boolean
        get() = ref.get() != null

    /**
     * The cached native handle, materializing it from `(id, schema)` on first
     * use. The caller must pass the same value pair every time — the cache is
     * keyed by the owning encoding value's identity, not by the arguments.
     */
    public fun getOrCreate(id: Int, schema: String?): Encoding {
        ref.get()?.let { return it }
        val fresh = Encoding.newFromId(id, schema) { je ->
            throw IllegalStateException("encoding_new_from_id binding error: $je")
        }
        return if (ref.compareAndSet(null, fresh)) {
            // Register on the HANDLE, capturing only its raw pointer: the
            // cache's strong ref keeps it alive as long as the owning SDK
            // value, and an in-flight native call keeps it alive past the
            // owner's death — so the cleaner can never free a handle that a
            // running call is using. Nothing else ever closes this handle
            // (borrow-only escapes), so the raw free cannot double-free.
            val rawPtr = fresh.ptr
            CLEANER.register(fresh) { Encoding.freePtr(rawPtr) }
            fresh
        } else {
            fresh.close()
            ref.get()!!
        }
    }

    private companion object {
        val CLEANER: Cleaner = Cleaner.create()
    }
}
