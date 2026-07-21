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

import io.zenoh.jni.JniErrorHandler

/**
 * Pure-JVM implementation of the Zenoh serialization format, shared by every
 * SDK built on these bindings (zenoh-java, zenoh-kotlin).
 *
 * The Zenoh serialization wire format ([RFC][1], implemented by
 * `zenoh_ext::{ZSerializer, ZDeserializer}`) is a simple byte format, so it is
 * reimplemented here in Kotlin and produces/consumes the `ByteArray` directly —
 * **no JNI crossing per element** (the previous `serializeViaJNI` path made a
 * native up-call for every scalar/leaf, which dominates small payloads). The
 * byte-for-byte correspondence with the native serializer is verified by the
 * consuming SDK's `ZSerdeCorrespondenceTest`.
 *
 * **Error model** — like every other zenoh-flat-jni function (the generated
 * wrappers and the hand-written surface alike), these functions **never throw**:
 * [serialize] / [deserialize] take a trailing [JniErrorHandler] and, on any
 * failure, invoke `onError.run(message)` and return whatever it returns (the
 * caller's sentinel). So a consuming SDK wires them exactly as it wires a
 * generated wrapper — no special case for hand-written code.
 *
 * Format summary:
 * - scalars: little-endian fixed width (`to_le_bytes`); `bool` = 1 byte (0/1);
 *   unsigned values use the same fixed LE width as signed.
 * - lengths: LEB128 unsigned ([VarInt]).
 * - list/string/byte-array: `VarInt(len)` then contiguous elements
 *   (`String` = its UTF-8 bytes).
 * - map: `VarInt(count)` then `key, value` per entry.
 * - tuple (`Pair`/`Triple`): fields concatenated, no length prefix.
 *
 * A caller describes the value's shape with a [SerdeType], which it builds from
 * its own reflection representation (each SDK has a small adapter — `KType` for
 * Kotlin, `java.lang.reflect.Type` for Java — since the two reflection APIs
 * can't be shared; the byte codec below is what's shared).
 *
 * [1]: https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Serialization.md
 */
object SerializationCodec {

    /**
     * The recursive type descriptor. Mirrors the set of types the native
     * serializer supports. The unsigned and [ZPair]/[ZTriple] leaves are only
     * producible from a Kotlin `KType` (Java erases them).
     */
    sealed interface SerdeType {
        object Bool : SerdeType
        object I8 : SerdeType
        object I16 : SerdeType
        object I32 : SerdeType
        object I64 : SerdeType
        object U8 : SerdeType
        object U16 : SerdeType
        object U32 : SerdeType
        object U64 : SerdeType
        object F32 : SerdeType
        object F64 : SerdeType
        object Str : SerdeType
        object Bytes : SerdeType
        data class ZList(val elem: SerdeType) : SerdeType
        data class ZMap(val key: SerdeType, val value: SerdeType) : SerdeType
        data class ZPair(val first: SerdeType, val second: SerdeType) : SerdeType
        data class ZTriple(val first: SerdeType, val second: SerdeType, val third: SerdeType) : SerdeType
    }

    /** Signals a (de)serialization failure — truncated input, invalid `bool`,
     *  non-UTF-8 string, leftover bytes, or a value that doesn't match its
     *  [SerdeType]. Never escapes [serialize] / [deserialize] (routed to the
     *  `onError` handler); the SDKs' `SerdeType` builders may throw it too. */
    class SerdeException(message: String) : RuntimeException(message)

    // ── Public entry points (callback-based, never throw) ─────────────────────

    /**
     * Serialize [value] (described by [type]) to the Zenoh wire format. On any
     * failure, invokes [onError] with the message and returns its result — the
     * same no-throw, error-sink convention as the generated wrappers.
     */
    fun serialize(value: Any, type: SerdeType, onError: JniErrorHandler<ByteArray>): ByteArray =
        try {
            val w = Writer()
            writeValue(w, value, type)
            w.toByteArray()
        } catch (e: Exception) {
            onError.run(e.message ?: "serialization error")
        }

