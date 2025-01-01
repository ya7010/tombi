use tower_lsp::lsp_types::DidChangeWatchedFilesParams;

pub async fn handle_did_change_watched_files(
    DidChangeWatchedFilesParams { .. }: DidChangeWatchedFilesParams,
) {
}
