#![deny(warnings)]

use std::cell::Cell;

#[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
const EXPECTED: &[&str] = &[
    "error=\"library error\"",
    "library warning code=42",
    "library info",
    "library debug",
    "library trace",
    "library context",
    "library missing",
    "parent info",
];

fn main() {
    #[cfg(feature = "tracing")]
    let output = tracing_output::Output::default();
    #[cfg(feature = "tracing")]
    let _subscriber = {
        let writer = output.clone();
        tracing::subscriber::set_default(
            tracing_subscriber::fmt()
                .without_time()
                .with_ansi(false)
                .with_level(false)
                .with_target(false)
                .with_max_level(tracing::Level::TRACE)
                .with_writer(move || writer.clone())
                .finish(),
        )
    };
    #[cfg(feature = "log")]
    {
        log::set_logger(&log_output::Logger).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    }
    #[cfg(feature = "defmt")]
    defmt::export::fetch_bytes();

    let evaluations = Cell::new(0);
    feature_free_library::emit_logs(&evaluations);
    trail::info!("parent info");

    // Each argument/closure runs once in total, even with multiple backends.
    let enabled = cfg!(any(feature = "tracing", feature = "log", feature = "defmt"));
    assert_eq!(evaluations.get(), if enabled { 2 } else { 0 });

    #[cfg(feature = "tracing")]
    {
        let text = String::from_utf8(output.0.lock().unwrap().clone()).unwrap();
        assert_eq!(
            text.lines().map(str::trim_end).collect::<Vec<_>>(),
            EXPECTED
        );
    }
    #[cfg(feature = "log")]
    assert_eq!(*log_output::MESSAGES.lock().unwrap(), EXPECTED);
    #[cfg(feature = "defmt")]
    {
        let bytes = defmt::export::fetch_bytes();
        for message in EXPECTED {
            let occurrences = bytes
                .windows(message.len())
                .filter(|part| *part == message.as_bytes())
                .count();
            assert_eq!(
                occurrences, 1,
                "defmt output missing or duplicated {message}"
            );
        }
    }
}

#[cfg(feature = "tracing")]
mod tracing_output {
    use std::io;
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Default)]
    pub struct Output(pub Arc<Mutex<Vec<u8>>>);

    impl io::Write for Output {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
}

#[cfg(feature = "log")]
mod log_output {
    use std::sync::Mutex;

    pub static MESSAGES: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub struct Logger;

    impl log::Log for Logger {
        fn enabled(&self, _: &log::Metadata<'_>) -> bool {
            true
        }

        fn log(&self, record: &log::Record<'_>) {
            MESSAGES.lock().unwrap().push(record.args().to_string());
        }

        fn flush(&self) {}
    }
}