    /**
     * Deserialize [bytes] (described by [type]); fails if bytes remain. On any
     * failure, invokes [onError] with the message and returns its result.
     */
    fun deserialize(bytes: ByteArray, type: SerdeType, onError: JniErrorHandler<Any>): Any =
        try {
            val r = Reader(bytes)
            val v = readValue(r, type)
            if (!r.done()) throw SerdeException("trailing bytes after deserialization")
            v
        } catch (e: Exception) {
            onError.run(e.message ?: "deserialization error")
        }

    // ── Serialization ────────────────────────────────────────────────────────

    private fun writeValue(w: Writer, value: Any, type: SerdeType) {
        when (type) {
            SerdeType.Bool -> w.putU8(if (value as Boolean) 1 else 0)
            SerdeType.I8 -> w.putU8((value as Byte).toInt())
            SerdeType.I16 -> w.putLe16((value as Short).toInt())
            SerdeType.I32 -> w.putLe32(value as Int)
            SerdeType.I64 -> w.putLe64(value as Long)
            SerdeType.U8 -> w.putU8((value as UByte).toInt())
            SerdeType.U16 -> w.putLe16((value as UShort).toInt())
            SerdeType.U32 -> w.putLe32((value as UInt).toInt())
            SerdeType.U64 -> w.putLe64((value as ULong).toLong())
            SerdeType.F32 -> w.putLe32((value as Float).toRawBits())
            SerdeType.F64 -> w.putLe64((value as Double).toRawBits())
            SerdeType.Str -> {
                val bytes = (value as String).encodeToByteArray()
                w.putVarInt(bytes.size.toLong())
                w.putBytes(bytes)
            }
            SerdeType.Bytes -> {
                val bytes = value as ByteArray
                w.putVarInt(bytes.size.toLong())
                w.putBytes(bytes)
            }
            is SerdeType.ZList -> {
                val list = value as List<*>
                w.putVarInt(list.size.toLong())
                for (e in list) writeValue(w, e!!, type.elem)
            }
            is SerdeType.ZMap -> {
                val map = value as Map<*, *>
                w.putVarInt(map.size.toLong())
                for ((k, v) in map) {
                    writeValue(w, k!!, type.key)
                    writeValue(w, v!!, type.value)
                }
            }
            is SerdeType.ZPair -> {
                val pair = value as Pair<*, *>
                writeValue(w, pair.first!!, type.first)
                writeValue(w, pair.second!!, type.second)
            }
            is SerdeType.ZTriple -> {
                val triple = value as Triple<*, *, *>
                writeValue(w, triple.first!!, type.first)
                writeValue(w, triple.second!!, type.second)
                writeValue(w, triple.third!!, type.third)
            }
        }
    }

    // ── Deserialization ──────────────────────────────────────────────────────

    private fun readValue(r: Reader, type: SerdeType): Any = when (type) {
        SerdeType.Bool -> when (r.getU8()) {
            0 -> false
            1 -> true
            else -> throw SerdeException("invalid boolean byte")
        }
        SerdeType.I8 -> r.getU8().toByte()
        SerdeType.I16 -> r.getLe16().toShort()
        SerdeType.I32 -> r.getLe32()
        SerdeType.I64 -> r.getLe64()
        SerdeType.U8 -> r.getU8().toUByte()
        SerdeType.U16 -> r.getLe16().toUShort()
        SerdeType.U32 -> r.getLe32().toUInt()
        SerdeType.U64 -> r.getLe64().toULong()
        SerdeType.F32 -> Float.fromBits(r.getLe32())
        SerdeType.F64 -> Double.fromBits(r.getLe64())
        SerdeType.Str -> {
            val len = r.getVarIntAsInt()
            r.getBytes(len).decodeToString()
        }
        SerdeType.Bytes -> {
            val len = r.getVarIntAsInt()
            r.getBytes(len)
        }
        is SerdeType.ZList -> {
            val len = r.getVarIntAsInt()
            val out = ArrayList<Any>(len)
            for (i in 0 until len) out.add(readValue(r, type.elem))
            out
        }
        is SerdeType.ZMap -> {
            val len = r.getVarIntAsInt()
            val out = LinkedHashMap<Any, Any>(len)
            for (i in 0 until len) {
                val k = readValue(r, type.key)
                val v = readValue(r, type.value)
                out[k] = v
            }
            out
        }
        is SerdeType.ZPair -> Pair(readValue(r, type.first), readValue(r, type.second))
        is SerdeType.ZTriple ->
            Triple(readValue(r, type.first), readValue(r, type.second), readValue(r, type.third))
    }

