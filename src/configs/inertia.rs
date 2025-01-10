use crate::{
    env_config::RustEnv,
    error::{DomainError, IntoDomainError},
    ENV_VARS,
};

use super::vite::initialize_vite;
use inertia_rust::{
    template_resolvers::ViteTemplateResolver, Inertia, InertiaConfig, InertiaError, InertiaVersion,
    IntoInertiaError,
};
use std::io;

pub async fn initialize_inertia() -> Result<Inertia, io::Error> {
    let vite = initialize_vite().await;
    let version = vite.get_hash().unwrap_or("development").to_string();
    let resolver = ViteTemplateResolver::new(vite);

    let url = Box::leak(
        format!(
            "{}://{}:{}",
            if ENV_VARS.https { "https" } else { "http" },
            ENV_VARS.domain,
            ENV_VARS.port
        )
        .into_boxed_str(),
    );

    let mut inertia_config = InertiaConfig::builder()
        .set_url(url)
        .set_version(InertiaVersion::Literal(version))
        .set_template_path("www/root.html")
        .set_template_resolver(Box::new(resolver))
        .build();

    if ENV_VARS.rust_env == RustEnv::Production {
        inertia_config.with_ssr = true;
    }

    Inertia::new(inertia_config)
}

impl IntoDomainError for InertiaError {
    fn into_domain_error(self) -> DomainError {
        DomainError::internal_err().with_message(self.get_cause())
    }
}

impl IntoInertiaError for DomainError {
    fn into_inertia_error(self) -> InertiaError {
        InertiaError::RenderError(self.to_string())
    }
}
