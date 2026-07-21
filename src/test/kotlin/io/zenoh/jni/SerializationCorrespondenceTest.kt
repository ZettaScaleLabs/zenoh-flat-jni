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

import io.zenoh.jni.bytes.SerializationCodec
import io.zenoh.jni.bytes.SerializationCodec.SerdeType
import io.zenoh.jni.test.deserializeViaJNIKType
import io.zenoh.jni.test.serializeViaJNIKType
import kotlin.random.Random
import kotlin.reflect.typeOf
import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertEquals

/**
 * Self-verification of the pure-Kotlin [SerializationCodec] against the native
 * zenoh-ext serializer (the `serializeViaJNIKType` oracle in the test-only
 * `io.zenoh.jni.test` package). For each value the pure bytes must be
 * **byte-identical** to the native bytes, and the pure deserializer must
 * round-trip the native bytes. Inputs are edge values plus a seeded-`Random`
 * corpus. Also home to the perf comparison (pure vs per-element JNI), the
 * evidence that motivated replacing the JNI path.
 *
 * The [SerdeType] describing each value is built explicitly here (the SDKs build
 * it from their own reflection API — `KType` in zenoh-kotlin, `Type` in
 * zenoh-java); the native oracle takes the raw `KType` (`typeOf<T>()`).
 */
class SerializationCorrespondenceTest {

    private val boom = JniErrorHandler<Nothing> { je ->
        throw AssertionError("unexpected native serializer error: $je")
    }

    /** Pure bytes (via [type]) == native bytes (via `KType`), and pure
     *  round-trips the native bytes. */
    private inline fun <reified T : Any> assertCorresponds(value: T, type: SerdeType) {
        val pure = SerializationCodec.serialize(value, type, boom)
        val native = serializeViaJNIKType(value, typeOf<T>(), boom)
        assertContentEquals(native, pure, "serialize mismatch for <$value>")
        @Suppress("UNCHECKED_CAST")
        val back = SerializationCodec.deserialize(native, type, boom as JniErrorHandler<Any>)
        // ByteArray.equals is identity-based; compare by content.
        if (value is ByteArray && back is ByteArray) {
            assertContentEquals(value, back, "round-trip mismatch for <bytes>")
        } else {
            assertEquals(value, back, "round-trip mismatch for <$value>")
        }
    }

    // ── Golden vectors from the zenoh serialization RFC (no native lib) ──────

    @Test
    fun goldenVectorsMatchTheRfc() {
        assertContentEquals(
            byteArrayOf(134.toByte(), 214.toByte(), 18, 0),
            SerializationCodec.serialize(1234566, SerdeType.I32, boom),
        )
        assertContentEquals(
            byteArrayOf(4, 116, 101, 115, 116),
            SerializationCodec.serialize("test", SerdeType.Str, boom),
        )
        assertContentEquals(
            byteArrayOf(244.toByte(), 1),
            SerializationCodec.serialize(500.toUShort(), SerdeType.U16, boom),
        )
        assertContentEquals(
            byteArrayOf(0, 64, 154.toByte(), 68),
            SerializationCodec.serialize(1234.0f, SerdeType.F32, boom),
        )
    }

    // ── Scalars ─────────────────────────────────────────────────────────────

    @Test
    fun scalarsMatchNative() {
        val rng = Random(20260721)
        assertCorresponds(true, SerdeType.Bool); assertCorresponds(false, SerdeType.Bool)
        for (e in listOf(Int.MIN_VALUE, -1, 0, 1, Int.MAX_VALUE)) assertCorresponds(e, SerdeType.I32)
        for (e in listOf(Long.MIN_VALUE, 0L, Long.MAX_VALUE)) assertCorresponds(e, SerdeType.I64)
        assertCorresponds(Byte.MIN_VALUE, SerdeType.I8); assertCorresponds(Byte.MAX_VALUE, SerdeType.I8)
        assertCorresponds(Short.MIN_VALUE, SerdeType.I16); assertCorresponds(Short.MAX_VALUE, SerdeType.I16)
        assertCorresponds(3.1415f, SerdeType.F32); assertCorresponds(Float.MAX_VALUE, SerdeType.F32)
        assertCorresponds(2.718281828, SerdeType.F64); assertCorresponds(Double.MAX_VALUE, SerdeType.F64)
        repeat(200) {
            assertCorresponds(rng.nextInt(), SerdeType.I32)
            assertCorresponds(rng.nextLong(), SerdeType.I64)
            assertCorresponds(rng.nextInt().toShort(), SerdeType.I16)
            assertCorresponds(rng.nextInt().toByte(), SerdeType.I8)
            assertCorresponds(Float.fromBits(rng.nextInt()), SerdeType.F32)
            assertCorresponds(Double.fromBits(rng.nextLong()), SerdeType.F64)
        }
    }