    // ── Byte writer: little-endian primitives + LEB128 varints ────────────────

    private class Writer(initial: Int = 32) {
        private var buf = ByteArray(initial)
        private var len = 0

        private fun ensure(extra: Int) {
            if (len + extra > buf.size) {
                var n = buf.size * 2
                while (n < len + extra) n *= 2
                buf = buf.copyOf(n)
            }
        }

        fun putU8(v: Int) {
            ensure(1)
            buf[len++] = v.toByte()
        }

        fun putLe16(v: Int) {
            ensure(2)
            buf[len++] = v.toByte()
            buf[len++] = (v ushr 8).toByte()
        }

        fun putLe32(v: Int) {
            ensure(4)
            buf[len++] = v.toByte()
            buf[len++] = (v ushr 8).toByte()
            buf[len++] = (v ushr 16).toByte()
            buf[len++] = (v ushr 24).toByte()
        }

        fun putLe64(v: Long) {
            ensure(8)
            var x = v
            for (i in 0 until 8) {
                buf[len++] = x.toByte()
                x = x ushr 8
            }
        }

        fun putBytes(b: ByteArray) {
            ensure(b.size)
            b.copyInto(buf, len)
            len += b.size
        }

        /** LEB128 unsigned. `n` is a non-negative length (fits in u64). */
        fun putVarInt(n: Long) {
            var x = n
            while (true) {
                val byte = (x and 0x7F).toInt()
                x = x ushr 7
                if (x != 0L) {
                    putU8(byte or 0x80)
                } else {
                    putU8(byte)
                    break
                }
            }
        }

        fun toByteArray(): ByteArray = buf.copyOf(len)
    }

    // ── Byte reader ───────────────────────────────────────────────────────────

    private class Reader(private val buf: ByteArray) {
        private var pos = 0

        fun done(): Boolean = pos >= buf.size

        private fun need(n: Int) {
            if (pos + n > buf.size) throw SerdeException("unexpected end of input")
        }

        fun getU8(): Int {
            need(1)
            return buf[pos++].toInt() and 0xFF
        }

        fun getLe16(): Int {
            need(2)
            val v = (buf[pos].toInt() and 0xFF) or ((buf[pos + 1].toInt() and 0xFF) shl 8)
            pos += 2
            return v
        }

        fun getLe32(): Int {
            need(4)
            val v = (buf[pos].toInt() and 0xFF) or
                ((buf[pos + 1].toInt() and 0xFF) shl 8) or
                ((buf[pos + 2].toInt() and 0xFF) shl 16) or
                ((buf[pos + 3].toInt() and 0xFF) shl 24)
            pos += 4
            return v
        }

        fun getLe64(): Long {
            need(8)
            var v = 0L
            for (i in 0 until 8) {
                v = v or ((buf[pos + i].toLong() and 0xFF) shl (8 * i))
            }
            pos += 8
            return v
        }

        fun getBytes(n: Int): ByteArray {
            need(n)
            val out = buf.copyOfRange(pos, pos + n)
            pos += n
            return out
        }

        /** LEB128 unsigned. */
        fun getVarInt(): Long {
            var result = 0L
            var shift = 0
            while (true) {
                val byte = getU8()
                result = result or ((byte.toLong() and 0x7F) shl shift)
                if (byte and 0x80 == 0) break
                shift += 7
                if (shift >= 64) throw SerdeException("varint too long")
            }
            return result
        }

        fun getVarIntAsInt(): Int {
            val n = getVarInt()
            if (n < 0 || n > Int.MAX_VALUE) throw SerdeException("length out of range: $n")
            return n.toInt()
        }
    }
}
