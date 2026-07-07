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
//! A type's default flatten shape — its input (`.flatten_input(fun!(ctor))` /
//! `.flatten_input_self()`) and output (`.flatten_output(fun!(acc).name(..))` /
//! `.flatten_output_self()`) — is declared once on the `ptr_class!` decl and
//! AUTO-APPLIES to every matching param / return. Class members are declared
//! with `.fun(fun!(f).name(..))` (instance methods) and
//! `.constructor(fun!(f).name(..))` (companion factories), and the per-fn
//! `.flatten_input_suppress` / `.flatten_output_suppress` modifiers (chained
//! on the `fun!` decl) opt out.
//!
//! Errors are delivered through the per-call `onError` callback (no Rust-side
//! JVM throw): `Error` (zenoh's native error) is the `E` of every fallible
//! `Result<_, Error>`, and its default output flatten (`error_get_message ->
//! String`) auto-applies to the `E` position so `onError` receives the message.
//!
//! Names mirror zenoh-flat's de-prefixed Rust identifiers one-to-one
//! (`KeyExpr`, `Session`, `keyexpr_new_try_from`, `open`, …); the Kotlin-side
//! names are derived from them automatically.

use prebindgen::core::Registry;
use prebindgen::lang::{FunctionDecl, JniGen, JniGenConfig};
use prebindgen::{enum_class, fun, ident, package, ptr_class, value_class};
use syn::parse_quote as pq;

fn fail(context: &str, err: impl std::fmt::Display) -> ! {
    eprintln!("error: prebindgen jnigen {context}: {err}");
    std::process::exit(1);
}

