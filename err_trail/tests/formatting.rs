#![cfg(all(feature = "log", feature = "tracing"))]

use std::io;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct Output(Arc<Mutex<Vec<u8>>>);

impl Output {
    fn take(&self) -> String {
        String::from_utf8(std::mem::take(&mut *self.0.lock().unwrap())).unwrap()
    }
}

impl io::Write for Output {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

static MESSAGES: Mutex<Vec<String>> = Mutex::new(Vec::new());

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        MESSAGES.lock().unwrap().push(record.args().to_string());
    }

    fn flush(&self) {}
}

#[test]
fn generated_text_matches_tracing_default_field_style() {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let output = Output::default();
    let writer = output.clone();
    // Keep the default Full event and DefaultFields formatters; omit metadata
    // that log's logger controls separately from the generated message body.
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_ansi(false)
        .with_level(false)
        .with_target(false)
        .with_writer(move || writer.clone())
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        macro_rules! compare {
            ($($args:tt)*) => {{
                tracing::warn!($($args)*);
                let expected = output.take();
                MESSAGES.lock().unwrap().clear();

                err_trail::warn!($($args)*);
                assert_eq!(output.take(), expected);
                assert_eq!(
                    std::mem::take(&mut *MESSAGES.lock().unwrap()),
                    [expected.strip_suffix('\n').unwrap()],
                );
            }};
        }

        let text = "connection reset\n\"retry\"";
        let values = [1, 2, 3];
        compare!("plain message");
        compare!(
            count = 3u64,
            ready = true,
            ratio = 1.5f64,
            text,
            "Retry {}",
            2
        );
        compare!(count = 3u64, "");
        compare!({ count = 3u64, ready = false, });
        compare!(text, display = %text, debug = ?text, ?values, "Values");
        compare!(r#type = "request", "r#quoted" = 2, "Labels");
        compare!(request.id = 42u64, "request-id" = 7, "braces{}" = true);
        compare!(target: "network", attempts = 3, "Retrying");
    });
}
