use vite_rust::{Vite, ViteConfig};

pub async fn initialize_vite() -> Vite {
    let vite_config = ViteConfig::default()
        .set_manifest_path("public/bundle/manifest.json")
        .set_entrypoints(vec!["www/app.tsx"])
        .set_prefix("bundle")
        .set_heart_beat_retries_limit(1);

    match Vite::new(vite_config).await {
        Err(err) => panic!("{}", err),
        Ok(vite) => vite,
    }
}