    @Test
    fun unsignedMatchNative() {
        val rng = Random(20260722)
        assertCorresponds(UByte.MIN_VALUE, SerdeType.U8); assertCorresponds(UByte.MAX_VALUE, SerdeType.U8)
        assertCorresponds(UShort.MIN_VALUE, SerdeType.U16); assertCorresponds(UShort.MAX_VALUE, SerdeType.U16)
        assertCorresponds(UInt.MIN_VALUE, SerdeType.U32); assertCorresponds(UInt.MAX_VALUE, SerdeType.U32)
        assertCorresponds(ULong.MIN_VALUE, SerdeType.U64); assertCorresponds(ULong.MAX_VALUE, SerdeType.U64)
        repeat(200) {
            assertCorresponds(rng.nextInt().toUByte(), SerdeType.U8)
            assertCorresponds(rng.nextInt().toUShort(), SerdeType.U16)
            assertCorresponds(rng.nextInt().toUInt(), SerdeType.U32)
            assertCorresponds(rng.nextLong().toULong(), SerdeType.U64)
        }
    }

    @Test
    fun stringsAndBytesMatchNative() {
        val rng = Random(20260723)
        for (s in listOf("", "test", "ключ", "→∑≈", "a b\tc\n")) assertCorresponds(s, SerdeType.Str)
        assertCorresponds(byteArrayOf(1, 2, 3, 4), SerdeType.Bytes)
        repeat(100) {
            val s = buildString { repeat(rng.nextInt(0, 32)) { append(rng.nextInt(0x20, 0x7E).toChar()) } }
            assertCorresponds(s, SerdeType.Str)
        }
    }

    // ── Containers, tuples, nesting ─────────────────────────────────────────

    @Test
    fun containersAndTuplesMatchNative() {
        val rng = Random(20260724)
        val listI = SerdeType.ZList(SerdeType.I32)
        assertCorresponds(listOf(1, 2, 3, 4, 5), listI)
        assertCorresponds(emptyList<Int>(), listI)
        assertCorresponds(listOf("a", "bb", "ccc"), SerdeType.ZList(SerdeType.Str))
        assertCorresponds(listOf(true, false, true), SerdeType.ZList(SerdeType.Bool))
        assertCorresponds(mapOf("a" to 1, "b" to 2), SerdeType.ZMap(SerdeType.Str, SerdeType.I32))
        assertCorresponds(mapOf("x" to 10uL, "y" to 20uL), SerdeType.ZMap(SerdeType.Str, SerdeType.U64))
        assertCorresponds(1 to 2.5, SerdeType.ZPair(SerdeType.I32, SerdeType.F64))
        assertCorresponds(
            Triple(1, 2.5, listOf(true, false)),
            SerdeType.ZTriple(SerdeType.I32, SerdeType.F64, SerdeType.ZList(SerdeType.Bool)),
        )
        // Nested
        assertCorresponds(
            listOf(mapOf("a" to 1uL), mapOf("b" to 2uL)),
            SerdeType.ZList(SerdeType.ZMap(SerdeType.Str, SerdeType.U64)),
        )
        assertCorresponds(
            mapOf("k" to listOf(1, 2, 3)),
            SerdeType.ZMap(SerdeType.Str, SerdeType.ZList(SerdeType.I32)),
        )
        assertCorresponds(
            1 to (2.5 to false),
            SerdeType.ZPair(SerdeType.I32, SerdeType.ZPair(SerdeType.F64, SerdeType.Bool)),
        )
        repeat(100) {
            assertCorresponds(List(rng.nextInt(0, 8)) { rng.nextInt() }, listI)
            assertCorresponds(
                (0 until rng.nextInt(0, 6)).associate { "k$it" to rng.nextLong() },
                SerdeType.ZMap(SerdeType.Str, SerdeType.I64),
            )
            assertCorresponds(
                rng.nextInt() to rng.nextLong().toULong(),
                SerdeType.ZPair(SerdeType.I32, SerdeType.U64),
            )
        }
    }

    // ── Perf comparison (pure Kotlin vs per-element JNI) ────────────────────

    @Test
    fun perfPureVsJni() {
        val warmup = 20_000
        val n = 200_000L
        benchPair("Int", 42, SerdeType.I32, warmup, n)
        benchPair("List<Int>(4)", listOf(1, 2, 3, 4), SerdeType.ZList(SerdeType.I32), warmup, n)
        benchPair("Map<String,Int>(2)", mapOf("a" to 1, "b" to 2), SerdeType.ZMap(SerdeType.Str, SerdeType.I32), warmup, n)
    }

    private inline fun <reified T : Any> benchPair(label: String, value: T, type: SerdeType, warmup: Int, n: Long) {
        val kt = typeOf<T>()
        repeat(warmup) { SerializationCodec.serialize(value, type, boom); serializeViaJNIKType(value, kt, boom) }
        val pureNs = time(n) { SerializationCodec.serialize(value, type, boom) }
        val jniNs = time(n) { serializeViaJNIKType(value, kt, boom) }
        println(
            "serialize %-20s pure=%8.1f ns/op   jni=%9.1f ns/op   speedup=%.1fx"
                .format(label, pureNs, jniNs, jniNs / pureNs)
        )
    }

    private inline fun time(n: Long, body: () -> Unit): Double {
        val start = System.nanoTime()
        var i = 0L
        while (i < n) { body(); i++ }
        return (System.nanoTime() - start).toDouble() / n
    }
}
