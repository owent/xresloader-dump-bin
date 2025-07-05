pub struct Logger {
    stdout: Box<env_logger::Logger>,
    stderr: Box<env_logger::Logger>,
}

impl Logger {
    pub fn new(max_log_level: log::LevelFilter) -> Self {
        Self {
            stdout: Box::new(
                env_logger::Builder::new()
                    .filter_level(max_log_level)
                    .format_level(false)
                    .format_module_path(false)
                    .format_target(false)
                    .format_timestamp(None)
                    .target(env_logger::Target::Stdout)
                    .parse_default_env()
                    .build(),
            ),
            stderr: Box::new(
                env_logger::Builder::new()
                    .filter_level(log::LevelFilter::Warn)
                    .format_level(false)
                    .format_module_path(false)
                    .format_target(false)
                    .format_timestamp(None)
                    .target(env_logger::Target::Stderr)
                    .parse_default_env()
                    .build(),
            ),
        }
    }

    pub fn init(self) -> Result<(), log::SetLoggerError> {
        let max_level = self.stdout.filter();
        let r = log::set_boxed_logger(Box::new(self));

        if r.is_ok() {
            log::set_max_level(max_level);
        }

        r
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.stdout.enabled(metadata) || self.stderr.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        if self.stderr.enabled(record.metadata()) {
            self.stderr.log(record);
        } else if self.stdout.enabled(record.metadata()) {
            self.stdout.log(record);
        }
    }

    fn flush(&self) {
        self.stdout.flush();
        self.stderr.flush();
    }
}
