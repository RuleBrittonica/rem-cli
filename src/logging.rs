use flexi_logger::{
    Logger,
    FileSpec,
    Criterion,
    Naming,
    Cleanup,
    Age,
    DeferredNow,
    Record

};

pub fn init_logging() {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("rem-cli-log")
                .suffix("log")
        )
        .rotate(
            Criterion::Age(Age::Day), // Rotate Daily
            Naming::Timestamps, // Use timestamps
            Cleanup::KeepLogFiles(60)
        )
        .format_for_files(format_log_with_timestamp)
        .start()
        .unwrap_or_else(|e| panic!("Logger intialization failed: {}", e));
}


// Custom log formatter function that includes the timestamp.
fn format_log_with_timestamp(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    write!(
        w,
        "{} [{}] - {}",
        now.now().format("%Y-%m-%d %H:%M:%S"), // Timestamp in "YYYY-MM-DD HH:MM:SS" format
        record.level(),
        record.args()
    )
}