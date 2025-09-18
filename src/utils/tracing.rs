use poem::{FromRequest, Request, RequestBody};
use tracing_appender::non_blocking;
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

pub fn init_tracing(log_level: &str) {
    color_eyre::install().expect("Failed to install color_eyre");

    let (non_blocking, _guard) = non_blocking(std::io::stdout());

    registry()
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(false)
                .with_level(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(EnvFilter::new(log_level))
        .with(ErrorLayer::default())
        .init();

    std::mem::forget(_guard);
}

#[derive(Clone)]
pub struct RequestContext {
    pub request_id: String,
}

impl<'a> FromRequest<'a> for RequestContext {
    async fn from_request(req: &'a Request, _: &mut RequestBody) -> poem::Result<Self> {
        let request_id = req
            .header("x-request-id")
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        Ok(RequestContext { request_id })
    }
}
