//! Build script for the zenoh-flat JNI/Kotlin layer.
//!
//! Reads the `#[prebindgen]` items captured by `zenoh-flat` and drives them
//! through the [`prebindgen::lang::JniGen`] adapter to produce:
//!   * `src/generated_bindings.rs` — the Rust-side JNI wrappers (included by
//!     `src/lib.rs`), and
//!   * `kotlin/generated/**` — the matching typed Kotlin classes.
//!
//! ## Model
//!
//! This is the **flat tier**: every `#[prebindgen]` free function is declared
//! as a package function (`package!("x").fun(fun!(...))`) under a module
//! namespace, and each `PackageDecl` batch is handed to [`JniGen::package`].
//! Opaque handles stay typed Kotlin classes derived from `NativeHandle`
//! (locked, closeable) via `.class(ptr_class!(T))`.
//!
//! A type's default boundary shape is declared once, per direction, at the
//! generator level and AUTO-APPLIES to every matching param / return: input
//! variants via `.expand(expand_param!(T).variant(fun!(ctor)).variant_self())`
//! (an OR-list, runtime-selected) and output fields via
//! `.expand(expand_return!(T).field(fun!(acc)).field_self())` (an
//! AND-set, one crossing). A `.field(fun!(acc))` inherits its Kotlin name from
//! the class member declaration of the same fn. Class members are declared
//! with `.fun(fun!(f).name(..))` (instance methods) and
//! `.constructor(fun!(f).name(..))` (companion factories), and the per-fn
//! `.expand_param(name, …)` / `.expand_return(…)` overrides (chained on
//! the `fun!` decl, taking the same expand-decl objects) replace them — an
//! identity-only set (`.variant_self()` / `.field_self()` alone) is the plain
//! raw-handle form.
//!
//! Errors are delivered through the per-call `onError` callback (no Rust-side
//! JVM throw): `Error` (zenoh's native error) is the `E` of every fallible
//! `Result<_, Error>`, and its default return field (`error_get_message ->
//! String`) auto-applies to the `E` position so `onError` receives the message.
//!
//! The Rust source keeps its original `Z*` / `z_*` opaque-tier naming. JniGen's
//! type and function mangle hooks remove that FFI marker for Kotlin, while
//! per-member names describe only semantic differences (`id`, `toStr`, …).

use prebindgen::{
    core::Registry,
    enum_class, expand_param, expand_return, fun,
    lang::{ConstDecl, JniGen},
    package, ptr_class, value_class,
};
use syn::parse_quote as pq;

const ENCODING_NAMES: &[&str] = &[
    "zenoh_bytes",
    "zenoh_string",
    "zenoh_serialized",
    "application_octet_stream",
    "text_plain",
    "application_json",
    "text_json",
    "application_cdr",
    "application_cbor",
    "application_yaml",
    "text_yaml",
    "text_json5",
    "application_python_serialized_object",
    "application_protobuf",
    "application_java_serialized_object",
    "application_openmetrics_text",
    "image_png",
    "image_jpeg",
    "image_gif",
    "image_bmp",
    "image_webp",
    "application_xml",
    "application_x_www_form_urlencoded",
    "text_html",
    "text_xml",
    "text_css",
    "text_javascript",
    "text_markdown",
    "text_csv",
    "application_sql",
    "application_coap_payload",
    "application_json_patch_json",
    "application_json_seq",
    "application_jsonpath",
    "application_jwt",
    "application_mp4",
    "application_soap_xml",
    "application_yang",
    "audio_aac",
    "audio_flac",
    "audio_mp4",
    "audio_ogg",
    "audio_vorbis",
    "video_h261",
    "video_h263",
    "video_h264",
    "video_h265",
    "video_h266",
    "video_mp4",
    "video_ogg",
    "video_raw",
    "video_vp8",
    "video_vp9",
];

fn fail(context: &str, err: impl std::fmt::Display) -> ! {
    eprintln!("error: prebindgen jnigen {context}: {err}");
    std::process::exit(1);
}

