

#[derive(Debug)]
pub struct CriRuntimePlugin {
    cri_client: Box<str>,
    runner: tokio::runtime::Runtime,
}