use lazy_static::lazy_static;

lazy_static! {
    pub static ref RUN_TIME: tokio::runtime::Runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();
}