/// The two expression constants for one predefined encoding, composed from
/// zenoh-flat's general accessors over its `z_encoding_<name>()` loaning
/// factory (no per-preset accessor exists in the source crate):
/// `ENCODING_<NAME>: String` (canonical form, upstream `Display`) and
/// `ENCODING_<NAME>_ID: Int` (numeric wire id).
fn encoding_consts(lower: &str) -> [ConstDecl; 2] {
    let upper = lower.to_uppercase();
    let factory: syn::Ident = syn::parse_str(&format!("z_encoding_{lower}")).unwrap();
    [
        ConstDecl::named(format!("ENCODING_{upper}"))
            .expr(pq!(String), pq!(z_encoding_to_string(#factory()))),
        ConstDecl::named(format!("ENCODING_{upper}_ID"))
            .expr(pq!(i32), pq!(z_encoding_id(#factory()))),
    ]
}

fn main() {
    // Declarations are plain values (`PackageDecl` / class decls / `fun!`
    // decls) handed to `JniGen::package` batch by batch — no typestate
    // cursor, so the long encoding-constant list below is an ordinary loop
    // over an ordinary `PtrClassDecl` binding.

    // ── Bytes: Encoding ───────────────────────────────────────────────
    // Canonical input: encoding params cross as their decomposed value
    // `(id: i32, schema: Option<String>)` (built via `encoding_new_from_id`)
    // — cheap primitives, no per-call String parse, no native handle.
    // Canonical output: the handle (identity) + id (raw jint), both free
    // jvalue slots; schema and the canonical string stay on-demand accessors.
    let encoding = ptr_class!(ZEncoding)
        .fun(fun!(z_encoding_id).name("id"))
        .fun(fun!(z_encoding_schema).name("getSchema"))
        .fun(fun!(z_encoding_to_string).name("toStr"))
        // Whole-handle clone return — see KeyExpr's `newClone`.
        .fun(
            fun!(z_encoding_clone)
                .name("newClone")
                .expand_return(expand_return!(ZEncoding).field_self()),
        )
        // `encoding_new_with_schema(&Encoding, schema) -> Encoding` derives a new
        // Encoding — a companion factory returning a raw handle (a constructor
        // never return-field-decomposes, so the result stays a usable handle rather than
        // a decomposed builder).
        .constructor(fun!(z_encoding_with_schema).name("withSchema"))
        // Factories → companion members; `fromId` is also the input variant
        // (see the `expand_param!(Encoding)` declaration below).
        .constructor(fun!(z_encoding_from_id).name("fromId"))
        .constructor(fun!(z_encoding_from_string).name("fromString"));

    // ── Bytes package: ZBytes + Encoding + predefined-encoding consts ────
    // ZBytes canonical input: `payload`/`attachment` params accept a `ByteArray`
    // (built via `zbytes_new_from_vec`). Canonical output: the handle only
    // (identity) — the bytes are heavy and fetched on demand via
    // `zbytesAsBytes` (one borrow-copy).
    //
    // Predefined encoding constants — each surfaces as a pair of
    // lazily-initialized top-level `val`s in the bytes package (pure JVM
    // values, no native handle), both **expression constants**
    // (`.expr`-sourced constants): this binding composes zenoh-flat's general
    // accessors over the `encoding_const_<name>()` loaning factory
    // (`encoding_to_string(...)` / `encoding_get_id(...)`), evaluated once
    // on first access — the source crate exposes each preset exactly once and
    // stores no decomposed values (see `encoding_consts` above). The loaning
    // factories themselves are not bound as Kotlin members (see the
    // `ignore` declaration below).
    let mut bytes = package!("bytes")
        .class(
            ptr_class!(ZZBytes)
                .fun(fun!(z_zbytes_to_bytes).name("asBytes"))
                // Whole-handle clone return — see KeyExpr's `newClone`.
                .fun(
                    fun!(z_zbytes_clone)
                        .name("newClone")
                        .expand_return(expand_return!(ZZBytes).field_self()),
                )
                // `fromVec` builds a ZBytes from a `ByteArray` — both the
                // param-variant build arm (see `expand_param!(ZBytes)` below)
                // AND a companion factory (a constructor never
                // return-field-decomposes, so the factory keeps its
                // raw-handle return).
                .constructor(fun!(z_zbytes_from_vec).name("fromVec")),
        )
        .class(encoding);
    for lower in ENCODING_NAMES {
        for decl in encoding_consts(lower) {
            bytes = bytes.constant(decl);
        }
    }

    let mut jni = JniGen::new()
        .set_package_prefix("io.zenoh.jni") // base package of the generated JNI bindings
        // Keep the Rust FFI tier visibly prefixed while presenting the same
        // concise Kotlin names (`ZSession` -> `Session`, `ZZBytes` -> `ZBytes`).
        .set_ptr_class_name_mangle(|name| name.strip_prefix('Z').unwrap_or(name).to_string())
        .set_data_class_name_mangle(|name| name.strip_prefix('Z').unwrap_or(name).to_string())
        // The default JNI name is already camel-cased. Remove a leading `z`
        // namespace marker and lowercase the next character (`zOpen` -> `open`,
        // `zSessionPut` -> `sessionPut`).
        .set_fun_name_mangle(|name| {
            let Some(rest) = name
                .strip_prefix('z')
                .filter(|rest| rest.chars().next().is_some_and(char::is_uppercase))
            else {
                return name.to_string();
            };
            let mut chars = rest.chars();
            let Some(first) = chars.next() else {
                return name.to_string();
            };
            first.to_lowercase().chain(chars).collect()
        })
        // Every generated native call routes through `JNINative`; trigger our own
        // loader from its static initializer so the native library is loaded
        // transparently before any extern resolves (consumers never load it).
        .set_jni_native_init("io.zenoh.jni.NativeLibrary.ensureLoaded()")
        // ── Errors ────────────────────────────────────────────────────────
        // `Error` is the `E` of every fallible `Result<_, Error>` — a
        // RUST-SIDE-ONLY type: no class declaration, so no Kotlin `Error`
        // class exists and the value never crosses. Its canonical output is
        // the message string (1 leaf), auto-applied to the `E` position of
        // every such `Result` — i.e. the `onError` callback's argument. The
        // field name is explicit (no class member to inherit from).
        .expand(expand_return!(Error).field(fun!(error_get_message).name("message")))
        // ── Key expressions ──────────────────────────────────────────────
        // Canonical input: a key-expr param accepts EITHER a String (built via
        // `keyexpr_new_try_from`) OR an existing handle (identity), selector-
        // dispatched. Canonical output: the handle only (identity, 1 raw jlong);
        // the string form stays an on-demand accessor method (`getStr`).
        .package(
            package!("keyexpr")
                .class(
                    ptr_class!(ZKeyExpr)
                        // Read accessors → instance methods on the KeyExpr class.
                        // `newClone` returns the borrowed clone as a WHOLE handle —
                        // override the class's default return fields (identity via
                        // the owned converter) so the borrowed-opaque clone path
                        // applies instead.
                        .fun(fun!(z_keyexpr_get_str))
                        .fun(
                            fun!(z_keyexpr_clone)
                                .name("newClone")
                                .expand_return(expand_return!(ZKeyExpr).field_self()),
                        )
                        .fun(fun!(z_keyexpr_to_string).name("toStr"))
                        // Constructors → companion factories returning `Result<KeyExpr, Error>`;
                        // `tryFrom` is also the build-from-String input variant
                        // (see `expand_param!(KeyExpr)` below).
                        .constructor(fun!(z_keyexpr_try_from).name("tryFrom"))
                        .constructor(fun!(z_keyexpr_autocanonize).name("autocanonize"))
                        .constructor(fun!(z_keyexpr_join).name("join").split_on_param("a"))
                        .constructor(fun!(z_keyexpr_concat).name("concat").split_on_param("a"))
                        // Consumer methods: the receiver key-expr is `this`; the other
                        // param accepts a String (built via the default param variants below).
                        .fun(fun!(z_keyexpr_intersects).split_on_param("b"))
                        .fun(fun!(z_keyexpr_includes).split_on_param("b"))
                        .fun(fun!(z_keyexpr_relation_to).split_on_param("b")),
                )
                .class(enum_class!(SetIntersectionLevel)),
        )
        // Default param variants: a key-expr param accepts EITHER a String (built
        // via `tryFrom`) OR an existing handle (self). Default return field: the
        // handle only (the string form stays the `getStr` accessor method).
        .expand(
            expand_param!(ZKeyExpr)
                .variant(fun!(z_keyexpr_try_from))
                .variant_self(),
        )
        .expand(expand_return!(ZKeyExpr).field_self())
        // ── Config + ZenohId ──────────────────────────────────────────────
        .package(
            package!("config")
                .class(
                    ptr_class!(ZConfig)
                        .fun(fun!(z_config_get_json))
                        .fun(fun!(z_config_clone).name("newClone"))
                        // Factories → Config companion-object members.
                        .constructor(fun!(z_config_default).name("newDefault"))
                        .constructor(fun!(z_config_from_file).name("fromFile"))
                        .constructor(fun!(z_config_from_json).name("fromJson"))
                        .constructor(fun!(z_config_from_json5).name("fromJson5"))
                        .constructor(fun!(z_config_from_yaml).name("fromYaml")),
                )
                .fun(fun!(z_config_insert_json5).name("configInsertJson5"))
                .class(enum_class!(WhatAmI))
                // `ZenohId` is a `Copy` value (zenoh's `ZenohId`, repr(transparent)), so
                // it crosses as a raw byte-blob `ByteArray` rather than a closeable jlong
                // handle. `Vec<ZenohId>` (session peers/routers) folds each element WHOLE
                // as the typed `ZenohId` value class. Its read accessors become methods
                // on the value class (receiver = `this.bytes`).
                .class(
                    value_class!(ZZenohId)
                        .fun(fun!(z_zenoh_id_to_bytes))
                        .fun(fun!(z_zenoh_id_to_string).name("toStr")),
                ),
        )
        // ── Scouting ──────────────────────────────────────────────────────
        // Canonical output: the scout callback decomposes a `Hello` into its
        // three read fields in ONE crossing (no handle — read-only). Auto-applies
        // to `scout`'s `Fn(Hello)`.
        .package(
            package!("scouting")
                .class(
                    ptr_class!(ZHello)
                        .fun(fun!(z_hello_whatami).name("whatami")) // WhatAmI enum -> Int
                        .fun(fun!(z_hello_zid).name("zid")) // ZenohId value class -> ByteArray
                        .fun(fun!(z_hello_locators).name("locators")), // Vec<String> -> List<String>
                )
                .class(ptr_class!(ZScout))
                .fun(fun!(z_scout).name("scout")),
        )
        .expand(
            expand_return!(ZHello)
                .field(fun!(z_hello_whatami))
                .field(fun!(z_hello_zid))
                .field(fun!(z_hello_locators)),
        )
        // ── Logger ────────────────────────────────────────────────────────
        .package(
            package!("logger")
                .fun(fun!(init_android_logs))
                .fun(fun!(try_init_zenoh_logs_from_env))
                .fun(fun!(init_zenoh_logs_from_env_or)),
        )
        // ── QoS enums ─────────────────────────────────────────────────────
        .package(
            package!("qos")
                .class(enum_class!(Reliability))
                .class(enum_class!(Priority))
                .class(enum_class!(CongestionControl)),
        )
        // ── Bytes: ZBytes + Encoding (declared above) ─────────────────────
        .package(bytes)
        // ZBytes default input: built from a `ByteArray` via `fromVec`.
        // Default output: the handle only — the bytes are heavy and fetched
        // on demand via `zbytesAsBytes` (one borrow-copy).
        .expand(expand_param!(ZZBytes).variant(fun!(z_zbytes_from_vec)))
        .expand(expand_return!(ZZBytes).field_self())
        // Encoding default input: the decomposed value `(id, schema?)` (built
        // via `fromId`) — cheap primitives, no per-call String parse. Default
        // output: the handle + its id (both free jvalue slots); schema and
        // the canonical string stay on-demand accessors.
        .expand(expand_param!(ZEncoding).variant(fun!(z_encoding_from_id)))
        .expand(
            expand_return!(ZEncoding)
                .field_self()
                .field(fun!(z_encoding_id)),
        )
        // ── Time ──────────────────────────────────────────────────────────
        // Canonical output: a timestamp is its NTP64 value (`timestamp_get_ntp64`
        // -> i64 -> Long, 1 leaf); nested in a `Sample` it contributes that Long.
        .package(
            package!("time").class(
                ptr_class!(ZTimestamp)
                    .fun(fun!(z_timestamp_ntp64).name("ntp64"))
                    .fun(fun!(z_timestamp_id).name("getId")),
            ),
        )
        .expand(expand_return!(ZTimestamp).field(fun!(z_timestamp_ntp64)))
        // ── Sample ────────────────────────────────────────────────────────
        // Canonical INPUT: identity only — a `Sample` param takes the owned
        // handle directly. (The full-options constructors carry `Option<ptr_class>`
        // params the recursive-input fold can't build through, so a `Sample` is
        // built via the `sample_new_*` constructors below and consumed by handle.)
        // Canonical OUTPUT: the full sample decomposed in ONE crossing. Each record
        // is unwrapped per its return type's own canonical output (key_expr ->
        // handle+String, payload/attachment -> ByteArray, encoding -> String,
        // timestamp -> Long?, kind/priority/congestion/reliability -> Int, express
        // -> Boolean, source_zid -> ByteArray?, source_eid -> Int, source_sn ->
        // Long). Auto-applies to every (non-Result) `Sample` return.
        .package(
            package!("sample").class(enum_class!(SampleKind)).class(
                ptr_class!(ZSample)
                    // All sample getters are record sources AND instance methods on
                    // the Sample class; decomposition happens via the canonical
                    // output below.
                    .fun(fun!(z_sample_key_expr).name("keyExpr"))
                    .fun(fun!(z_sample_payload).name("payload"))
                    .fun(fun!(z_sample_encoding).name("encoding"))
                    .fun(fun!(z_sample_kind).name("kind"))
                    .fun(fun!(z_sample_timestamp).name("timestamp"))
                    .fun(fun!(z_sample_express).name("express"))
                    .fun(fun!(z_sample_priority).name("priority"))
                    .fun(fun!(z_sample_congestion_control).name("congestionControl"))
                    .fun(fun!(z_sample_attachment).name("attachment"))
                    .fun(fun!(z_sample_reliability).name("reliability"))
                    .fun(fun!(z_sample_source_zid).name("sourceZid"))
                    .fun(fun!(z_sample_source_eid).name("sourceEid"))
                    .fun(fun!(z_sample_source_sn).name("sourceSn")),
            ),
        )
        // Identity-only input: exactly the default (documented no-op).
        .expand(expand_param!(ZSample).variant_self())
        // Full-sample decomposition; field names inherit from the members above.
        .expand(
            expand_return!(ZSample)
                .field(fun!(z_sample_key_expr))
                .field(fun!(z_sample_payload))
                .field(fun!(z_sample_encoding))
                .field(fun!(z_sample_kind))
                .field(fun!(z_sample_timestamp))
                .field(fun!(z_sample_express))
                .field(fun!(z_sample_priority))
                .field(fun!(z_sample_congestion_control))
                .field(fun!(z_sample_attachment))
                .field(fun!(z_sample_reliability))
                .field(fun!(z_sample_source_zid))
                .field(fun!(z_sample_source_eid))
                .field(fun!(z_sample_source_sn)),
        )
        // ── Pub/Sub ───────────────────────────────────────────────────────
        // key_expr / payload / attachment / encoding params are auto-constructed
        // by their types' canonical inputs (no per-fn calls).
        .package(
            package!("pubsub")
                .class(ptr_class!(ZPublisher))
                .fun(fun!(z_publisher_put).name("publisherPut"))
                .fun(fun!(z_publisher_delete).name("publisherDelete"))
                .class(ptr_class!(ZSubscriber)),
        )
        // ── Query / Queryable / Querier ───────────────────────────────────
        .package(
            package!("query")
                .class(ptr_class!(ZQueryable))
                .class(ptr_class!(ZQuerier))
                .fun(fun!(z_querier_get).name("querierGet"))
                .class(enum_class!(ReplyKeyExpr))
                .class(enum_class!(QueryTarget))
                .class(enum_class!(ConsolidationMode))
                .class(
                    ptr_class!(ZQuery)
                        .fun(fun!(z_query_keyexpr).name("keyExpr"))
                        .fun(fun!(z_query_parameters).name("parameters"))
                        .fun(fun!(z_query_payload).name("payload"))
                        .fun(fun!(z_query_encoding).name("encoding"))
                        .fun(fun!(z_query_attachment).name("attachment"))
                        .fun(fun!(z_query_accepts_replies).name("acceptsReplies")),
                )
                // Reply ops on the owned/borrowed query handle.
                .fun(
                    fun!(z_query_reply_success)
                        .name("queryReplySuccess")
                        .split_on_param("key_expr"),
                )
                .fun(fun!(z_query_reply_error).name("queryReplyError"))
                .fun(
                    fun!(z_query_reply_delete)
                        .name("queryReplyDelete")
                        .split_on_param("key_expr"),
                )
                // ── Reply ─────────────────────────────────────────────────────
                .class(
                    ptr_class!(ZReply)
                        // Record sources are class methods — `reply.sample()`'s
                        // standalone export is therefore the cloned-handle form.
                        .fun(fun!(z_reply_replier_zid).name("replierZid"))
                        .fun(fun!(z_reply_replier_eid).name("replierEid"))
                        .fun(fun!(z_reply_is_ok))
                        .fun(fun!(z_reply_sample).name("sample"))
                        .fun(fun!(z_reply_error_payload).name("errorPayload"))
                        .fun(fun!(z_reply_error_encoding).name("errorEncoding")),
                ),
        )
        // Canonical output: the queryable callback decomposes a `Query` into
        // BOTH its read fields AND the owned handle (identity) in ONE crossing
        // — keeping the handle lets the consumer reply (`query_reply_*`) after
        // the callback returns; a query must outlive its callback to be
        // answered. `.field_self()` is declared LAST: the root identity moves
        // the owned query while the nested KeyExpr identity (from
        // `query_get_keyexpr`) clones from a borrow of it — the generator
        // hard-errors on the reverse order.
        .expand(
            expand_return!(ZQuery)
                .field(fun!(z_query_keyexpr))
                .field(fun!(z_query_parameters))
                .field(fun!(z_query_payload))
                .field(fun!(z_query_encoding))
                .field(fun!(z_query_attachment))
                .field(fun!(z_query_accepts_replies))
                .field_self(),
        )
        // Reply canonical output: the whole reply decomposed in ONE crossing
        // (PRODUCT model — both arms' leaves always present, the not-taken
        // arm's are null). replier zid/eid + the is_ok discriminator, then the
        // ok arm splices the full sample and the err arm splices
        // payload/encoding. Auto-applies to the `Fn(Reply)` callbacks of
        // `session_get` / `querier_get` / liveliness get; no identity record,
        // so no `Reply` handle crosses.
        .expand(
            expand_return!(ZReply)
                .field(fun!(z_reply_replier_zid))
                .field(fun!(z_reply_replier_eid))
                .field(fun!(z_reply_is_ok))
                .field(fun!(z_reply_sample))
                .field(fun!(z_reply_error_payload))
                .field(fun!(z_reply_error_encoding)),
        )
        // ── Liveliness + Session ──────────────────────────────────────────
        // `LivelinessToken` is just an opaque handle; the liveliness operations
        // (`liveliness_*`) are declared under the `session` package below,
        // alongside the session API they extend.
        .package(package!("liveliness").class(ptr_class!(ZLivelinessToken)))
        .package(
            package!("session")
                .class(ptr_class!(ZSession).fun(fun!(z_session_zid).name("getZid")))
                .fun(fun!(z_open).name("open"))
                .fun(
                    fun!(z_session_declare_publisher)
                        .name("sessionDeclarePublisher")
                        .split_on_param("key_expr"),
                )
                .fun(
                    fun!(z_session_put)
                        .name("sessionPut")
                        .split_on_param("key_expr"),
                )
                .fun(
                    fun!(z_session_delete)
                        .name("sessionDelete")
                        .split_on_param("key_expr"),
                )
                .fun(
                    fun!(z_session_declare_subscriber)
                        .name("sessionDeclareSubscriber")
                        .split_on_param("key_expr"),
                )
                .fun(
                    fun!(z_session_declare_querier)
                        .name("sessionDeclareQuerier")
                        .split_on_param("key_expr"),
                )
                .fun(
                    fun!(z_session_declare_queryable)
                        .name("sessionDeclareQueryable")
                        .split_on_param("key_expr"),
                )
                .fun(fun!(z_session_declare_keyexpr).name("sessionDeclareKeyexpr"))
                // Undeclaring needs the declared handle, not a string — opt its
                // key_expr param out of the (String-building) default param variants.
                .fun(
                    fun!(z_session_undeclare_keyexpr)
                        .name("sessionUndeclareKeyexpr")
                        .expand_param("key_expr", expand_param!(ZKeyExpr).variant_self()),
                )
                .fun(
                    fun!(z_session_get)
                        .name("sessionGet")
                        .split_on_param("key_expr"),
                )
                // `Vec<ZenohId>`: ZenohId is a value class, so these return
                // `List<ZenohId>` via the normal Vec converter.
                .fun(fun!(z_session_peers_zid).name("sessionGetPeersZid"))
                .fun(fun!(z_session_routers_zid).name("sessionGetRoutersZid"))
                // Liveliness ops (key_expr params auto-constructed by the canonical input).
                .fun(
                    fun!(z_liveliness_declare_token)
                        .name("livelinessDeclareToken")
                        .split_on_param("key_expr"),
                )
                .fun(
                    fun!(z_liveliness_get)
                        .name("livelinessGet")
                        .split_on_param("key_expr"),
                )
                .fun(
                    fun!(z_liveliness_declare_subscriber)
                        .name("livelinessDeclareSubscriber")
                        .split_on_param("key_expr"),
                ),
        );

    // zenoh-flat's `z_encoding_*` `&'static ZEncoding` loaning factories
    // are superseded here by the `ENCODING_*` consts above — acknowledge the
    // whole naming family so the generator doesn't warn about undeclared
    // functions.
    jni = jni.ignore(prebindgen::matching(|name| {
        name.strip_prefix("z_encoding_")
            .is_some_and(|suffix| ENCODING_NAMES.contains(&suffix))
    }));

    // The source crate deliberately retains its higher-level `expanded` tier
    // for Rust callers. JNI binds the original opaque `z_*` tier instead, so
    // acknowledge the expanded twins as a family while keeping newly-added
    // opaque functions visible as generation warnings.
    jni = jni.ignore(prebindgen::matching(|name| {
        [
            "encoding_",
            "keyexpr_",
            "liveliness_",
            "publisher_",
            "querier_",
            "query_",
            "reply_",
            "session_",
        ]
        .iter()
        .any(|prefix| name.starts_with(prefix))
            || matches!(
                name,
                "Encoding"
                    | "Hello"
                    | "KeyExpr"
                    | "Query"
                    | "Reply"
                    | "Sample"
                    | "Timestamp"
                    | "ZBytes"
                    | "ZenohId"
                    | "hello_zid"
                    | "scout"
                    | "zenoh_id_to_string"
                    | "z_query_expand"
                    | "z_reply_expand"
                    | "z_sample_expand"
                    | "z_timestamp_expand"
                    | "z_zbytes_from_slice"
            )
    }));

    // ── Outputs ───────────────────────────────────────────────────────────
    // Run the configured adapter over zenoh-flat's captured `#[prebindgen]`
    // items and write both generated artifacts.
    let source = prebindgen::Source::new(zenoh_flat::PREBINDGEN_OUT_DIR);
    let registry = match Registry::from_items(source.items_all()) {
        Ok(registry) => registry,
        Err(err) => fail("scan failed", err),
    };

    // Rust bindings → src/generated_bindings.rs. Absolute path so the file lands
    // in the source tree (committed to git and included by `src/lib.rs`).
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let rust_dest = std::path::Path::new(&crate_dir)
        .join("src")
        .join("generated_bindings.rs");
    let generation = match registry.resolve(jni) {
        Ok(generation) => generation,
        Err(err) => fail("resolve failed", err),
    };
    let rust_path = match generation.write_rust(&rust_dest) {
        Ok(path) => path,
        Err(err) => fail("write_rust failed", err),
    };
    println!(
        "cargo:warning=Generated bindings at: {}",
        rust_path.display()
    );

    // ── Kotlin bindings → kotlin/generated/ ─────────────────────────────
    // The runtime module's Gradle source set picks these up via
    // `kotlin.srcDir("$rootDir/zenoh-flat-jni/kotlin/generated")`.
    let kotlin_root = std::path::Path::new(&crate_dir)
        .join("kotlin")
        .join("generated");
    // The root is generator-owned: `write_kotlin` deletes and recreates it,
    // so no consumer-side cleanup is needed.
    for path in match generation.write_kotlin(&kotlin_root) {
        Ok(paths) => paths,
        Err(err) => fail("write_kotlin failed", err),
    } {
        println!("cargo:warning=Wrote {}", path.display());
    }

    // The resolved-surface report: committed next to the regen so a decl's
    // effect is reviewable without reading generated Kotlin.
    if let Err(err) = std::fs::write(
        std::path::Path::new(&crate_dir)
            .join("kotlin")
            .join("REPORT.md"),
        generation.report(),
    ) {
        fail("write REPORT.md failed", err);
    }
}
