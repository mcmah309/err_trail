#![allow(dead_code, unused_variables)]

use std::cell::Cell;
use std::fmt;

struct DisplayOnly;

impl fmt::Display for DisplayOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("display value")
    }
}

#[derive(Debug)]
struct DebugOnly(u8);

#[test]
fn syntax_borrowing_and_single_evaluation() {
    use err_trail::{debug, error, info, trace, warn};

    let display = DisplayOnly;
    let debug_value = DebugOnly(7);
    let text = String::from("owned");
    let count = 3u64;
    let calls = Cell::new(0);
    let next = || {
        calls.set(calls.get() + 1);
        String::from("temporary")
    };

    // Every expression is evaluated once across all selected backends. The
    // temporary Strings must survive until the last backend has consumed them.
    info!(target: concat!("net", "work"), first = %next(), second = ?next(), "{}", next());
    assert_eq!(
        calls.get(),
        if cfg!(any(feature = "tracing", feature = "log", feature = "defmt")) {
            3
        } else {
            0
        }
    );

    error!(%display);
    warn!(?debug_value, count,);
    info!(display = %display, debug_value = ?debug_value, text, count);
    debug!(debug_value.inner = ?debug_value);
    trace!({ request.id = count, "http.status" = 200u64, "braces{}" = true, }, "{text}");
    info!({ count, %display, ?debug_value, });
    info!(target: "only_fields", { count, });
    info!(target: "message", "{text} {named:04}", named = count);
    info!(concat!("number=", "{}"), count);
    info!(core::concat!("{}", "!"), text);
    info!(computed = ?[1, 2].map(|x| x + 1), "done");
    info!(computed = ?Result::<u8, u16>::Ok(1), "done");
    info!(r#type = count);

    // Generated names and backend imports must not shadow the caller's names.
    let __err_trail_message = "caller message";
    let __err_trail_field_0 = "caller field";
    let defmt = "caller variable";
    info!(%__err_trail_field_0, %defmt, "{__err_trail_message}");
    #[allow(non_upper_case_globals)]
    const __err_trail_target: &str = "caller target";
    info!(target: __err_trail_target, "{__err_trail_target}");

    struct Request {
        id: u64,
    }
    let request = Request { id: 42 };
    info!(request.id, %request.id, ?request.id);

    assert_eq!(text, "owned");
    assert_eq!(format!("{display}"), "display value");
    assert_eq!(debug_value.0, 7);
}

#[cfg(not(any(feature = "tracing", feature = "log", feature = "defmt")))]
#[test]
fn no_backend_does_not_resolve_or_typecheck_values() {
    struct NoFormatting;
    let value = NoFormatting;
    err_trail::info!(target: nonexistent::TARGET, %value, ?missing, "{}", nonexistent());
    let _: () = err_trail::info!(?value);
}

#[cfg(feature = "tracing")]
mod tracing_output {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};
    use tracing::{
        Event, Metadata, Subscriber,
        field::{Field, Visit},
        span::{Attributes, Id, Record},
    };

    #[derive(Debug, PartialEq)]
    enum Value {
        U64(u64),
        Bool(bool),
        Str(String),
        Debug(String),
    }

    #[derive(Default)]
    struct Fields(BTreeMap<String, Value>);

    impl Visit for Fields {
        fn record_u64(&mut self, field: &Field, value: u64) {
            self.0.insert(field.name().into(), Value::U64(value));
        }
        fn record_bool(&mut self, field: &Field, value: bool) {
            self.0.insert(field.name().into(), Value::Bool(value));
        }
        fn record_str(&mut self, field: &Field, value: &str) {
            self.0.insert(field.name().into(), Value::Str(value.into()));
        }
        fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
            self.0
                .insert(field.name().into(), Value::Debug(format!("{value:?}")));
        }
    }

    struct Captured {
        fields: Fields,
        target: String,
        module: Option<&'static str>,
        file: Option<&'static str>,
        line: Option<u32>,
    }

    #[derive(Clone, Default)]
    struct Capture(Arc<Mutex<Vec<Captured>>>);

    impl Subscriber for Capture {
        fn enabled(&self, metadata: &Metadata<'_>) -> bool {
            // Field-based filtering must see real fields in event metadata.
            metadata.fields().field("request.id").is_some()
        }
        fn new_span(&self, _: &Attributes<'_>) -> Id {
            Id::from_u64(1)
        }
        fn record(&self, _: &Id, _: &Record<'_>) {}
        fn record_follows_from(&self, _: &Id, _: &Id) {}
        fn enter(&self, _: &Id) {}
        fn exit(&self, _: &Id) {}
        fn event(&self, event: &Event<'_>) {
            let mut fields = Fields::default();
            event.record(&mut fields);
            let metadata = event.metadata();
            self.0.lock().unwrap().push(Captured {
                fields,
                target: metadata.target().into(),
                module: metadata.module_path(),
                file: metadata.file(),
                line: metadata.line(),
            });
        }
    }

    #[test]
    fn preserves_structured_types_metadata_and_fields_without_message() {
        let capture = Capture::default();
        let display = DisplayOnly;
        let request = DebugOnly(7);
        let expected_line = Cell::new(0);
        tracing::subscriber::with_default(capture.clone(), || {
            expected_line.set(line!() + 1);
            err_trail::warn!(target: "network", request.id = 42u64, ready = true, label = "raw", %display, ?request, "Retry {}", 3);
            err_trail::info!({ request.id = 43u64, "http.status" = 200u64 });
        });
        let events = capture.0.lock().unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].target, "network");
        assert_eq!(events[0].module, Some(module_path!()));
        assert_eq!(events[0].file, Some(file!()));
        assert_eq!(events[0].line, Some(expected_line.get()));
        assert_eq!(
            events[0].fields.0,
            BTreeMap::from([
                ("request.id".into(), Value::U64(42)),
                ("ready".into(), Value::Bool(true)),
                ("label".into(), Value::Str("raw".into())),
                ("display".into(), Value::Debug("display value".into())),
                ("request".into(), Value::Debug("DebugOnly(7)".into())),
                ("message".into(), Value::Debug("Retry 3".into())),
            ])
        );
        assert_eq!(events[1].target, module_path!());
        assert_eq!(
            events[1].fields.0,
            BTreeMap::from([
                ("request.id".into(), Value::U64(43)),
                ("http.status".into(), Value::U64(200)),
            ])
        );
    }
}

