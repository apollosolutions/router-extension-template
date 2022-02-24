use anyhow::{ensure, Context, Result};
use apollo_router::configuration::Configuration;
use apollo_router::{ApolloRouterBuilder, GLOBAL_ENV_FILTER};
use apollo_router::{ConfigurationKind, SchemaKind, ShutdownKind, State};
use clap::Parser;
use futures::StreamExt;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = String::from("info"))]
    log_level: String,

    #[clap(short, long = "config", parse(from_os_str))]
    configuration_path: Option<PathBuf>,

    #[clap(short, long = "supergraph", parse(from_os_str))]
    supergraph_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Args::parse();
    let filter_level = args.log_level.to_string();
    let env_filter = tracing_subscriber::EnvFilter::try_new(&filter_level).unwrap();
    tracing_subscriber::fmt::fmt()
        .with_env_filter(env_filter)
        .init();

    GLOBAL_ENV_FILTER.set(filter_level).unwrap();

    let current_directory = std::env::current_dir()?;

    let ot_propagator = opentelemetry::sdk::propagation::TraceContextPropagator::new();
    opentelemetry::global::set_text_map_propagator(ot_propagator);

    let router_configuration = args
        .configuration_path
        .as_ref()
        .map(|p| {
            let p = if p.is_relative() {
                current_directory.join(p)
            } else {
                p.to_path_buf()
            };

            tracing::info!("Router Config Path: {:?}", p);

            ConfigurationKind::File {
                path: p,
                watch: false,
                delay: None,
            }
        })
        .unwrap_or_else(|| ConfigurationKind::Instance(Configuration::builder().build().boxed()));

    let supergraph_schema = args
        .supergraph_path
        .as_ref()
        .map(|p| {
            let p = if p.is_relative() {
                current_directory.join(p)
            } else {
                p.to_path_buf()
            };

            tracing::info!("Supergraph Schema Path: {:?}", p);

            SchemaKind::File {
                path: p,
                watch: false,
                delay: None,
            }
        })
        .unwrap();

    let server = ApolloRouterBuilder::default()
        .configuration(router_configuration)
        .schema(supergraph_schema)
        .shutdown(ShutdownKind::CtrlC)
        .build();

    let mut handle = server.serve();

    handle
        .state_receiver()
        .for_each(|state| {
            match state {
                State::Startup => tracing::info!("Starting Apollo Router"),
                State::Running { address, .. } => tracing::info!("Listening on {}  🚀", address),
                State::Stopped => tracing::info!("Stopped"),
                State::Errored => tracing::info!("Stopped with error"),
            }
            futures::future::ready(())
        })
        .await;

    if let Err(err) = handle.await {
        tracing::error!("{:?}", err);
        return Err(err.into());
    }

    Ok(())
}
