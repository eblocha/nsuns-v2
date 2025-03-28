use anyhow::{Context, Result};
use nsuns_server::{
    auth, log_error, observability::metrics::server as metrics_server,
    observability::tracing::setup::setup_tracing, server, settings::Settings,
};

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new()
        .context("failed to load settings")
        .map_err(log_error!())?;

    let _guard = setup_tracing(&settings)?;

    tokio::try_join!(
        server::run(&settings),
        metrics_server::run(&settings.metrics),
        auth::cleanup::run(&settings)
    )
    .map_err(log_error!())?;

    Ok(())
}
