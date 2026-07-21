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

package io.zenoh.jni

import io.zenoh.jni.query.Parameters
import io.zenoh.jni.test.parametersContainsKey
import io.zenoh.jni.test.parametersExtend
import io.zenoh.jni.test.parametersGet
import io.zenoh.jni.test.parametersInsert
import io.zenoh.jni.test.parametersIsWellFormed
import io.zenoh.jni.test.parametersRemove
import io.zenoh.jni.test.parametersValues
import kotlin.random.Random
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * Self-verification of the shared pure-Kotlin [io.zenoh.jni.query.Parameters]
 * against the native zenoh-flat parameters API (the `parameters*` functions in
 * the test-only `io.zenoh.jni.test` package, thin wrappers over
 * `zenoh::query::Parameters`). The SDKs run the pure implementation on their
 * production path — crossing JNI per string operation is a JNI cost, not a
 * zenoh-flat design choice — so this crate verifies the correspondence itself,
 * driving both over edge shapes and randomized inputs and asserting equality.
 */
class ParametersCorrespondenceTest {

    // The native functions never throw — they take an onError sink; a failure
    // here is a test failure. `<Nothing>` satisfies every wrapper's `<out R>`.
    private val boom = JniErrorHandler<Nothing> { je ->
        throw AssertionError("unexpected native parameters error: $je")
    }

    /** Edge shapes of the format rules. */
    private val edgeCases = listOf(
        "", ";", ";;", "a", "a=", "=v", "=", "a=1", "a=1;b=2", "a=1;a=2",
        "a=b=c", "a==", "k=%zz", "k=%20", "flag;a=1", ";;a=1;;b=2;;",
        "c=1|2|3", "c=|", "c=ified|", "ключ=значение", "a=1;=2;b=3",
        " a = 1 ; b = 2 ", "a;a;a", "a=1;b;a=2",
    )

    private val keys = listOf("a", "b", "c", "k", "flag", "", "missing", "ключ", " a ")

    private fun assertCorrespondence(s: String) {
        val pure = Parameters.fromString(s)
        for (k in keys) {
            assertEquals(parametersGet(s, k, boom), pure.get(k), "get(\"$k\") for \"$s\"")
            assertEquals(parametersValues(s, k, boom), pure.values(k), "values(\"$k\") for \"$s\"")
            assertEquals(
                parametersContainsKey(s, k, boom),
                pure.containsKey(k),
                "containsKey(\"$k\") for \"$s\"",
            )
        }
        assertEquals(parametersIsWellFormed(s, boom), pure.isWellFormed(), "isWellFormed for \"$s\"")
        for (k in keys) {
            assertEquals(
                parametersInsert(s, k, "val", boom),
                Parameters.fromString(s).also { it.insert(k, "val") }.asString(),
                "insert(\"$k\", \"val\") for \"$s\"",
            )
        }
        assertEquals(
            parametersExtend(s, "zk1=zv1;zk2", boom),
            Parameters.fromString(s)
                .also { it.extend(Parameters.fromString("zk1=zv1;zk2")) }
                .asString(),
            "extend for \"$s\"",
        )
        // `remove` is deliberately NOT compared — see [nativeRemoveBugCanary].
    }

    @Test
    fun edgeCasesMatchNative() {
        for (s in edgeCases) assertCorrespondence(s)
    }

    @Test
    fun randomizedInputsMatchNative() {
        // Alphabet dense in separators, so structural collisions (duplicate
        // keys, empty chunks, '=' in values) are common.
        val alphabet = "ab;=|%; =;"
        val rng = Random(20260718)
        repeat(500) {
            val s = buildString {
                repeat(rng.nextInt(0, 24)) { append(alphabet[rng.nextInt(alphabet.length)]) }
            }
            assertCorrespondence(s)
        }
    }

    /** The shared implementation follows Rust's DOCUMENTED remove contract
     * ("preserving the insertion order"). */
    @Test
    fun removeFollowsTheDocumentedContract() {
        fun removed(s: String, k: String): String =
            Parameters.fromString(s).also { it.remove(k) }.asString()

        assertEquals("b=2;c=3", removed("b=2;a=1;c=3", "a"))
        assertEquals("b=2", removed("a=1;b=2;a=3", "a"))
        assertEquals("x=1;y=2", removed("x=1;y=2", "missing"))
        assertEquals("", removed("a=1", "a"))
        assertEquals("a=1", removed("flag;a=1", "flag"))
        assertEquals("1", Parameters.fromString("b=2;a=1").remove("a"))
    }

    /** CANARY: upstream zenoh's `parameters::remove` has an iterator-consumption
     * bug — `find` advances past every entry up to and including the first match
     * before `filter` builds the result, so entries PRECEDING the match are
     * dropped, and removing an absent key erases the whole string. The shared
     * implementation intentionally diverges (see [removeFollowsTheDocumentedContract]).
     * This pins the buggy native behavior: when upstream fixes it, this fails —
     * then delete it and fold `remove` into [assertCorrespondence]. */
    @Test
    fun nativeRemoveBugCanary() {
        assertEquals("c=3", parametersRemove("b=2;a=1;c=3", "a", boom))
        assertEquals("", parametersRemove("x=1;y=2", "missing", boom))
    }
}
