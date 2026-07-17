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
/// JSON-RPC "invalid params" error code.
const INVALID_PARAMS: i32 = -32602;

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
                    let response = self.handle_request(request);
                    connection.sender.send(Message::Response(response))?;
                }
                Message::Notification(notification) => self.handle_notification(&notification),
                Message::Response(_) => {}
            }
        }
        Ok(())
    }

    fn handle_request(&mut self, request: Request) -> Response {
        let id = request.id.clone();
        match request.method.as_str() {
            GotoDefinition::METHOD => {
                dispatch::<GotoDefinition, _>(request, id, |params| self.definition(&params))
            }
            DocumentHighlightRequest::METHOD => {
                dispatch::<DocumentHighlightRequest, _>(request, id, |params| {
                    self.highlights(&params)
                })
            }
            WorkspaceSymbolRequest::METHOD => {
                dispatch::<WorkspaceSymbolRequest, _>(request, id, |params| {
                    self.workspace_symbols(&params)
                })
            }
            HoverRequest::METHOD => {
                dispatch::<HoverRequest, _>(request, id, |params| self.hover(&params))
            }
            DocumentSymbolRequest::METHOD => {
                dispatch::<DocumentSymbolRequest, _>(request, id, |params| {
                    self.document_symbol(&params)
                })
            }
            References::METHOD => {
                dispatch::<References, _>(request, id, |params| self.references(&params))
            }
            _ => Response::new_err(id, METHOD_NOT_FOUND, "method not found".to_owned()),
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
            _ => return,
        }
        // The reverse index is a snapshot of the corpus; drop it whenever a
        // document opens or changes so the next `references` rebuilds against
        // current files rather than silently serving a stale answer.
        if let Some(index) = self.index.as_mut() {
            index.invalidate_uses();
        }
    }

    fn definition(&self, params: &GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let pos = &params.text_document_position_params;
        let text = self.documents.get(&pos.text_document.uri)?;
        let index = self.index.as_ref()?;
        let name = pick_name(index, text, convert::src_position(pos.position))?;
        let locations: Vec<_> = index
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
        // Resolve the name under an immutable borrow first; `references` needs a
        // mutable one to build its cache.
        let name = {
            let text = self.documents.get(&pos.text_document.uri)?;
            let index = self.index.as_ref()?;
            pick_name(index, text, convert::src_position(pos.position))?.to_owned()
        };
        let index = self.index.as_mut()?;
        let mut locations: Vec<LspLocation> = index
            .references(&name)
            .iter()
            .filter_map(workspace::Location::to_lsp)
            .collect();
        if params.context.include_declaration {
            // Every declaration, not just one — a name can be defined more than once.
            locations.extend(
                index
                    .definitions(&name)
                    .iter()
                    .filter_map(workspace::Location::to_lsp),
            );
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
        let index = self.index.as_ref()?;
        let name = pick_name(index, text, convert::src_position(pos.position))?;
        // A name can have several definitions (kind-scoped macros); show them all
        // rather than an arbitrary one.
        let symbols = index.symbols_named(name);
        if symbols.is_empty() {
            return None;
        }
        let value = symbols
            .iter()
            .map(|symbol| hover_markdown(&symbol.name, &symbol.container, symbol.detail.as_deref()))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value,
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

/// Extract `R`'s params and answer with `handler`'s result. Malformed params
/// get a JSON-RPC error rather than silence — a dropped response reads as a
/// hang to a conforming client.
fn dispatch<R, T>(
    request: Request,
    id: RequestId,
    handler: impl FnOnce(R::Params) -> Option<T>,
) -> Response
where
    R: lsp_types::request::Request,
    T: serde::Serialize,
{
    match request.extract::<R::Params>(R::METHOD) {
        Ok((id, params)) => Response::new_ok(id, handler(params)),
        Err(_) => Response::new_err(id, INVALID_PARAMS, "invalid params".to_owned()),
    }
}

/// The symbol name at `position`: the bare identifier under the cursor, or —
/// when that isn't a known symbol — the enclosing string literal, so multi-word
/// card names (`"Faramir, Steward of Gondor"`) resolve too.
fn pick_name<'a>(
    index: &WorkspaceIndex,
    text: &'a str,
    position: source::Position,
) -> Option<&'a str> {
    let word = source::word_at(text, position);
    let quoted = source::quoted_string_at(text, position);
    if let Some(word) = word
        && index.knows(word)
    {
        return Some(word);
    }
    if let Some(quoted) = quoted
        && index.knows(quoted)
    {
        return Some(quoted);
    }
    word.or(quoted)
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
