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
//! This is the **flat tier**: the Rust source remains ordinary prefixed free
//! functions for C compatibility, while JniGen places receiver-shaped calls
//! into typed Kotlin classes with `.method(...)` and factories with
//! `.constructor(...)`. The package/class-aware name hooks remove structural
//! `z_<namespace>_` prefixes; only semantic renames need `.name(...)`.
//!
//! A type's default boundary shape is declared once, per direction, at the
//! generator level and AUTO-APPLIES to every matching param / return: input
//! variants via `.expand(expand_param!(T).variant(fun!(ctor)).variant_self())`
//! (an OR-list, runtime-selected) and output fields via
//! `.expand(expand_return!(T).field(fun!(acc)).field_self())` (an
//! AND-set, one crossing). A `.field(fun!(acc))` inherits its Kotlin name from
//! the class member declaration of the same fn. Class members are declared
//! with `.method(fun!(f).name(..))` (instance methods) and
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
//! package-aware type/function hooks and the package/class-aware method hook
//! remove that FFI marker for Kotlin. This changes no zenoh-flat symbol, so
//! zenoh-flat-c keeps consuming the same stable `z_*` surface.

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

fn lower_first(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// Remove the leading `z` FFI marker from a camelCased Rust function name.
fn strip_z_marker(name: &str) -> String {
    name.strip_prefix('z')
        .filter(|rest| rest.chars().next().is_some_and(char::is_uppercase))
        .map(lower_first)
        .unwrap_or_else(|| name.to_string())
}

fn strip_context_prefix(context: &str, name: &str) -> String {
    if name
        .get(..context.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(context))
    {
        let rest = &name[context.len()..];
        if !rest.is_empty() {
            return lower_first(rest);
        }
    }
    name.to_string()
}

fn mangle_flat_function(package: &str, name: &str) -> String {
    let unprefixed = strip_z_marker(name);
    let namespace = package.rsplit('.').next().unwrap_or(package);
    strip_context_prefix(namespace, &unprefixed)
}

fn mangle_flat_method(_package: &str, class: &str, name: &str) -> String {
    let unprefixed = strip_z_marker(name);
    if class == "JNINative" {
        unprefixed
    } else {
        strip_context_prefix(class, &unprefixed)
    }
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
        .method(fun!(z_encoding_id))
        .method(fun!(z_encoding_schema).name("getSchema"))
        .method(fun!(z_encoding_to_string).name("toStr"))
        // Whole-handle clone return — see KeyExpr's `newClone`.
        .method(
            fun!(z_encoding_clone)
                .name("newClone")
                .expand_return(expand_return!(ZEncoding).field_self()),
        )
        // `encoding_new_with_schema(&Encoding, schema) -> Encoding` derives a new
        // Encoding — a companion factory returning a raw handle (a constructor
        // never return-field-decomposes, so the result stays a usable handle rather than
        // a decomposed builder).
        .constructor(fun!(z_encoding_with_schema))
        // Factories → companion members; `fromId` is also the input variant
        // (see the `expand_param!(Encoding)` declaration below).
        .constructor(fun!(z_encoding_from_id))
        .constructor(fun!(z_encoding_from_string));

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
                .method(fun!(z_zbytes_to_bytes).name("asBytes"))
                // Whole-handle clone return — see KeyExpr's `newClone`.
                .method(
                    fun!(z_zbytes_clone)
                        .name("newClone")
                        .expand_return(expand_return!(ZZBytes).field_self()),
                )
                // `fromVec` builds a ZBytes from a `ByteArray` — both the
                // param-variant build arm (see `expand_param!(ZBytes)` below)
                // AND a companion factory (a constructor never
                // return-field-decomposes, so the factory keeps its
                // raw-handle return).
                .constructor(fun!(z_zbytes_from_vec)),
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
        .set_ptr_class_name_mangle(|_, name| name.strip_prefix('Z').unwrap_or(name).to_string())
        .set_data_class_name_mangle(|_, name| name.strip_prefix('Z').unwrap_or(name).to_string())
        .set_fun_name_mangle(mangle_flat_function)
        .set_method_name_mangle(mangle_flat_method)
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
                        .method(fun!(z_keyexpr_get_str))
                        .method(
                            fun!(z_keyexpr_clone)
                                .name("newClone")
                                .expand_return(expand_return!(ZKeyExpr).field_self()),
                        )
                        .method(fun!(z_keyexpr_to_string).name("toStr"))
                        // Constructors → companion factories returning `Result<KeyExpr, Error>`;
                        // `tryFrom` is also the build-from-String input variant
                        // (see `expand_param!(KeyExpr)` below).
                        .constructor(fun!(z_keyexpr_try_from))
                        .constructor(fun!(z_keyexpr_autocanonize))
                        .constructor(fun!(z_keyexpr_join).split_on_param("a"))
                        .constructor(fun!(z_keyexpr_concat).split_on_param("a"))
                        // Consumer methods: the receiver key-expr is `this`; the other
                        // param accepts a String (built via the default param variants below).
                        .method(fun!(z_keyexpr_intersects).split_on_param("b"))
                        .method(fun!(z_keyexpr_includes).split_on_param("b"))
                        .method(fun!(z_keyexpr_relation_to).split_on_param("b")),
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
                        .method(fun!(z_config_get_json))
                        .method(fun!(z_config_clone).name("newClone"))
                        .method(fun!(z_config_insert_json5))
                        // Factories → Config companion-object members.
                        .constructor(fun!(z_config_default).name("newDefault"))
                        .constructor(fun!(z_config_from_file))
                        .constructor(fun!(z_config_from_json))
                        .constructor(fun!(z_config_from_json5))
                        .constructor(fun!(z_config_from_yaml)),
                )
                .class(enum_class!(WhatAmI))
                // `ZenohId` is a `Copy` value (zenoh's `ZenohId`, repr(transparent)), so
                // it crosses as a raw byte-blob `ByteArray` rather than a closeable jlong
                // handle. `Vec<ZenohId>` (session peers/routers) folds each element WHOLE
                // as the typed `ZenohId` value class. Its read accessors become methods
                // on the value class (receiver = `this.bytes`).
                .class(
                    value_class!(ZZenohId)
                        .method(fun!(z_zenoh_id_to_bytes))
                        .method(fun!(z_zenoh_id_to_string).name("toStr")),
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
                        .method(fun!(z_hello_whatami)) // WhatAmI enum -> Int
                        .method(fun!(z_hello_zid)) // ZenohId value class -> ByteArray
                        .method(fun!(z_hello_locators)), // Vec<String> -> List<String>
                )
                .class(ptr_class!(ZScout))
                .fun(fun!(z_scout)),
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
                    .method(fun!(z_timestamp_ntp64))
                    .method(fun!(z_timestamp_id).name("getId")),
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
                    .method(fun!(z_sample_key_expr))
                    .method(fun!(z_sample_payload))
                    .method(fun!(z_sample_encoding))
                    .method(fun!(z_sample_kind))
                    .method(fun!(z_sample_timestamp))
                    .method(fun!(z_sample_express))
                    .method(fun!(z_sample_priority))
                    .method(fun!(z_sample_congestion_control))
                    .method(fun!(z_sample_attachment))
                    .method(fun!(z_sample_reliability))
                    .method(fun!(z_sample_source_zid))
                    .method(fun!(z_sample_source_eid))
                    .method(fun!(z_sample_source_sn)),
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
                .class(
                    ptr_class!(ZPublisher)
                        .method(fun!(z_publisher_put))
                        .method(fun!(z_publisher_delete)),
                )
                .class(ptr_class!(ZSubscriber)),
        )
        // ── Query / Queryable / Querier ───────────────────────────────────
        .package(
            package!("query")
                .class(ptr_class!(ZQueryable))
                .class(ptr_class!(ZQuerier).method(fun!(z_querier_get)))
                .class(enum_class!(ReplyKeyExpr))
                .class(enum_class!(QueryTarget))
                .class(enum_class!(ConsolidationMode))
                .class(
                    ptr_class!(ZQuery)
                        .method(fun!(z_query_keyexpr))
                        .method(fun!(z_query_parameters))
                        .method(fun!(z_query_payload))
                        .method(fun!(z_query_encoding))
                        .method(fun!(z_query_attachment))
                        .method(fun!(z_query_accepts_replies))
                        // Reply ops on the owned/borrowed query handle.
                        .method(fun!(z_query_reply_success).split_on_param("key_expr"))
                        .method(fun!(z_query_reply_error))
                        .method(fun!(z_query_reply_delete).split_on_param("key_expr")),
                )
                // ── Reply ─────────────────────────────────────────────────────
                .class(
                    ptr_class!(ZReply)
                        // Record sources are class methods — `reply.sample()`'s
                        // standalone export is therefore the cloned-handle form.
                        .method(fun!(z_reply_replier_zid))
                        .method(fun!(z_reply_replier_eid))
                        .method(fun!(z_reply_is_ok))
                        .method(fun!(z_reply_sample))
                        .method(fun!(z_reply_error_payload))
                        .method(fun!(z_reply_error_encoding)),
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
            package!("session").class(
                ptr_class!(ZSession)
                    .constructor(fun!(z_open))
                    .method(fun!(z_session_zid).name("getZid"))
                    .method(fun!(z_session_declare_publisher).split_on_param("key_expr"))
                    .method(fun!(z_session_put).split_on_param("key_expr"))
                    .method(fun!(z_session_delete).split_on_param("key_expr"))
                    .method(fun!(z_session_declare_subscriber).split_on_param("key_expr"))
                    .method(fun!(z_session_declare_querier).split_on_param("key_expr"))
                    .method(fun!(z_session_declare_queryable).split_on_param("key_expr"))
                    .method(fun!(z_session_declare_keyexpr))
                    // Undeclaring needs the declared handle, not a string — opt its
                    // key_expr param out of the default construction variants.
                    .method(
                        fun!(z_session_undeclare_keyexpr)
                            .expand_param("key_expr", expand_param!(ZKeyExpr).variant_self()),
                    )
                    .method(fun!(z_session_get).split_on_param("key_expr"))
                    // `Vec<ZenohId>` folds each value-blob element normally.
                    .method(fun!(z_session_peers_zid))
                    .method(fun!(z_session_routers_zid))
                    // Liveliness extends Session but retains its own source namespace;
                    // after stripping only the `z` marker these become
                    // `livelinessDeclareToken`, `livelinessGet`, … automatically.
                    .method(fun!(z_liveliness_declare_token).split_on_param("key_expr"))
                    .method(fun!(z_liveliness_get).split_on_param("key_expr"))
                    .method(fun!(z_liveliness_declare_subscriber).split_on_param("key_expr")),
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
