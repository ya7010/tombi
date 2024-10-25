use crate::server::state::{ServerState, State};

pub fn handle_shutdown(_state: State<ServerState>, _params: ()) -> Result<(), anyhow::Error> {
    tracing::info!("Server shutting down");

    Ok(())
}
