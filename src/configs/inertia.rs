use crate::{
    configs::app::APP_CONFIG,
    configs::env::RustEnv,
    error::{IntoSamambaiaError, SamambaiaError},
};

use super::vite::initialize_vite;
use actix_web::{web::Redirect, HttpRequest};
use inertia_rust::{
    hashmap, template_resolvers::ViteHBSTemplateResolver, Inertia, InertiaConfig, InertiaError,
    InertiaFacade, InertiaVersion, IntoInertiaError,
};
use std::io;

pub async fn initialize_inertia() -> Result<Inertia, io::Error> {
    let vite = initialize_vite().await;
    let version = vite.get_hash().unwrap_or("development").to_string();

    let resolver = ViteHBSTemplateResolver::builder()
        .set_vite(vite)
        .set_dev_mode(APP_CONFIG.rust_env.eq(&RustEnv::Development))
        .set_template_path("www/root.hbs")
        .build()
        .map_err(InertiaError::to_io_error)?;

    let url = Box::leak(
        format!(
            "{}://{}:{}",
            if APP_CONFIG.https { "https" } else { "http" },
            APP_CONFIG.domain,
            APP_CONFIG.port
        )
        .into_boxed_str(),
    );

    let mut inertia_config = InertiaConfig::builder()
        .set_url(url)
        .set_version(InertiaVersion::Literal(version))
        .set_template_resolver(Box::new(resolver))
        .build();

    if APP_CONFIG.rust_env == RustEnv::Production {
        inertia_config.with_ssr = true;
    }

    Inertia::new(inertia_config)
}

impl IntoSamambaiaError for InertiaError {
    fn into_samambaia_error(self) -> SamambaiaError {
        SamambaiaError::internal_err().with_message(self.get_cause())
    }
}

impl IntoInertiaError for SamambaiaError {
    fn into_inertia_error(self) -> InertiaError {
        InertiaError::RenderError(self.to_string())
    }
}

pub trait IntoInertiaRedirect<TRequest, TRedirect> {
    fn into_inertia_redirect(self, req: &TRequest) -> TRedirect;
}

impl IntoInertiaRedirect<HttpRequest, Redirect> for SamambaiaError {
    fn into_inertia_redirect(self, req: &HttpRequest) -> Redirect {
        Inertia::back_with_errors(
            req,
            hashmap![
                "error" => self.get_message().into()
            ],
        )
    }
}

impl<T: IntoSamambaiaError> IntoInertiaRedirect<HttpRequest, Redirect> for T {
    fn into_inertia_redirect(self, req: &HttpRequest) -> Redirect {
        Inertia::back_with_errors(
            req,
            hashmap![
                "error" => self.into_samambaia_error().get_message().into()
            ],
        )
    }
}