/// Companion-factory name for a predefined `encoding_const_<x>` constant:
/// strip the `encoding_const_` prefix and lower-camelCase the rest
/// (`encoding_const_text_plain` → `textPlain`).
fn const_name(ident: &syn::Ident) -> String {
    let s = ident.to_string();
    let leaf = s.strip_prefix("encoding_const_").unwrap_or(&s);
    let mut out = String::new();
    let mut upper = false;
    for c in leaf.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            out.push(c.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
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
    let mut encoding = ptr_class!(Encoding)
        .fun(fun!(encoding_get_id).name("id"))
        .fun(fun!(encoding_get_schema).name("getSchema"))
        .fun(fun!(encoding_to_string).name("toStr"))
        // Whole-handle clone return — see KeyExpr's `newClone`.
        .fun(fun!(encoding_new_clone).name("newClone").flatten_output_suppress())
        // `encoding_new_with_schema(&Encoding, schema) -> Encoding` derives a new
        // Encoding — a companion factory returning a raw handle (a constructor
        // never output-flattens, so the result stays a usable handle rather than
        // a decomposed builder).
        .constructor(fun!(encoding_new_with_schema).name("withSchema"))
        // Factories → companion members; `fromId` is also the input variant.
        .constructor(fun!(encoding_new_from_id).name("fromId"))
        .constructor(fun!(encoding_new_from_string).name("fromString"))
        // Input flatten: encoding params cross as the decomposed value
        // `(id, schema?)` (built via `fromId`). Output flatten: the handle +
        // its id (both free jvalue slots); schema/string stay accessors.
        .flatten_input(fun!(encoding_new_from_id))
        .flatten_output_self()
        .flatten_output(fun!(encoding_get_id).name("id"));

    // Predefined encoding constants — each is a nullary `() -> &'static Encoding`
    // factory, declared as a companion `.constructor` of `Encoding` so they
    // collapse into one loop instead of ~50 identical lines. The factory name
    // strips the `encoding_const_` prefix and camelCases the rest
    // (`text_plain` → `textPlain`). Runtime idents can't use the `fun!`
    // macro, so this is the spelled-out `FunctionDecl::new(ident)` form.
    for ctor in [
        ident!(encoding_const_zenoh_bytes),
        ident!(encoding_const_zenoh_string),
        ident!(encoding_const_zenoh_serialized),
        ident!(encoding_const_application_octet_stream),
        ident!(encoding_const_text_plain),
        ident!(encoding_const_application_json),
        ident!(encoding_const_text_json),
        ident!(encoding_const_application_cdr),
        ident!(encoding_const_application_cbor),
        ident!(encoding_const_application_yaml),
        ident!(encoding_const_text_yaml),
        ident!(encoding_const_text_json5),
        ident!(encoding_const_application_python_serialized_object),
        ident!(encoding_const_application_protobuf),
        ident!(encoding_const_application_java_serialized_object),
        ident!(encoding_const_application_openmetrics_text),
        ident!(encoding_const_image_png),
        ident!(encoding_const_image_jpeg),
        ident!(encoding_const_image_gif),
        ident!(encoding_const_image_bmp),
        ident!(encoding_const_image_webp),
        ident!(encoding_const_application_xml),
        ident!(encoding_const_application_x_www_form_urlencoded),
        ident!(encoding_const_text_html),
        ident!(encoding_const_text_xml),
        ident!(encoding_const_text_css),
        ident!(encoding_const_text_javascript),
        ident!(encoding_const_text_markdown),
        ident!(encoding_const_text_csv),
        ident!(encoding_const_application_sql),
        ident!(encoding_const_application_coap_payload),
        ident!(encoding_const_application_json_patch_json),
        ident!(encoding_const_application_json_seq),
        ident!(encoding_const_application_jsonpath),
        ident!(encoding_const_application_jwt),
        ident!(encoding_const_application_mp4),
        ident!(encoding_const_application_soap_xml),
        ident!(encoding_const_application_yang),
        ident!(encoding_const_audio_aac),
        ident!(encoding_const_audio_flac),
        ident!(encoding_const_audio_mp4),
        ident!(encoding_const_audio_ogg),
        ident!(encoding_const_audio_vorbis),
        ident!(encoding_const_video_h261),
        ident!(encoding_const_video_h263),
        ident!(encoding_const_video_h264),
        ident!(encoding_const_video_h265),
        ident!(encoding_const_video_h266),
        ident!(encoding_const_video_mp4),
        ident!(encoding_const_video_ogg),
        ident!(encoding_const_video_raw),
        ident!(encoding_const_video_vp8),
        ident!(encoding_const_video_vp9),
    ] {
        let name = const_name(&ctor);
        encoding = encoding.constructor(FunctionDecl::new(ctor).name(name));
    }

    let jni = JniGen::new(
        JniGenConfig::new()
            .source_module(pq!(zenoh_flat)) // module prefix for prebindgen-marked items
            .package_prefix("io.zenoh.jni") // base package of the generated JNI bindings
            // Every generated native call routes through `JNINative`; trigger our own
            // loader from its static initializer so the native library is loaded
            // transparently before any extern resolves (consumers never load it).
            .jni_native_init("io.zenoh.jni.NativeLibrary.ensureLoaded()"),
    )
    // ── Errors ────────────────────────────────────────────────────────
    // `Error` is the `E` of every fallible `Result<_, Error>`. Declaring it
    // as a `ptr_class` lets the `Result<T, Error>` resolver find its
    // converters; the handle never actually crosses (the extern peels the
    // `Result` and delivers the message via the error callback). Its
    // canonical output is the message string (1 leaf), auto-applied to the
    // `E` of every such `Result` — i.e. the `onError` callback's argument.
    .package(
        package!("errors").class(
            ptr_class!(Error)
                .fun(fun!(error_get_message).name("message"))
                .flatten_output(fun!(error_get_message).name("message")),
        ),
    )
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
                    // suppress the class's default output flatten (identity via
                    // the owned converter) so the borrowed-opaque clone path
                    // applies instead.
                    .fun(fun!(keyexpr_get_str).name("getStr"))
                    .fun(fun!(keyexpr_new_clone).name("newClone").flatten_output_suppress())
                    .fun(fun!(keyexpr_to_string).name("toStr"))
                    // Constructors → companion factories returning `Result<KeyExpr, Error>`;
                    // `tryFrom` is also the build-from-String input variant.
                    .constructor(fun!(keyexpr_new_try_from).name("tryFrom"))
                    .constructor(fun!(keyexpr_new_autocanonize).name("autocanonize"))
                    .constructor(fun!(keyexpr_new_join).name("join"))
                    .constructor(fun!(keyexpr_new_concat).name("concat"))
                    // Input flatten: a key-expr param accepts EITHER a String (built
                    // via `tryFrom`) OR an existing handle (self). Output flatten: the
                    // handle only (the string form stays the `getStr` accessor method).
                    .flatten_input(fun!(keyexpr_new_try_from))
                    .flatten_input_self()
                    .flatten_output_self()
                    // Consumer methods: the receiver key-expr is `this`; the other
                    // param accepts a String (built via the input flatten above).
                    .fun(fun!(keyexpr_intersects).name("intersects"))
                    .fun(fun!(keyexpr_includes).name("includes"))
                    .fun(fun!(keyexpr_relation_to).name("relationTo")),
            )
            .class(enum_class!(SetIntersectionLevel)),
    )
    // ── Config + ZenohId ──────────────────────────────────────────────
    .package(
        package!("config")
            .class(
                ptr_class!(Config)
                    .fun(fun!(config_get_json).name("getJson"))
                    .fun(fun!(config_new_clone).name("newClone"))
                    // Factories → Config companion-object members.
                    .constructor(fun!(config_new_default).name("newDefault"))
                    .constructor(fun!(config_new_from_file).name("fromFile"))
                    .constructor(fun!(config_new_from_json).name("fromJson"))
                    .constructor(fun!(config_new_from_json5).name("fromJson5"))
                    .constructor(fun!(config_new_from_yaml).name("fromYaml")),
            )
            .fun(fun!(config_insert_json5))
            .class(enum_class!(WhatAmI))
            // `ZenohId` is a `Copy` value (zenoh's `ZenohId`, repr(transparent)), so
            // it crosses as a raw byte-blob `ByteArray` rather than a closeable jlong
            // handle. `Vec<ZenohId>` (session peers/routers) folds each element WHOLE
            // as the typed `ZenohId` value class. Its read accessors become methods
            // on the value class (receiver = `this.bytes`).
            .class(
                value_class!(ZenohId)
                    .fun(fun!(zenoh_id_to_bytes).name("toBytes"))
                    .fun(fun!(zenoh_id_to_string).name("toStr")),
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
                    .fun(fun!(hello_get_whatami).name("whatami")) // WhatAmI enum -> Int
                    .fun(fun!(hello_get_zid).name("zid")) // ZenohId value class -> ByteArray
                    .fun(fun!(hello_get_locators).name("locators")) // Vec<String> -> List<String>
                    .flatten_output(fun!(hello_get_whatami).name("whatami"))
                    .flatten_output(fun!(hello_get_zid).name("zid"))
                    .flatten_output(fun!(hello_get_locators).name("locators")),
            )
            .class(ptr_class!(Scout))
            .fun(fun!(scout)),
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
    // ── Bytes: ZBytes + Encoding ──────────────────────────────────────
    // ZBytes canonical input: `payload`/`attachment` params accept a `ByteArray`
    // (built via `zbytes_new_from_vec`). Canonical output: the handle only
    // (identity) — the bytes are heavy and fetched on demand via
    // `zbytesAsBytes` (one borrow-copy).
    .package(
        package!("bytes")
            .class(
                ptr_class!(ZBytes)
                    .fun(fun!(zbytes_as_bytes).name("asBytes"))
                    // Whole-handle clone return — see KeyExpr's `newClone`.
                    .fun(fun!(zbytes_new_clone).name("newClone").flatten_output_suppress())
                    // `fromVec` builds a ZBytes from a `ByteArray` — both the
                    // input-flatten build variant AND a companion factory (a
                    // constructor never output-flattens, so the factory keeps its
                    // raw-handle return).
                    .constructor(fun!(zbytes_new_from_vec).name("fromVec"))
                    .flatten_input(fun!(zbytes_new_from_vec))
                    .flatten_output_self(),
            )
            .class(encoding),
    )
    // ── Time ──────────────────────────────────────────────────────────
    // Canonical output: a timestamp is its NTP64 value (`timestamp_get_ntp64`
    // -> i64 -> Long, 1 leaf); nested in a `Sample` it contributes that Long.
    .package(
        package!("time").class(
            ptr_class!(Timestamp)
                .fun(fun!(timestamp_get_ntp64).name("ntp64"))
                .fun(fun!(timestamp_get_id).name("getId"))
                .flatten_output(fun!(timestamp_get_ntp64).name("ntp64")),
        ),
    )
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
                    .fun(fun!(sample_get_key_expr).name("keyExpr"))
                    .fun(fun!(sample_get_payload).name("payload"))
                    .fun(fun!(sample_get_encoding).name("encoding"))
                    .fun(fun!(sample_get_kind).name("kind"))
                    .fun(fun!(sample_get_timestamp).name("timestamp"))
                    .fun(fun!(sample_get_express).name("express"))
                    .fun(fun!(sample_get_priority).name("priority"))
                    .fun(fun!(sample_get_congestion_control).name("congestionControl"))
                    .fun(fun!(sample_get_attachment).name("attachment"))
                    .fun(fun!(sample_get_reliability).name("reliability"))
                    .fun(fun!(sample_get_source_zid).name("sourceZid"))
                    .fun(fun!(sample_get_source_eid).name("sourceEid"))
                    .fun(fun!(sample_get_source_sn).name("sourceSn"))
                    .flatten_input_self()
                    .flatten_output(fun!(sample_get_key_expr).name("keyExpr"))
                    .flatten_output(fun!(sample_get_payload).name("payload"))
                    .flatten_output(fun!(sample_get_encoding).name("encoding"))
                    .flatten_output(fun!(sample_get_kind).name("kind"))
                    .flatten_output(fun!(sample_get_timestamp).name("timestamp"))
                    .flatten_output(fun!(sample_get_express).name("express"))
                    .flatten_output(fun!(sample_get_priority).name("priority"))
                    .flatten_output(fun!(sample_get_congestion_control).name("congestionControl"))
                    .flatten_output(fun!(sample_get_attachment).name("attachment"))
                    .flatten_output(fun!(sample_get_reliability).name("reliability"))
                    .flatten_output(fun!(sample_get_source_zid).name("sourceZid"))
                    .flatten_output(fun!(sample_get_source_eid).name("sourceEid"))
                    .flatten_output(fun!(sample_get_source_sn).name("sourceSn")),
            )
            // Standalone sample constructors (callable from Kotlin); consumed by handle.
            .fun(fun!(sample_new_put))
            .fun(fun!(sample_new_delete)),
    )
    // ── Pub/Sub ───────────────────────────────────────────────────────
    // key_expr / payload / attachment / encoding params are auto-constructed
    // by their types' canonical inputs (no per-fn calls).
    .package(
        package!("pubsub")
            .class(ptr_class!(Publisher))
            .fun(fun!(publisher_put))
            .fun(fun!(publisher_delete))
            .class(ptr_class!(Subscriber)),
    )
    // ── Query / Queryable / Querier ───────────────────────────────────
    .package(
        package!("query")
            .class(ptr_class!(Queryable))
            .class(ptr_class!(Querier))
            .fun(fun!(querier_get))
            .class(enum_class!(ReplyKeyExpr))
            .class(enum_class!(QueryTarget))
            .class(enum_class!(ConsolidationMode))
            // Canonical output: the queryable callback decomposes a `Query` into
            // BOTH its read fields AND the owned handle (identity) in ONE crossing
            // — the `output_direct` keeps the handle so the consumer can reply
            // (`query_reply_*`) after the callback returns; a query must outlive
            // its callback to be answered.
            //
            // ORDER MATTERS: `.flatten_output_self()` MUST be LAST. The root
            // identity moves the owned query while the nested KeyExpr identity
            // (from `query_get_keyexpr`) clones from a borrow of the same query;
            // identity leaves are emitted in declaration order, so the root move
            // must follow the nested borrow to avoid a "use of moved value" error.
            .class(
                ptr_class!(Query)
                    .fun(fun!(query_get_keyexpr).name("keyExpr"))
                    .fun(fun!(query_get_parameters).name("parameters"))
                    .fun(fun!(query_get_payload).name("payload"))
                    .fun(fun!(query_get_encoding).name("encoding"))
                    .fun(fun!(query_get_attachment).name("attachment"))
                    .fun(fun!(query_get_accepts_replies).name("acceptsReplies"))
                    .flatten_output(fun!(query_get_keyexpr).name("keyExpr"))
                    .flatten_output(fun!(query_get_parameters).name("parameters"))
                    .flatten_output(fun!(query_get_payload).name("payload"))
                    .flatten_output(fun!(query_get_encoding).name("encoding"))
                    .flatten_output(fun!(query_get_attachment).name("attachment"))
                    .flatten_output(fun!(query_get_accepts_replies).name("acceptsReplies"))
                    .flatten_output_self(),
            )
            // Reply ops on the owned/borrowed query handle.
            .fun(fun!(query_reply_success))
            .fun(fun!(query_reply_error))
            .fun(fun!(query_reply_delete))
            // `query_reply_sample` takes the sample by owned handle (Sample's
            // canonical input is identity — see the sample package above).
            .fun(fun!(query_reply_sample))
            // ── Reply / ReplyError ────────────────────────────────────────
            // ReplyError canonical output: a failed reply's error decomposed in
            // one crossing — payload -> ByteArray, encoding -> String.
            .class(
                ptr_class!(ReplyError)
                    .fun(fun!(reply_error_get_payload).name("payload"))
                    .fun(fun!(reply_error_get_encoding).name("encoding"))
                    .flatten_output(fun!(reply_error_get_payload).name("payload"))
                    .flatten_output(fun!(reply_error_get_encoding).name("encoding")),
            )
            // Reply canonical output: the whole reply decomposed in ONE crossing
            // (PRODUCT model — both arms' leaves always present, the not-taken
            // arm's are null). replier zid/eid + the is_ok discriminator, then the
            // ok arm splices the full sample and the err arm splices
            // payload/encoding. Auto-applies to the `Fn(Reply)` callbacks of
            // `session_get` / `querier_get` / liveliness get; no identity record,
            // so no `Reply` handle crosses.
            .class(
                ptr_class!(Reply)
                    // Record sources are class methods — `reply.sample()`'s
                    // standalone export is therefore the cloned-handle form.
                    .fun(fun!(reply_get_replier_zid).name("replierZid"))
                    .fun(fun!(reply_get_replier_eid).name("replierEid"))
                    .fun(fun!(reply_is_ok).name("isOk"))
                    .fun(fun!(reply_get_sample).name("sample"))
                    .fun(fun!(reply_get_err).name("err"))
                    .flatten_output(fun!(reply_get_replier_zid).name("replierZid"))
                    .flatten_output(fun!(reply_get_replier_eid).name("replierEid"))
                    .flatten_output(fun!(reply_is_ok).name("isOk"))
                    .flatten_output(fun!(reply_get_sample).name("sample"))
                    .flatten_output(fun!(reply_get_err).name("err")),
            ),
    )
    // ── Liveliness + Session ──────────────────────────────────────────
    // `LivelinessToken` is just an opaque handle; the liveliness operations
    // (`liveliness_*`) are declared under the `session` package below,
    // alongside the session API they extend.
    .package(package!("liveliness").class(ptr_class!(LivelinessToken)))
    .package(
        package!("session")
            .class(ptr_class!(Session).fun(fun!(session_get_zid).name("getZid")))
            .fun(fun!(open))
            .fun(fun!(session_declare_publisher))
            .fun(fun!(session_put))
            .fun(fun!(session_delete))
            .fun(fun!(session_declare_subscriber))
            .fun(fun!(session_declare_querier))
            .fun(fun!(session_declare_queryable))
            .fun(fun!(session_declare_keyexpr))
            // Undeclaring needs the declared handle, not a string — opt its
            // key_expr param out of the (String-building) input flatten.
            .fun(fun!(session_undeclare_keyexpr).flatten_input_suppress(pq!(key_expr)))
            .fun(fun!(session_get))
            // `Vec<ZenohId>`: ZenohId is a value class, so these return
            // `List<ZenohId>` via the normal Vec converter.
            .fun(fun!(session_get_peers_zid))
            .fun(fun!(session_get_routers_zid))
            // Liveliness ops (key_expr params auto-constructed by the canonical input).
            .fun(fun!(liveliness_declare_token))
            .fun(fun!(liveliness_get))
            .fun(fun!(liveliness_declare_subscriber)),
    );

    // ── Outputs ───────────────────────────────────────────────────────────
    // Run the configured adapter over zenoh-flat's captured `#[prebindgen]`
    // items and write both generated artifacts.
    let source = prebindgen::Source::new(zenoh_flat::PREBINDGEN_OUT_DIR);
    let mut registry = match Registry::from_items(source.items_all()) {
        Ok(registry) => registry,
        Err(err) => fail("scan failed", err),
    };

    // Rust bindings → src/generated_bindings.rs. Absolute path so the file lands
    // in the source tree (committed to git and included by `src/lib.rs`).
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let rust_dest = std::path::Path::new(&crate_dir)
        .join("src")
        .join("generated_bindings.rs");
    let rust_path = match registry.write_rust(&jni, &rust_dest) {
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
    // Remove stale generated files so package moves don't leave old classes
    // behind (e.g. io/zenoh/jni/* and io/zenoh/jni/<subpackage>/* side by side).
    if let Err(err) = std::fs::remove_dir_all(&kotlin_root) {
        if err.kind() != std::io::ErrorKind::NotFound {
            fail("cleanup kotlin/generated failed", err);
        }
    }
    for path in match jni.write_kotlin(&registry, &kotlin_root) {
        Ok(paths) => paths,
        Err(err) => fail("write_kotlin failed", err),
    } {
        println!("cargo:warning=Wrote {}", path.display());
    }
}
