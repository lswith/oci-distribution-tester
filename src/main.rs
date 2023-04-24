//! # Load test an OCI compliant registry
use anyhow::anyhow;
use clap::{Parser, Subcommand};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

/// The CLI Command.
#[derive(Debug, Parser)]
#[command(name = clap::crate_name!())]
#[command(version = clap::crate_version!())]
#[command(author = clap::crate_authors!())]
#[command(about = clap::crate_description!())]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(hide = true)]
    MarkdownHelp,
    /// Pushes a generated OCI image to an OCI distribution server.
    #[command()]
    PushImages {
        /// The amount of images to push.
        #[arg(short, long, value_name = "COUNT", default_value_t = 1)]
        count: usize,

        /// The OCI distribution server url.
        #[arg(
            long,
            value_name = "REGISTRY_URL",
            default_value = "http://localhost:6000"
        )]
        reg_url: String,

        /// The user+password to authenticate against the OCI distribution server in the format user:password.
        #[arg(long, value_name = "REGISTRY_USERPASS")]
        reg_userpass: Option<String>,

        /// The image namespace. This will be used to generate the complete image.
        /// Example: <namespace>/<image>-<count>:<tag>
        #[arg(short, long, value_name = "IMAGE_NAMESPACE", default_value = "test")]
        namespace: String,

        /// The image name. This will be used to generate the complete image.
        /// Example: <namespace>/<image>-<count>:<tag>
        #[arg(short, long, value_name = "IMAGE", default_value = "this")]
        image: String,

        /// The image tag. This will be used to generate the complete image.
        /// Example: <namespace>/<image>-<count>:<tag>
        #[arg(short, long, value_name = "IMAGE_TAG", default_value = "latest")]
        tag: String,
    },

    /// Pulls OCI images from an OCI distribution server.
    #[command()]
    PullImages {
        /// The amount of images to pull.
        #[arg(short, long, value_name = "COUNT", default_value_t = 1)]
        count: usize,

        /// The OCI distribution server url.
        #[arg(
            long,
            value_name = "REGISTRY_URL",
            default_value = "https://index.docker.io"
        )]
        reg_url: String,

        /// The user+password to authenticate against the OCI distribution server in the format user:password.
        #[arg(long, value_name = "REGISTRY_USERPASS")]
        reg_userpass: Option<String>,

        /// The image to pull.
        #[arg(short, long, value_name = "IMAGE", default_value = "alpine:latest")]
        image: String,
    },

    PushImageList {
        /// The OCI distribution server url.
        #[arg(
            long,
            value_name = "REGISTRY_URL",
            default_value = "http://localhost:6000"
        )]
        reg_url: String,

        /// The user+password to authenticate against the OCI distribution server in the format user:password.
        #[arg(long, value_name = "REGISTRY_USERPASS")]
        reg_userpass: Option<String>,

        /// Where to push the image list.
        #[arg(short, long, value_name = "IMAGE", default_value = "test/this:cache")]
        image: String,
    },
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    if args.verbose {
        filter = EnvFilter::from_default_env().add_directive(LevelFilter::DEBUG.into());
    }

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
        .map_err(|e| {
            eprintln!("Failed to initialize tracing: {e}");
            anyhow!(e)
        })?;

    match args.command {
        Commands::MarkdownHelp => {
            clap_markdown::print_help_markdown::<Cli>();
            Ok(())
        }
        Commands::PullImages {
            reg_url,
            count,
            reg_userpass,
            image,
        } => oci_tester::pull_images(reg_url, count, reg_userpass, image).await,
        Commands::PushImages {
            reg_url,
            count,
            reg_userpass,
            namespace,
            image,
            tag,
        } => oci_tester::push_images(reg_url, count, reg_userpass, namespace, image, tag).await,
        Commands::PushImageList {
            reg_url,
            reg_userpass,
            image,
        } => oci_tester::push_image_index(reg_url, reg_userpass, image).await,
    }
}
