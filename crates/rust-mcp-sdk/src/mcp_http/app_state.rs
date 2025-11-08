use crate::mcp_traits::McpServerHandler;
use crate::session_store::SessionStore;
use crate::{id_generator::FastIdGenerator, mcp_traits::IdGenerator, schema::InitializeResult};
use rust_mcp_transport::event_store::EventStore;
use rust_mcp_transport::{SessionId, TransportOptions};
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;

/// Application state struct for the Hyper ser
///
/// Holds shared, thread-safe references to session storage, ID generator,
/// server details, handler, ping interval, and transport options.
#[derive(Clone)]
pub struct McpAppState {
    pub session_store: Arc<dyn SessionStore>,
    pub id_generator: Arc<dyn IdGenerator<SessionId>>,
    pub stream_id_gen: Arc<FastIdGenerator>,
    pub server_details: Arc<InitializeResult>,
    pub handler: Arc<dyn McpServerHandler>,
    pub ping_interval: Duration,
    pub transport_options: Arc<TransportOptions>,
    pub enable_json_response: bool,
    /// Event store for resumability support
    /// If provided, resumability will be enabled, allowing clients to reconnect and resume messages
    pub event_store: Option<Arc<dyn EventStore>>,
    /// Session metadata for storing custom headers and context (like X-Thread-ID)
    /// Maps session_id -> (header_name -> header_value)
    pub session_metadata: Arc<RwLock<HashMap<SessionId, HashMap<String, String>>>>,
}

impl McpAppState {
    /// Retrieves a specific header value from session metadata
    ///
    /// # Arguments
    /// * `session_id` - The session identifier
    /// * `header_name` - The name of the header to retrieve (e.g., "X-Thread-ID")
    ///                   Note: Header names are case-insensitive and will be normalized to lowercase
    ///
    /// # Returns
    /// * `Option<String>` - The header value if it exists, None otherwise
    pub async fn get_session_header(&self, session_id: &SessionId, header_name: &str) -> Option<String> {
        let normalized_key = header_name.to_lowercase();
        self.session_metadata
            .read()
            .await
            .get(session_id)
            .and_then(|headers| headers.get(&normalized_key).cloned())
    }

    /// Retrieves all metadata for a given session
    ///
    /// # Arguments
    /// * `session_id` - The session identifier
    ///
    /// # Returns
    /// * `Option<HashMap<String, String>>` - All metadata for the session if it exists
    pub async fn get_session_metadata(&self, session_id: &SessionId) -> Option<HashMap<String, String>> {
        self.session_metadata
            .read()
            .await
            .get(session_id)
            .cloned()
    }
}
