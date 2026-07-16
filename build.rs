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
//! with `.method(fun!(f))` (instance methods) and `.constructor(fun!(f))`
//! (companion factories); the per-fn `.expand_param(name, …)` /
//! `.expand_return(…)` overrides (chained on the `fun!` decl, taking the same
//! expand-decl objects) replace the defaults — an identity-only set
//! (`.variant_self()` / `.field_self()` alone) is the plain raw-handle form.
//!
//! Kotlin **method names are derived automatically** by the
//! `set_method_name_mangle` hook ([`strip_flat_class_prefix`], which strips the
//! class-name prefix): `sample_get_payload` → `getPayload`, `keyexpr_get_str`
//! → `getStr`, `keyexpr_new_join` → `newJoin`. An explicit `.name(...)` is
//! used only where absolutely necessary: `toStr` (a derived `toString` would
//! clash with Kotlin's `Any.toString()`) and the `message` field label of the
//! class-less rust-side-only `Error` decomposition.
//!
//! Where a multi-variant param crosses as the string-or-handle `KeyExpr`
//! idiom, `.split_on_param("key_expr")` emits idiomatic typed overloads
//! (`f(keyExpr: String, …)` / `f(keyExpr: KeyExpr, …)`) over the selector form,
//! so callers pass a value directly instead of a `(selector, …)` tuple.
//!
//! Errors are delivered through the per-call `onError` callback (no Rust-side
//! JVM throw): `Error` (zenoh's native error) is the `E` of every fallible
//! `Result<_, Error>`, and its default return field (`error_get_message ->
//! String`) auto-applies to the `E` position so `onError` receives the message.
//!
//! Names mirror zenoh-flat's de-prefixed Rust identifiers one-to-one
//! (`KeyExpr`, `Session`, `keyexpr_new_try_from`, `open`, …); the Kotlin-side
//! names are derived from them automatically.

use prebindgen::{
    core::Registry,
    enum_class, expand_param, expand_return, fun,
    lang::{ConstDecl, FunctionDecl, JniGen},
    package, ptr_class, value_class,
};
use syn::parse_quote as pq;

fn fail(context: &str, err: impl std::fmt::Display) -> ! {
    eprintln!("error: prebindgen jnigen {context}: {err}");
    std::process::exit(1);
}

/// Namespace-relative member naming: strip the (case-insensitive) class-name
/// prefix from a class method's derived name so the flat crate's
/// `keyexpr_get_str` surfaces as `getStr` on `KeyExpr`, `zbytes_as_bytes` as
/// `asBytes` on `ZBytes`, etc. Registered via
/// [`JniGen::set_method_name_mangle`] — the generator's default method mangle
/// is identity (full camelCase), so this hook restores the de-prefixed API.
/// Members with an explicit `.name(...)` bypass the hook.
fn strip_flat_class_prefix(class: &str, name: &str) -> String {
    if name
        .get(..class.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(class))
    {
        let rest = &name[class.len()..];
        let mut chars = rest.chars();
        if let Some(first) = chars.next() {
            return first.to_lowercase().chain(chars).collect();
        }
    }
    name.to_string()
}

