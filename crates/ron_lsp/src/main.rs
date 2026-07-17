mod binding;
mod convert;
mod outline;
mod source;
mod workspace;

use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use lsp_server::Connection;
use lsp_server::Message;
use lsp_server::Request;
use lsp_server::RequestId;
use lsp_server::Response;
use lsp_types::DidChangeTextDocumentParams;
use lsp_types::DidOpenTextDocumentParams;
use lsp_types::DocumentHighlight;
use lsp_types::DocumentHighlightParams;
use lsp_types::DocumentSymbolParams;
use lsp_types::DocumentSymbolResponse;
use lsp_types::GotoDefinitionParams;
use lsp_types::GotoDefinitionResponse;
use lsp_types::Hover;
use lsp_types::HoverContents;
use lsp_types::HoverParams;
use lsp_types::HoverProviderCapability;
use lsp_types::InitializeParams;
use lsp_types::Location as LspLocation;
use lsp_types::MarkupContent;
use lsp_types::MarkupKind;
use lsp_types::OneOf;
use lsp_types::ReferenceParams;
use lsp_types::ServerCapabilities;
use lsp_types::SymbolInformation;
use lsp_types::SymbolKind as LspSymbolKind;
use lsp_types::TextDocumentSyncCapability;
use lsp_types::TextDocumentSyncKind;
use lsp_types::Uri;
use lsp_types::WorkspaceSymbolParams;
use lsp_types::WorkspaceSymbolResponse;
use lsp_types::notification::DidChangeTextDocument;
use lsp_types::notification::DidOpenTextDocument;
use lsp_types::notification::Notification as _;
use lsp_types::request::DocumentHighlightRequest;
use lsp_types::request::DocumentSymbolRequest;
use lsp_types::request::GotoDefinition;
use lsp_types::request::HoverRequest;
use lsp_types::request::References;
use lsp_types::request::Request as _;
use lsp_types::request::WorkspaceSymbolRequest;

use crate::binding::document_highlights;
use crate::workspace::SymbolKind;
use crate::workspace::WorkspaceIndex;

/// JSON-RPC "method not found" error code.
const METHOD_NOT_FOUND: i32 = -32601;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    eprintln!("deckmaste ron_lsp starting");
    let (connection, io_threads) = Connection::stdio();
    let capabilities = serde_json::to_value(server_capabilities())?;
    let init = connection.initialize(capabilities)?;
    let params: InitializeParams = serde_json::from_value(init)?;

    let root = workspace_root(&params);
    let mut server = Server::new(root.as_deref());
    server.run(&connection)?;

    // Drop `connection` (and its writer-channel sender) before joining. Otherwise
    // the writer thread never sees the channel close and `io_threads.join()` hangs,
    // leaving the process alive after the client's `exit`.
    drop(connection);
    io_threads.join()?;
    Ok(())
}

fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        definition_provider: Some(OneOf::Left(true)),
        document_highlight_provider: Some(OneOf::Left(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        ..ServerCapabilities::default()
    }
}

#[expect(
    deprecated,
    reason = "root_uri is the pre-workspaceFolders fallback the spec still requires"
)]
fn workspace_root(params: &InitializeParams) -> Option<PathBuf> {
    if let Some(folders) = &params.workspace_folders
        && let Some(first) = folders.first()
    {
        return uri_to_path(&first.uri);
    }
    params.root_uri.as_ref().and_then(uri_to_path)
}

fn uri_to_path(uri: &Uri) -> Option<PathBuf> {
    let path = uri.as_str().strip_prefix("file://")?;
    Some(PathBuf::from(percent_decode(path)))
}

struct Server {
    index: Option<WorkspaceIndex>,
    documents: HashMap<Uri, String>,
}

impl Server {
    fn new(root: Option<&Path>) -> Self {
        Self {
            index: root.map(WorkspaceIndex::build),
            documents: HashMap::new(),
        }
    }

    fn run(&mut self, connection: &Connection) -> Result<(), Box<dyn Error + Sync + Send>> {
        for message in &connection.receiver {
            match message {
                Message::Request(request) => {
                    if connection.handle_shutdown(&request)? {
                        return Ok(());
                    }
                    if let Some(response) = self.handle_request(request) {
                        connection.sender.send(Message::Response(response))?;
                    }
                }
                Message::Notification(notification) => self.handle_notification(&notification),
                Message::Response(_) => {}
            }
        }
        Ok(())
    }

    fn handle_request(&mut self, request: Request) -> Option<Response> {
        let id = request.id.clone();
        match request.method.as_str() {
            GotoDefinition::METHOD => {
                let (id, params) = cast::<GotoDefinition>(request)?;
                Some(ok(id, self.definition(&params)))
            }
            DocumentHighlightRequest::METHOD => {
                let (id, params) = cast::<DocumentHighlightRequest>(request)?;
                Some(ok(id, self.highlights(&params)))
            }
            WorkspaceSymbolRequest::METHOD => {
                let (id, params) = cast::<WorkspaceSymbolRequest>(request)?;
                Some(ok(id, self.workspace_symbols(&params)))
            }
            HoverRequest::METHOD => {
                let (id, params) = cast::<HoverRequest>(request)?;
                Some(ok(id, self.hover(&params)))
            }
            DocumentSymbolRequest::METHOD => {
                let (id, params) = cast::<DocumentSymbolRequest>(request)?;
                Some(ok(id, self.document_symbol(&params)))
            }
            References::METHOD => {
                let (id, params) = cast::<References>(request)?;
                Some(ok(id, self.references(&params)))
            }
            _ => Some(Response::new_err(
                id,
                METHOD_NOT_FOUND,
                "method not found".to_owned(),
            )),
        }
    }

