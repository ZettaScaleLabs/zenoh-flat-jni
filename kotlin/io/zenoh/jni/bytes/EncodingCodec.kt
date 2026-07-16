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

package io.zenoh.jni.bytes

/**
 * Pure-JVM encoding conversions, shared by every SDK built on these bindings
 * (zenoh-java, zenoh-kotlin).
 *
 * An encoding **is** its decomposed pair `(id, schema)` — exactly Zenoh's own
 * representation; the textual form (e.g. `"text/plain;utf-8"`) is derived from
 * a fixed id↔name table. The table data comes from the generated `ENCODING_*`
 * constants in this package (regenerated from Zenoh — single source of truth),
 * and the conversion rules below mirror Zenoh core (`zenoh/src/api/encoding.rs`)
 * exactly; the correspondence is verified against the native implementation by
 * the consuming SDK's test suite (`EncodingCorrespondenceTest` in zenoh-java).
 * No conversion here ever crosses the JNI boundary.
 */
object EncodingCodec {

    /**
     * Mirror of Zenoh's `CUSTOM_ENCODING_ID`: a custom (non-predefined)
     * encoding carries its whole textual form in the schema slot.
     */
    const val CUSTOM_ENCODING_ID: Int = 0xFFFF

    /**
     * The id→name table, from the generated constants plus the custom-id row
     * Zenoh keys with the empty name (render-side only).
     */
    val idToName: Map<Int, String> = mapOf(
        ENCODING_ZENOH_BYTES_ID to ENCODING_ZENOH_BYTES,
        ENCODING_ZENOH_STRING_ID to ENCODING_ZENOH_STRING,
        ENCODING_ZENOH_SERIALIZED_ID to ENCODING_ZENOH_SERIALIZED,
        ENCODING_APPLICATION_OCTET_STREAM_ID to ENCODING_APPLICATION_OCTET_STREAM,
        ENCODING_TEXT_PLAIN_ID to ENCODING_TEXT_PLAIN,
        ENCODING_APPLICATION_JSON_ID to ENCODING_APPLICATION_JSON,
        ENCODING_TEXT_JSON_ID to ENCODING_TEXT_JSON,
        ENCODING_APPLICATION_CDR_ID to ENCODING_APPLICATION_CDR,
        ENCODING_APPLICATION_CBOR_ID to ENCODING_APPLICATION_CBOR,
        ENCODING_APPLICATION_YAML_ID to ENCODING_APPLICATION_YAML,
        ENCODING_TEXT_YAML_ID to ENCODING_TEXT_YAML,
        ENCODING_TEXT_JSON5_ID to ENCODING_TEXT_JSON5,
        ENCODING_APPLICATION_PYTHON_SERIALIZED_OBJECT_ID to ENCODING_APPLICATION_PYTHON_SERIALIZED_OBJECT,
        ENCODING_APPLICATION_PROTOBUF_ID to ENCODING_APPLICATION_PROTOBUF,
        ENCODING_APPLICATION_JAVA_SERIALIZED_OBJECT_ID to ENCODING_APPLICATION_JAVA_SERIALIZED_OBJECT,
        ENCODING_APPLICATION_OPENMETRICS_TEXT_ID to ENCODING_APPLICATION_OPENMETRICS_TEXT,
        ENCODING_IMAGE_PNG_ID to ENCODING_IMAGE_PNG,
        ENCODING_IMAGE_JPEG_ID to ENCODING_IMAGE_JPEG,
        ENCODING_IMAGE_GIF_ID to ENCODING_IMAGE_GIF,
        ENCODING_IMAGE_BMP_ID to ENCODING_IMAGE_BMP,
        ENCODING_IMAGE_WEBP_ID to ENCODING_IMAGE_WEBP,
        ENCODING_APPLICATION_XML_ID to ENCODING_APPLICATION_XML,
        ENCODING_APPLICATION_X_WWW_FORM_URLENCODED_ID to ENCODING_APPLICATION_X_WWW_FORM_URLENCODED,
        ENCODING_TEXT_HTML_ID to ENCODING_TEXT_HTML,
        ENCODING_TEXT_XML_ID to ENCODING_TEXT_XML,
        ENCODING_TEXT_CSS_ID to ENCODING_TEXT_CSS,
        ENCODING_TEXT_JAVASCRIPT_ID to ENCODING_TEXT_JAVASCRIPT,
        ENCODING_TEXT_MARKDOWN_ID to ENCODING_TEXT_MARKDOWN,
        ENCODING_TEXT_CSV_ID to ENCODING_TEXT_CSV,
        ENCODING_APPLICATION_SQL_ID to ENCODING_APPLICATION_SQL,
        ENCODING_APPLICATION_COAP_PAYLOAD_ID to ENCODING_APPLICATION_COAP_PAYLOAD,
        ENCODING_APPLICATION_JSON_PATCH_JSON_ID to ENCODING_APPLICATION_JSON_PATCH_JSON,
        ENCODING_APPLICATION_JSON_SEQ_ID to ENCODING_APPLICATION_JSON_SEQ,
        ENCODING_APPLICATION_JSONPATH_ID to ENCODING_APPLICATION_JSONPATH,
        ENCODING_APPLICATION_JWT_ID to ENCODING_APPLICATION_JWT,
        ENCODING_APPLICATION_MP4_ID to ENCODING_APPLICATION_MP4,
        ENCODING_APPLICATION_SOAP_XML_ID to ENCODING_APPLICATION_SOAP_XML,
        ENCODING_APPLICATION_YANG_ID to ENCODING_APPLICATION_YANG,
        ENCODING_AUDIO_AAC_ID to ENCODING_AUDIO_AAC,
        ENCODING_AUDIO_FLAC_ID to ENCODING_AUDIO_FLAC,
        ENCODING_AUDIO_MP4_ID to ENCODING_AUDIO_MP4,
        ENCODING_AUDIO_OGG_ID to ENCODING_AUDIO_OGG,
        ENCODING_AUDIO_VORBIS_ID to ENCODING_AUDIO_VORBIS,
        ENCODING_VIDEO_H261_ID to ENCODING_VIDEO_H261,
        ENCODING_VIDEO_H263_ID to ENCODING_VIDEO_H263,
        ENCODING_VIDEO_H264_ID to ENCODING_VIDEO_H264,
        ENCODING_VIDEO_H265_ID to ENCODING_VIDEO_H265,
        ENCODING_VIDEO_H266_ID to ENCODING_VIDEO_H266,
        ENCODING_VIDEO_MP4_ID to ENCODING_VIDEO_MP4,
        ENCODING_VIDEO_OGG_ID to ENCODING_VIDEO_OGG,
        ENCODING_VIDEO_RAW_ID to ENCODING_VIDEO_RAW,
        ENCODING_VIDEO_VP8_ID to ENCODING_VIDEO_VP8,
        ENCODING_VIDEO_VP9_ID to ENCODING_VIDEO_VP9,
        CUSTOM_ENCODING_ID to "",
    )

