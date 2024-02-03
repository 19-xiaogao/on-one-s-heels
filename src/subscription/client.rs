use std::sync::Arc;
use ethers::prelude::{ Provider, ProviderError, Ws};
pub async fn create_client  (wss: &str) -> Result<Arc<Provider<Ws>>,ProviderError> {
    let provider = Provider::<Ws>::connect(wss).await?;
    let client = Arc::new(provider);
    Ok(client)
}