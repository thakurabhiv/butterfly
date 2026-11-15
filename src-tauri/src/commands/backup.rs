use crate::state::AppState;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use tauri::{AppHandle, Manager, Runtime, Wry};
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_store::StoreExt;
use yup_oauth2::authenticator_delegate::InstalledFlowDelegate;
use yup_oauth2::storage::{TokenInfo, TokenStorage};
use yup_oauth2::{ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

struct TauriBrowserDelegate {
    app_handle: tauri::AppHandle,
}

impl InstalledFlowDelegate for TauriBrowserDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        _need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        let handle = self.app_handle.clone();
        let url_owned = url.to_string();
        println!("#### URL:{}", url);

        Box::pin(async move {
            // Opens the URL in the user's default browser
            handle
                .opener()
                .open_url(&url_owned, None::<&str>)
                .map_err(|e| format!("Failed to open browser: {}", e))?;

            // Return empty string because HTTPRedirect receives the code via local server
            Ok(String::new())
        })
    }
}

pub struct TauriTokenStorage<R: Runtime = Wry> {
    app: tauri::AppHandle<R>,
}

impl<R: Runtime> TauriTokenStorage<R> {
    pub fn new(app: AppHandle<R>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl TokenStorage for TauriTokenStorage<Wry> {
    async fn set(&self, scopes: &[&str], token: TokenInfo) -> anyhow::Result<()> {
        let store = self
            .app
            .store("oauth_tokens.json")
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let key = scopes.join(" ");
        let value = serde_json::to_value(&token)?;

        store.set(key, value);
        store.save().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(())
    }

    async fn get(&self, scopes: &[&str]) -> Option<TokenInfo> {
        let store = self.app.store("oauth_tokens.json").ok()?;
        let key = scopes.join(" ");
        let value = store.get(&key)?;

        serde_json::from_value::<TokenInfo>(value).ok()
    }
}

// method  for open google auhenticator
// and caceh refresh token
#[tauri::command]
pub async fn authenticate_google_account_for_backup(
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let (client_id, client_secret) = {
        let state = handle.state::<AppState>();
        let app_state = state.lock().unwrap();
        (
            app_state.backup.client_id.clone(),
            app_state.backup.client_secret.clone(),
        )
    };

    // if not present return err
    if client_id.trim().is_empty() || client_secret.trim().is_empty() {
        return Err("Client id and client secret both need to be configured".to_string());
    }

    // construct the application secret
    let secret = ApplicationSecret {
        client_id,
        client_secret,
        token_uri: "https://oauth2.googleapis.com/token".to_string(),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        redirect_uris: vec!["http://localhost".to_string()],
        ..Default::default()
    };

    // build authenticator (opens browser once, cache refresh token)
    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .with_storage(Box::new(TauriTokenStorage::new(handle.clone())))
        .flow_delegate(Box::new(TauriBrowserDelegate {
            app_handle: handle.clone(),
        }))
        .build()
        .await
        .map_err(|err| format!("Google authentication failed: {}", err))?;

    // 5. Trigger the browser login & save tokens to tokencache.json
    let scopes = &["https://www.googleapis.com/auth/drive.file"];
    auth.token(scopes)
        .await
        .map_err(|err| format!("Google authentication failed: {}", err))?;

    Ok(())
}