    fn handle_notification(&mut self, notification: &lsp_server::Notification) {
        match notification.method.as_str() {
            DidOpenTextDocument::METHOD => {
                if let Ok(params) =
                    serde_json::from_value::<DidOpenTextDocumentParams>(notification.params.clone())
                {
                    self.documents
                        .insert(params.text_document.uri, params.text_document.text);
                }
            }
            DidChangeTextDocument::METHOD => {
                if let Ok(params) = serde_json::from_value::<DidChangeTextDocumentParams>(
                    notification.params.clone(),
                ) && let Some(change) = params.content_changes.into_iter().last()
                {
                    self.documents.insert(params.text_document.uri, change.text);
                }
            }
            _ => {}
        }
    }

    fn definition(&self, params: &GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let pos = &params.text_document_position_params;
        let text = self.documents.get(&pos.text_document.uri)?;
        let name = source::word_at(text, convert::src_position(pos.position))?;
        let locations: Vec<_> = self
            .index
            .as_ref()?
            .definitions(name)
            .iter()
            .filter_map(workspace::Location::to_lsp)
            .collect();
        (!locations.is_empty()).then_some(GotoDefinitionResponse::Array(locations))
    }

    fn highlights(&self, params: &DocumentHighlightParams) -> Option<Vec<DocumentHighlight>> {
        let pos = &params.text_document_position_params;
        let text = self.documents.get(&pos.text_document.uri)?;
        Some(document_highlights(
            text,
            convert::src_position(pos.position),
        ))
    }

    #[expect(
        deprecated,
        reason = "SymbolInformation::deprecated is a required field with no non-deprecated form"
    )]
    fn workspace_symbols(&self, params: &WorkspaceSymbolParams) -> Option<WorkspaceSymbolResponse> {
        let index = self.index.as_ref()?;
        let infos: Vec<SymbolInformation> = index
            .search(&params.query)
            .into_iter()
            .filter_map(|symbol| {
                Some(SymbolInformation {
                    name: symbol.name.clone(),
                    kind: lsp_symbol_kind(symbol.kind),
                    tags: None,
                    deprecated: None,
                    location: symbol.location.to_lsp()?,
                    container_name: Some(symbol.container.clone()),
                })
            })
            .collect();
        Some(WorkspaceSymbolResponse::Flat(infos))
    }

    fn references(&mut self, params: &ReferenceParams) -> Option<Vec<LspLocation>> {
        let pos = &params.text_document_position;
        let name = {
            let text = self.documents.get(&pos.text_document.uri)?;
            source::word_at(text, convert::src_position(pos.position))?.to_owned()
        };
        let index = self.index.as_mut()?;
        let mut locations: Vec<LspLocation> = index
            .references(&name)
            .iter()
            .filter_map(workspace::Location::to_lsp)
            .collect();
        if params.context.include_declaration
            && let Some(declaration) = index
                .symbol(&name)
                .and_then(|symbol| symbol.location.to_lsp())
        {
            locations.push(declaration);
        }
        Some(locations)
    }

    fn document_symbol(&self, params: &DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
        let text = self.documents.get(&params.text_document.uri)?;
        Some(DocumentSymbolResponse::Nested(outline::document_symbols(
            text,
        )))
    }

    fn hover(&self, params: &HoverParams) -> Option<Hover> {
        let pos = &params.text_document_position_params;
        let text = self.documents.get(&pos.text_document.uri)?;
        let name = source::word_at(text, convert::src_position(pos.position))?;
        let symbol = self.index.as_ref()?.symbol(name)?;
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: hover_markdown(&symbol.name, &symbol.container, symbol.detail.as_deref()),
            }),
            range: None,
        })
    }
}

fn hover_markdown(name: &str, container: &str, detail: Option<&str>) -> String {
    let body = detail
        .map(|detail| format!("\n\n```\n{detail}\n```"))
        .unwrap_or_default();
    format!("**{name}**  \n`{container}`{body}")
}

fn lsp_symbol_kind(kind: SymbolKind) -> LspSymbolKind {
    match kind {
        SymbolKind::Card => LspSymbolKind::STRUCT,
        SymbolKind::Macro => LspSymbolKind::FUNCTION,
        SymbolKind::Keyword => LspSymbolKind::KEY,
        SymbolKind::AbilityWord => LspSymbolKind::CONSTANT,
        SymbolKind::RustType => LspSymbolKind::CLASS,
    }
}

fn cast<R>(request: Request) -> Option<(RequestId, R::Params)>
where
    R: lsp_types::request::Request,
{
    request.extract::<R::Params>(R::METHOD).ok()
}

fn ok<T: serde::Serialize>(id: RequestId, result: Option<T>) -> Response {
    Response::new_ok(id, result)
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut result = Vec::with_capacity(value.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(byte) = u8::from_str_radix(&value[i + 1..i + 3], 16)
        {
            result.push(byte);
            i += 3;
        } else {
            result.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&result).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_file_uri() {
        assert_eq!(
            uri_to_path(&"file:///tmp/a%20b".parse::<Uri>().unwrap()),
            Some(PathBuf::from("/tmp/a b"))
        );
    }

    #[test]
    fn hover_markdown_for_macro() {
        let md = hover_markdown(
            "SacrificeThis",
            "builtin/macros",
            Some("template: Sacrifice this permanent\nkinds: [CostComponent]"),
        );
        assert!(md.contains("SacrificeThis"));
        assert!(md.contains("Sacrifice this permanent"));
        assert!(md.contains("builtin/macros"));
    }
}
