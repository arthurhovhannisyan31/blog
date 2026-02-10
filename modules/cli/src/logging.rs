pub fn init_logging() {
  tracing_subscriber::fmt()
    .with_line_number(true)
    .with_thread_ids(true)
    .init();
}