    /**
     * Parse-side reverse table. Zenoh's `STR_TO_ID` has NO empty-string row
     * (the `CUSTOM ↔ ""` mapping is render-side only), so a leading `;` input
     * falls through to the custom arm with the whole string as schema.
     */
    private val nameToId: Map<String, Int> = idToName.entries
        .filter { (_, name) -> name.isNotEmpty() }
        .associate { (id, name) -> name to id }

    /**
     * Parse a textual encoding into its `(id, schema)` pair, per Zenoh's rule:
     * empty → `(0, null)`; everything before the first `;` is looked up as a
     * known name — a match yields `(id, rest-if-nonempty)`, a miss yields the
     * custom id with the whole string as schema.
     */
    fun parse(s: String): Pair<Int, String?> {
        if (s.isEmpty()) return 0 to null
        val sep = s.indexOf(';')
        val name = if (sep >= 0) s.substring(0, sep) else s
        val rest = if (sep >= 0) s.substring(sep + 1) else ""
        val id = nameToId[name]
        return if (id != null) {
            id to rest.takeIf { it.isNotEmpty() }
        } else {
            CUSTOM_ENCODING_ID to s
        }
    }

    /**
     * Canonical textual form of `(id, schema)`, per Zenoh's rendering rule:
     * the known name, `"name;schema"`, the bare schema for a custom encoding,
     * or `"unknown(id)"` for an unrecognized id.
     */
    fun render(id: Int, schema: String?): String {
        val name = idToName[id]
        return when {
            name == null -> if (schema == null) "unknown($id)" else "unknown($id);$schema"
            schema == null -> name
            name.isEmpty() -> schema
            else -> "$name;$schema"
        }
    }

    /**
     * Zenoh's `with_schema` rule applied to `(id, schema)`. For a custom
     * encoding the schema slot holds `"name[;schema]"`; the name part is
     * preserved and only the schema part is replaced.
     */
    fun withSchema(id: Int, schema: String?, newSchema: String): Pair<Int, String?> {
        if (id != CUSTOM_ENCODING_ID) return id to newSchema
        val name = schema?.substringBefore(';') ?: ""
        return id to when {
            name.isEmpty() -> newSchema
            newSchema.isEmpty() -> name
            else -> "$name;$newSchema"
        }
    }
}
