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

package io.zenoh.jni.query

/**
 * Selector parameters, shared by every SDK built on these bindings
 * (zenoh-java, zenoh-kotlin).
 *
 * A STRING-BACKED, pure-Kotlin mirror of Rust's
 * `zenoh-protocol/core/parameters.rs` (`zenoh::query::Parameters`): the
 * canonical state is the raw `a=b;c=d|e;f=g` string, exactly as it travels on
 * the wire, and every operation reproduces the Rust semantics —
 *
 * - construction trims trailing `;`, `=`, `|` characters (Rust
 *   `Parameters::from`'s `trim_end_matches`); the rest is stored verbatim.
 * - iteration splits on `;`, SKIPS empty chunks, and splits each chunk on the
 *   FIRST `=` (no `=` ⇒ empty value). There is NO percent-decoding.
 * - construction from any string is infallible: parameters arrive from the
 *   network (a remote client controls them), and the binding accepts
 *   everything the Rust layer accepts — which is everything.
 * - [get] returns the FIRST matching entry; duplicates are preserved in the
 *   string ([asString] round-trips verbatim) and only normalized away by
 *   [insert]/[remove].
 * - [insert] removes every entry for the key and appends the new pair at the
 *   end; rebuilding skips empty keys and omits `=` for empty values.
 * - [toMap] collects iteration order into a map, so for duplicated keys the
 *   LAST entry wins there (Rust's `HashMap` conversion), unlike [get].
 *
 * No JNI crossing on any path: zenoh-flat exposes the same operations as
 * regular API (the `parameters*` functions in this package), but calling
 * them per string operation would cross JNI each time — a JNI peculiarity,
 * not a zenoh-flat design choice — so the JVM production path runs this pure
 * implementation instead. The correspondence between the two is verified by
 * the consuming SDK's test suite (`ParametersCorrespondenceTest` in
 * zenoh-java).
 */
public class Parameters private constructor(private var inner: String) {

    public companion object {
        public const val LIST_SEPARATOR: Char = ';'
        public const val FIELD_SEPARATOR: Char = '='
        public const val VALUE_SEPARATOR: Char = '|'

        /** Wrap any string as parameters — infallible. As in Rust's
         * `Parameters::from`, trailing `;`, `=`, `|` characters are trimmed
         * at construction; the rest is stored verbatim. */
        public fun fromString(s: String): Parameters =
            Parameters(s.trimEnd(LIST_SEPARATOR, FIELD_SEPARATOR, VALUE_SEPARATOR))

        /** Empty parameters. */
        public fun empty(): Parameters = Parameters("")

        /** Build from key-value pairs (iteration order preserved). */
        public fun fromMap(map: Map<String, String>): Parameters =
            Parameters(concat(map.entries.asSequence().map { it.key to it.value }))

        private fun splitOnce(s: String, c: Char): Pair<String, String> {
            val i = s.indexOf(c)
            return if (i >= 0) s.substring(0, i) to s.substring(i + 1) else s to ""
        }

        /** Rebuild a parameters string: empty keys skipped, `=` omitted for
         * empty values (Rust `concat_into`). */
        private fun concat(pairs: Sequence<Pair<String, String>>): String {
            val sb = StringBuilder()
            var first = true
            for ((k, v) in pairs) {
                if (k.isEmpty()) continue
                if (!first) sb.append(LIST_SEPARATOR)
                sb.append(k)
                if (v.isNotEmpty()) {
                    sb.append(FIELD_SEPARATOR)
                    sb.append(v)
                }
                first = false
            }
            return sb.toString()
        }
    }

    /** The raw parameters string, verbatim. */
    public fun asString(): String = inner

    /** Key-value pairs in string order (Rust `iter`): `;`-split, empty
     * chunks skipped, each chunk split on its first `=`. */
    public fun iter(): List<Pair<String, String>> =
        inner.split(LIST_SEPARATOR)
            .filter { it.isNotEmpty() }
            .map { splitOnce(it, FIELD_SEPARATOR) }

    public fun isEmpty(): Boolean = iter().isEmpty()

    public fun containsKey(key: String): Boolean = iter().any { it.first == key }

    /** The FIRST matching entry's value (Rust `get`). */
    public fun get(key: String): String? = iter().firstOrNull { it.first == key }?.second

    public fun getOrDefault(key: String, default: String): String = get(key) ?: default

    /** The first matching entry's value split on `|` (Rust `values`);
     * empty when the key is absent. */
    public fun values(key: String): List<String> =
        get(key)?.split(VALUE_SEPARATOR) ?: emptyList()

    /** `true` if at least one entry exists and no key is empty
     * (Rust `is_well_formed`). */
    public fun isWellFormed(): Boolean {
        val entries = iter()
        return entries.isNotEmpty() && entries.all { it.first.isNotEmpty() }
    }

    /** Remove every entry for [key] and append `(key, value)` at the end;
     * returns the previous FIRST value (Rust `insert`). The string is
     * normalized as a side effect. */
    public fun insert(key: String, value: String): String? {
        val old = get(key)
        inner = concat(
            iter().asSequence().filter { it.first != key } + sequenceOf(key to value)
        )
        return old
    }

    /** Remove every entry for [key]; returns the previous FIRST value.
     * The string is normalized as a side effect.
     *
     * NOTE: this follows Rust's DOCUMENTED `remove` contract ("preserving
     * the insertion order"), not its current implementation, which has an
     * iterator-consumption bug that also drops every entry PRECEDING the
     * first match (and erases everything when the key is absent). The
     * divergence is pinned by `ParametersCorrespondenceTest`'s canary in
     * zenoh-java; when upstream zenoh fixes `parameters::remove`, the canary
     * fails and full remove-correspondence can be enabled. */
    public fun remove(key: String): String? {
        val old = get(key)
        inner = concat(iter().asSequence().filter { it.first != key })
        return old
    }

    /** Insert every pair of [other] (their values win — Rust `extend`). */
    public fun extend(other: Parameters) {
        for ((k, v) in other.iter()) insert(k, v)
    }

    /** Insert every pair of [map] (its values win). */
    public fun extend(map: Map<String, String>) {
        for ((k, v) in map) insert(k, v)
    }

    /** Iteration order collected into a map — for duplicated keys the LAST
     * entry wins here (Rust's `HashMap` conversion), unlike [get]. */
    public fun toMap(): Map<String, String> = iter().toMap()

    /** A deep copy backed by the same string. */
    public fun copy(): Parameters = Parameters(inner)

    override fun toString(): String = inner

    /** String equality, as Rust: `"a=1;b=2" != "b=2;a=1"`. */
    override fun equals(other: Any?): Boolean =
        other is Parameters && inner == other.inner

    override fun hashCode(): Int = inner.hashCode()
}