#[cfg(feature = "log")]
mod log_output {
    use super::*;
    use std::sync::Mutex;

    static RECORDS: Mutex<Vec<(String, String, log::Level)>> = Mutex::new(Vec::new());
    struct Logger;
    impl log::Log for Logger {
        fn enabled(&self, _: &log::Metadata<'_>) -> bool {
            true
        }
        fn log(&self, record: &log::Record<'_>) {
            if record.target() == "fallback" {
                RECORDS.lock().unwrap().push((
                    record.target().into(),
                    record.args().to_string(),
                    record.level(),
                ));
            }
        }
        fn flush(&self) {}
    }

    #[test]
    fn formats_fields_and_preserves_target() {
        log::set_logger(&Logger).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
        let display = DisplayOnly;
        let debug_value = DebugOnly(7);
        err_trail::warn!(target: "fallback", count = 3u64, %display, ?debug_value, text = "raw", "Retry {}", 2);
        err_trail::info!(target: "fallback", { "braces{}" = true });
        err_trail::error!(target: "fallback", "{named:04}", named = 7);
        assert_eq!(
            *RECORDS.lock().unwrap(),
            vec![
                (
                    "fallback".into(),
                    "Retry 2 count=3 display=display value debug_value=DebugOnly(7) text=\"raw\""
                        .into(),
                    log::Level::Warn
                ),
                ("fallback".into(), "braces{}=true".into(), log::Level::Info),
                ("fallback".into(), "0007".into(), log::Level::Error),
            ]
        );
    }
}

#[cfg(feature = "defmt")]
#[test]
fn defmt_uses_core_formatting_for_fields() {
    let display = DisplayOnly;
    let debug_value = DebugOnly(7);
    defmt::export::fetch_bytes();
    err_trail::error!(count = 3u64, %display, ?debug_value, "Retry {}", 2);
    let bytes = defmt::export::fetch_bytes();
    let expected = b"Retry 2 count=3 display=display value debug_value=DebugOnly(7)";
    assert!(bytes.windows(expected.len()).any(|part| part == expected));
    err_trail::error!(target: "network", { count = 3u64 });
    let bytes = defmt::export::fetch_bytes();
    assert!(bytes.windows(7).any(|part| part == b"network"));
    assert!(bytes.windows(7).any(|part| part == b"count=3"));
}
