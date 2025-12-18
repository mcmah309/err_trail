#[cfg(feature = "tracing")]
#[cfg(test)]
mod tracing {
    use err_trail::{ErrContext, NoneContext};
    use flaky_test::flaky_test;
    use tracing_test::traced_test;

    #[traced_test]
    #[flaky_test]
    fn test_error() {
        let result: Result<(), &str> = Err("error");
        let _ = result.log_error_msg("An error occurred");

        assert!(logs_contain("An error occurred"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_warn() {
        let result: Result<(), &str> = Err("warning");
        let _ = result.log_warn_msg("A warning occurred");

        assert!(logs_contain("A warning occurred"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_with_error() {
        let result: Result<(), &str> = Err("error");
        let _ = result.log_error_with(|e| format!("An error occurred: `{}`", e));

        assert!(logs_contain("An error occurred: `error`"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_with_warn() {
        let result: Result<(), &str> = Err("warning");
        let _ = result.log_warn_with(|e| format!("A warning occurred: `{}`", e));

        assert!(logs_contain("A warning occurred: `warning`"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_ok_as_error() {
        let result: Result<(), &str> = Err("consumed error");
        let _ = result.log_error();

        assert!(logs_contain("consumed error"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_ok_as_warn() {
        let result: Result<(), &str> = Err("consumed warning");
        let _ = result.log_warn();

        assert!(logs_contain("consumed warning"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_error() {
        let option: Option<()> = None;
        let _ = option.log_error_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_warn() {
        let option: Option<()> = None;
        let _ = option.log_warn_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_with_error() {
        let option: Option<()> = None;
        let _ = option.log_error_with(|| "Lazy error context");

        assert!(logs_contain("Lazy error context"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_with_warn() {
        let option: Option<()> = None;
        let _ = option.log_warn_with(|| "Lazy warn context");

        assert!(logs_contain("Lazy warn context"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_info() {
        let result: Result<(), &str> = Err("info");
        let _ = result.log_info_msg("An info occurred");

        assert!(logs_contain("An info occurred"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_debug() {
        let result: Result<(), &str> = Err("debug");
        let _ = result.log_debug_msg("A debug occurred");

        assert!(logs_contain("A debug occurred"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_trace() {
        let result: Result<(), &str> = Err("trace");
        let _ = result.log_trace_msg("A trace occurred");

        assert!(logs_contain("A trace occurred"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_with_info() {
        let result: Result<(), &str> = Err("info");
        let _ = result.log_info_with(|e| format!("An info occurred: `{}`", e));

        assert!(logs_contain("An info occurred: `info`"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_with_debug() {
        let result: Result<(), &str> = Err("debug");
        let _ = result.log_debug_with(|e| format!("A debug occurred: `{}`", e));

        assert!(logs_contain("A debug occurred: `debug`"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_with_trace() {
        let result: Result<(), &str> = Err("trace");
        let _ = result.log_trace_with(|e| format!("A trace occurred: `{}`", e));

        assert!(logs_contain("A trace occurred: `trace`"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_info() {
        let option: Option<()> = None;
        let _ = option.log_info_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_debug() {
        let option: Option<()> = None;
        let _ = option.log_debug_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_trace() {
        let option: Option<()> = None;
        let _ = option.log_trace_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_with_info() {
        let option: Option<()> = None;
        let _ = option.log_info_with(|| "Lazy info context");

        assert!(logs_contain("Lazy info context"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_with_debug() {
        let option: Option<()> = None;
        let _ = option.log_debug_with(|| "Lazy debug context");

        assert!(logs_contain("Lazy debug context"));
    }

    #[traced_test]
    #[flaky_test]
    fn test_option_with_trace() {
        let option: Option<()> = None;
        let _ = option.log_trace_with(|| "Lazy trace context");

        assert!(logs_contain("Lazy trace context"));
    }
}

#[cfg(feature = "log")]
#[cfg(test)]
mod log {
    use err_trail::{ErrContext, NoneContext};
    use flaky_test::flaky_test;
    use lazy_static::lazy_static;
    use log::{Level, Metadata, Record};
    use std::sync::{Arc, Mutex};

    struct TestLogger {
        logs: Arc<Mutex<Vec<String>>>,
    }

    impl log::Log for TestLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Trace
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let mut logs = self.logs.lock().unwrap();
                logs.push(format!("{}", record.args()));
            }
        }

        fn flush(&self) {}
    }

    lazy_static! {
        static ref LOGS: Arc<Mutex<Vec<String>>> = {
            let logs = Arc::new(Mutex::new(Vec::new()));
            let test_logger = TestLogger { logs: logs.clone() };

            log::set_boxed_logger(Box::new(test_logger)).unwrap();
            log::set_max_level(log::LevelFilter::Trace);

            logs
        };
    }

    fn logs_contain(expected: &str) -> bool {
        let logs = LOGS.lock().unwrap();
        logs.iter().any(|log| log.contains(expected))
    }

    fn clear_logs() {
        let mut logs = LOGS.lock().unwrap();
        logs.clear();
    }

    #[flaky_test]
    fn test_error() {
        clear_logs();
        let result: Result<(), &str> = Err("error");
        let _ = result.log_error_msg("An error occurred");

        assert!(logs_contain("An error occurred"));
    }

    #[flaky_test]
    fn test_warn() {
        clear_logs();
        let result: Result<(), &str> = Err("warning");
        let _ = result.log_warn_msg("A warning occurred");

        assert!(logs_contain("A warning occurred"));
    }

    #[flaky_test]
    fn test_with_error() {
        clear_logs();
        let result: Result<(), &str> = Err("error");
        let _ = result.log_error_with(|e| format!("An error occurred: `{}`", e));

        assert!(logs_contain("An error occurred: `error`"));
    }

    #[flaky_test]
    fn test_with_warn() {
        clear_logs();
        let result: Result<(), &str> = Err("warning");
        let _ = result.log_warn_with(|e| format!("A warning occurred: `{}`", e));

        assert!(logs_contain("A warning occurred: `warning`"));
    }

    #[flaky_test]
    fn test_ok_as_error() {
        clear_logs();
        let result: Result<(), &str> = Err("consumed error");
        let _ = result.log_error();

        assert!(logs_contain("consumed error"));
    }

    #[flaky_test]
    fn test_ok_as_warn() {
        clear_logs();
        let result: Result<(), &str> = Err("consumed warning");
        let _ = result.log_warn();

        assert!(logs_contain("consumed warning"));
    }

    #[flaky_test]
    fn test_option_error() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_error_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[flaky_test]
    fn test_option_warn() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_warn_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[flaky_test]
    fn test_option_with_error() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_error_with(|| "Lazy error context");

        assert!(logs_contain("Lazy error context"));
    }

    #[flaky_test]
    fn test_option_with_warn() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_warn_with(|| "Lazy warn context");

        assert!(logs_contain("Lazy warn context"));
    }

    #[flaky_test]
    fn test_info() {
        clear_logs();
        let result: Result<(), &str> = Err("info");
        let _ = result.log_info_msg("An info occurred");

        assert!(logs_contain("An info occurred"));
    }

    #[flaky_test]
    fn test_debug() {
        clear_logs();
        let result: Result<(), &str> = Err("debug");
        let _ = result.log_debug_msg("A debug occurred");

        assert!(logs_contain("A debug occurred"));
    }

    #[flaky_test]
    fn test_trace() {
        clear_logs();
        let result: Result<(), &str> = Err("trace");
        let _ = result.log_trace_msg("A trace occurred");

        assert!(logs_contain("A trace occurred"));
    }

    #[flaky_test]
    fn test_with_info() {
        clear_logs();
        let result: Result<(), &str> = Err("info");
        let _ = result.log_info_with(|e| format!("An info occurred: `{}`", e));

        assert!(logs_contain("An info occurred: `info`"));
    }

    #[flaky_test]
    fn test_with_debug() {
        clear_logs();
        let result: Result<(), &str> = Err("debug");
        let _ = result.log_debug_with(|e| format!("A debug occurred: `{}`", e));

        assert!(logs_contain("A debug occurred: `debug`"));
    }

    #[flaky_test]
    fn test_with_trace() {
        clear_logs();
        let result: Result<(), &str> = Err("trace");
        let _ = result.log_trace_with(|e| format!("A trace occurred: `{}`", e));

        assert!(logs_contain("A trace occurred: `trace`"));
    }

    #[flaky_test]
    fn test_option_info() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_info_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[flaky_test]
    fn test_option_debug() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_debug_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[flaky_test]
    fn test_option_trace() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_trace_msg("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[flaky_test]
    fn test_option_with_info() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_info_with(|| "Lazy info context");

        assert!(logs_contain("Lazy info context"));
    }

    #[flaky_test]
    fn test_option_with_debug() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_debug_with(|| "Lazy debug context");

        assert!(logs_contain("Lazy debug context"));
    }

    #[flaky_test]
    fn test_option_with_trace() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.log_trace_with(|| "Lazy trace context");

        assert!(logs_contain("Lazy trace context"));
    }
}