/// The two expression constants for one predefined encoding, composed from
/// zenoh-flat's general accessors over its `encoding_const_<name>()` loaning
/// factory (no per-preset accessor exists in the source crate):
/// `ENCODING_<NAME>: String` (canonical form, upstream `Display`) and
/// `ENCODING_<NAME>_ID: Int` (numeric wire id).
fn encoding_consts(lower: &str) -> [ConstDecl; 2] {
    let upper = lower.to_uppercase();
    let factory: syn::Ident = syn::parse_str(&format!("encoding_const_{lower}")).unwrap();
    [
        ConstDecl::named(format!("ENCODING_{upper}"))
            .expr(pq!(String), pq!(encoding_to_string(#factory()))),
        ConstDecl::named(format!("ENCODING_{upper}_ID"))
            .expr(pq!(i32), pq!(encoding_get_id(#factory()))),
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
    let encoding = ptr_class!(Encoding)
        .method(fun!(encoding_get_id))
        .method(fun!(encoding_get_schema))
        .method(fun!(encoding_to_string).name("toStr"))
        // Whole-handle clone return — see KeyExpr's `newClone`.
        .method(fun!(encoding_new_clone).expand_return(expand_return!(Encoding).field_self()))
        // `encoding_new_with_schema(&Encoding, schema) -> Encoding` derives a new
        // Encoding — a companion factory returning a raw handle (a constructor
        // never return-field-decomposes, so the result stays a usable handle rather than
        // a decomposed builder).
        .constructor(fun!(encoding_new_with_schema))
        // Factories → companion members; `fromId` is also the input variant
        // (see the `expand_param!(Encoding)` declaration below).
        .constructor(fun!(encoding_new_from_id))
        .constructor(fun!(encoding_new_from_string));

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
            ptr_class!(ZBytes)
                .method(fun!(zbytes_as_bytes))
                // Whole-handle clone return — see KeyExpr's `newClone`.
                .method(fun!(zbytes_new_clone).expand_return(expand_return!(ZBytes).field_self()))
                // `fromVec` builds a ZBytes from a `ByteArray` — both the
                // param-variant build arm (see `expand_param!(ZBytes)` below)
                // AND a companion factory (a constructor never
                // return-field-decomposes, so the factory keeps its
                // raw-handle return).
                .constructor(fun!(zbytes_new_from_vec)),
        )
        .class(encoding);
    for lower in [
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
    ] {
        for decl in encoding_consts(lower) {
            bytes = bytes.constant(decl);
        }
    }

    let mut jni = JniGen::new()
        .set_package_prefix("io.zenoh.jni") // base package of the generated JNI bindings
        // Every generated native call routes through `JNINative`; trigger our own
        // loader from its static initializer so the native library is loaded
        // transparently before any extern resolves (consumers never load it).
        .set_jni_native_init("io.zenoh.jni.NativeLibrary.ensureLoaded()")
        // De-prefix class-method names (`keyexpr_get_str` -> `getStr`): the
        // generator's default method mangle is identity, so restore the
        // namespace-relative naming this binding's Kotlin API expects.
        .set_method_name_mangle(|_, class, n| strip_flat_class_prefix(class, n))
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
                    ptr_class!(KeyExpr)
                        // Read accessors → instance methods on the KeyExpr class.
                        // `newClone` returns the borrowed clone as a WHOLE handle —
                        // override the class's default return fields (identity via
                        // the owned converter) so the borrowed-opaque clone path
                        // applies instead.
                        .method(fun!(keyexpr_get_str))
                        .method(
                            fun!(keyexpr_new_clone)
                                .expand_return(expand_return!(KeyExpr).field_self()),
                        )
                        .method(fun!(keyexpr_to_string).name("toStr"))
                        // Constructors → companion factories returning `Result<KeyExpr, Error>`;
                        // `tryFrom` is also the build-from-String input variant
                        // (see `expand_param!(KeyExpr)` below).
                        .constructor(fun!(keyexpr_new_try_from))
                        .constructor(fun!(keyexpr_new_autocanonize))
                        // `a` is a `&KeyExpr` (string-or-handle); split it so
                        // `join(a: KeyExpr, b: String)` is an idiomatic overload.
                        .constructor(fun!(keyexpr_new_join).split_on_param("a"))
                        .constructor(fun!(keyexpr_new_concat).split_on_param("a"))
                        // Consumer methods: the receiver key-expr is `this`; the other
                        // param accepts a String (built via the default param variants below).
                        .method(fun!(keyexpr_intersects).split_on_param("b"))
                        .method(fun!(keyexpr_includes).split_on_param("b"))
                        .method(fun!(keyexpr_relation_to).split_on_param("b")),
                )
                .class(enum_class!(SetIntersectionLevel)),
        )
        // Default param variants: a key-expr param accepts EITHER a String (built
        // via `tryFrom`) OR an existing handle (self). Default return field: the
        // handle only (the string form stays the `getStr` accessor method).
        .expand(
            expand_param!(KeyExpr)
                .variant(fun!(keyexpr_new_try_from))
                .variant_self(),
        )
        .expand(expand_return!(KeyExpr).field_self())
        // ── Config + ZenohId ──────────────────────────────────────────────
        .package(
            package!("config")
                .class(
                    ptr_class!(Config)
                        .method(fun!(config_get_json))
                        .method(fun!(config_new_clone))
                        // `config.insertJson5(...)` — receiver-style mutator.
                        .method(fun!(config_insert_json5))
                        // Factories → Config companion-object members.
                        .constructor(fun!(config_new_default))
                        .constructor(fun!(config_new_from_file))
                        .constructor(fun!(config_new_from_json))
                        .constructor(fun!(config_new_from_json5))
                        .constructor(fun!(config_new_from_yaml)),
                )
                .class(enum_class!(WhatAmI))
                // `ZenohId` is a `Copy` value (zenoh's `ZenohId`, repr(transparent)), so
                // it crosses as a raw byte-blob `ByteArray` rather than a closeable jlong
                // handle. `Vec<ZenohId>` (session peers/routers) folds each element WHOLE
                // as the typed `ZenohId` value class. Its read accessors become methods
                // on the value class (receiver = `this.bytes`).
                .class(
                    value_class!(ZenohId)
                        .method(fun!(zenoh_id_to_bytes))
                        .method(fun!(zenoh_id_to_string).name("toStr")),
                ),
        )
        // ── Scouting ──────────────────────────────────────────────────────
        // Canonical output: the scout callback decomposes a `Hello` into its
        // three read fields in ONE crossing (no handle — read-only). Auto-applies
        // to `scout`'s `Fn(Hello)`.
        .package(
            package!("scouting")
                .class(
                    ptr_class!(Hello)
                        .method(fun!(hello_get_whatami)) // WhatAmI enum -> Int
                        .method(fun!(hello_get_zid)) // ZenohId value class -> ByteArray
                        .method(fun!(hello_get_locators)), // Vec<String> -> List<String>
                )
                .class(ptr_class!(Scout))
                .fun(fun!(scout)),
        )
        .expand(
            expand_return!(Hello)
                .field(fun!(hello_get_whatami))
                .field(fun!(hello_get_zid))
                .field(fun!(hello_get_locators)),
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
        .expand(expand_param!(ZBytes).variant(fun!(zbytes_new_from_vec)))
        .expand(expand_return!(ZBytes).field_self())
        // Encoding default input: the decomposed value `(id, schema?)` (built
        // via `fromId`) — cheap primitives, no per-call String parse. Default
        // output: the handle + its id (both free jvalue slots); schema and
        // the canonical string stay on-demand accessors.
        .expand(expand_param!(Encoding).variant(fun!(encoding_new_from_id)))
        .expand(
            expand_return!(Encoding)
                .field_self()
                .field(fun!(encoding_get_id)),
        )
        // ── Time ──────────────────────────────────────────────────────────
        // Canonical output: a timestamp is its NTP64 value (`timestamp_get_ntp64`
        // -> i64 -> Long, 1 leaf); nested in a `Sample` it contributes that Long.
        .package(
            package!("time").class(
                ptr_class!(Timestamp)
                    .method(fun!(timestamp_get_ntp64))
                    .method(fun!(timestamp_get_id)),
            ),
        )
        .expand(expand_return!(Timestamp).field(fun!(timestamp_get_ntp64)))
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
            package!("sample")
                .class(enum_class!(SampleKind))
                .class(
                    ptr_class!(Sample)
                        // All sample getters are record sources AND instance methods on
                        // the Sample class; decomposition happens via the canonical
                        // output below.
                        .method(fun!(sample_get_key_expr))
                        .method(fun!(sample_get_payload))
                        .method(fun!(sample_get_encoding))
                        .method(fun!(sample_get_kind))
                        .method(fun!(sample_get_timestamp))
                        .method(fun!(sample_get_express))
                        .method(fun!(sample_get_priority))
                        .method(fun!(sample_get_congestion_control))
                        .method(fun!(sample_get_attachment))
                        .method(fun!(sample_get_reliability))
                        .method(fun!(sample_get_source_zid))
                        .method(fun!(sample_get_source_eid))
                        .method(fun!(sample_get_source_sn)),
                )
                // Standalone sample constructors (callable from Kotlin); consumed by handle.
                .fun(fun!(sample_new_put))
                .fun(fun!(sample_new_delete)),
        )
        // Identity-only input: exactly the default (documented no-op).
        .expand(expand_param!(Sample).variant_self())
        // Full-sample decomposition; field names inherit from the members above.
        .expand(
            expand_return!(Sample)
                .field(fun!(sample_get_key_expr))
                .field(fun!(sample_get_payload))
                .field(fun!(sample_get_encoding))
                .field(fun!(sample_get_kind))
                .field(fun!(sample_get_timestamp))
                .field(fun!(sample_get_express))
                .field(fun!(sample_get_priority))
                .field(fun!(sample_get_congestion_control))
                .field(fun!(sample_get_attachment))
                .field(fun!(sample_get_reliability))
                .field(fun!(sample_get_source_zid))
                .field(fun!(sample_get_source_eid))
                .field(fun!(sample_get_source_sn)),
        )
        // ── Pub/Sub ───────────────────────────────────────────────────────
        // key_expr / payload / attachment / encoding params are auto-constructed
        // by their types' canonical inputs (no per-fn calls).
        .package(
            package!("pubsub")
                // `publisher.put(...)` / `publisher.delete(...)` — receiver-style.
                .class(
                    ptr_class!(Publisher)
                        .method(fun!(publisher_put))
                        .method(fun!(publisher_delete)),
                )
                .class(ptr_class!(Subscriber)),
        )
        // ── Query / Queryable / Querier ───────────────────────────────────
        .package(
            package!("query")
                .class(ptr_class!(Queryable))
                // `querier.get(...)` — receiver-style method on Querier.
                .class(ptr_class!(Querier).method(fun!(querier_get)))
                .class(enum_class!(ReplyKeyExpr))
                .class(enum_class!(QueryTarget))
                .class(enum_class!(ConsolidationMode))
                .class(
                    ptr_class!(Query)
                        .method(fun!(query_get_keyexpr))
                        .method(fun!(query_get_parameters))
                        .method(fun!(query_get_payload))
                        .method(fun!(query_get_encoding))
                        .method(fun!(query_get_attachment))
                        .method(fun!(query_get_accepts_replies))
                        // Reply ops on the owned/borrowed query handle →
                        // `query.replySuccess(...)` / `replyError` / `replyDelete`.
                        .method(fun!(query_reply_success).split_on_param("key_expr"))
                        .method(fun!(query_reply_error))
                        .method(fun!(query_reply_delete).split_on_param("key_expr"))
                        // `query_reply_sample` takes the sample by owned handle
                        // (Sample's canonical input is identity).
                        .method(fun!(query_reply_sample)),
                )
                // ── Reply / ReplyError ────────────────────────────────────────
                .class(
                    ptr_class!(ReplyError)
                        .method(fun!(reply_error_get_payload))
                        .method(fun!(reply_error_get_encoding)),
                )
                .class(
                    ptr_class!(Reply)
                        // Record sources are class methods — `reply.sample()`'s
                        // standalone export is therefore the cloned-handle form.
                        .method(fun!(reply_get_replier_zid))
                        .method(fun!(reply_get_replier_eid))
                        .method(fun!(reply_is_ok))
                        .method(fun!(reply_get_sample))
                        .method(fun!(reply_get_err)),
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
            expand_return!(Query)
                .field(fun!(query_get_keyexpr))
                .field(fun!(query_get_parameters))
                .field(fun!(query_get_payload))
                .field(fun!(query_get_encoding))
                .field(fun!(query_get_attachment))
                .field(fun!(query_get_accepts_replies))
                .field_self(),
        )
        // ReplyError canonical output: a failed reply's error decomposed in
        // one crossing — payload -> ByteArray, encoding -> String.
        .expand(
            expand_return!(ReplyError)
                .field(fun!(reply_error_get_payload))
                .field(fun!(reply_error_get_encoding)),
        )
        // Reply canonical output: the whole reply decomposed in ONE crossing
        // (PRODUCT model — both arms' leaves always present, the not-taken
        // arm's are null). replier zid/eid + the is_ok discriminator, then the
        // ok arm splices the full sample and the err arm splices
        // payload/encoding. Auto-applies to the `Fn(Reply)` callbacks of
        // `session_get` / `querier_get` / liveliness get; no identity record,
        // so no `Reply` handle crosses.
        .expand(
            expand_return!(Reply)
                .field(fun!(reply_get_replier_zid))
                .field(fun!(reply_get_replier_eid))
                .field(fun!(reply_is_ok))
                .field(fun!(reply_get_sample))
                .field(fun!(reply_get_err)),
        )
        // ── Liveliness + Session ──────────────────────────────────────────
        // `LivelinessToken` is just an opaque handle; the liveliness operations
        // (`liveliness_*`) are declared under the `session` package below,
        // alongside the session API they extend.
        .package(package!("liveliness").class(ptr_class!(LivelinessToken)))
        .package(
            package!("session").class(
                // Every session operation is a RECEIVER-STYLE instance method on
                // `Session` (its `&Session` first param binds to `this`), so the
                // Kotlin surface reads `session.put(...)` / `session.declarePublisher(...)`.
                // `open` has no receiver (it creates a Session) → companion factory.
                ptr_class!(Session)
                    .constructor(fun!(open))
                    .method(fun!(session_get_zid))
                    .method(fun!(session_declare_publisher).split_on_param("key_expr"))
                    .method(fun!(session_put).split_on_param("key_expr"))
                    .method(fun!(session_delete).split_on_param("key_expr"))
                    .method(fun!(session_declare_subscriber).split_on_param("key_expr"))
                    .method(fun!(session_declare_querier).split_on_param("key_expr"))
                    .method(fun!(session_declare_queryable).split_on_param("key_expr"))
                    .method(fun!(session_declare_keyexpr))
                    // Undeclaring needs the declared handle, not a string — opt its
                    // key_expr param out of the (String-building) default param variants.
                    .method(
                        fun!(session_undeclare_keyexpr)
                            .expand_param("key_expr", expand_param!(KeyExpr).variant_self()),
                    )
                    .method(fun!(session_get).split_on_param("key_expr"))
                    // `Vec<ZenohId>`: ZenohId is a value class, so these return
                    // `List<ZenohId>` via the normal Vec converter. Named to drop
                    // the `get` prefix (`peersZid` / `routersZid`).
                    .method(fun!(session_get_peers_zid))
                    .method(fun!(session_get_routers_zid))
                    // Liveliness ops also take `&Session` first → Session methods.
                    .method(fun!(liveliness_declare_token).split_on_param("key_expr"))
                    .method(fun!(liveliness_get).split_on_param("key_expr"))
                    .method(fun!(liveliness_declare_subscriber).split_on_param("key_expr")),
            ),
        );

    // zenoh-flat's `encoding_const_*` `&'static Encoding` loaning factories
    // are superseded here by the `ENCODING_*` consts above — acknowledge the
    // whole naming family so the generator doesn't warn about undeclared
    // functions.
    jni = jni.ignore(prebindgen::matching(|name| {
        name.starts_with("encoding_const_")
    }));

    // The remaining flat functions are intentionally outside this binding
    // surface. Handle classes already provide close/take lifecycle operations;
    // the SDK retains its declared key expression/entity metadata; and the Vec
    // ZBytes constructor is the canonical ByteArray input path. Acknowledge the
    // exclusions explicitly so newly added source functions remain visible as
    // generation warnings instead of being lost in a standing warning list.
    for name in [
        "liveliness_undeclare_token",
        "publisher_get_eid",
        "publisher_get_keyexpr",
        "publisher_get_zid",
        "publisher_undeclare",
        "querier_get_eid",
        "querier_get_keyexpr",
        "querier_get_zid",
        "querier_undeclare",
        "queryable_get_eid",
        "queryable_get_keyexpr",
        "queryable_get_zid",
        "queryable_undeclare",
        "session_close",
        "session_is_closed",
        "session_new_timestamp",
        "subscriber_get_eid",
        "subscriber_get_keyexpr",
        "subscriber_get_zid",
        "subscriber_undeclare",
        "zbytes_new_from_slice",
    ] {
        jni = jni.ignore(FunctionDecl::new(syn::parse_str(name).unwrap()));
    }

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
