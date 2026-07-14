mod binding;
mod source;
mod workspace;

use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use std::io::{self};
use std::path::PathBuf;

use serde_json::Value;
use serde_json::json;

use crate::binding::document_highlights;
use crate::source::Position;
use crate::source::word_at;
use crate::workspace::WorkspaceIndex;

fn main() -> io::Result<()> {
    Server::default().run()
}

#[derive(Default)]
struct Server {
    index: Option<WorkspaceIndex>,
    documents: HashMap<String, String>,
}

impl Server {
    fn run(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut input = stdin.lock();
        let stdout = io::stdout();
        let mut output = stdout.lock();

        while let Some(message) = read_message(&mut input)? {
            if let Some(response) = self.handle(&message) {
                write_message(&mut output, &response)?;
            }
        }
        Ok(())
    }

    fn handle(&mut self, message: &Value) -> Option<Value> {
        let method = message.get("method")?.as_str()?;
        let id = message.get("id").cloned();
        let params = message.get("params").unwrap_or(&Value::Null);

        match method {
            "initialize" => {
                let root = workspace_root(params);
                self.index = root.as_ref().map(|path| WorkspaceIndex::build(path));
                id.map(|id| response(&id, &json!({
                    "capabilities": {
                        "definitionProvider": true,
                        "documentHighlightProvider": true,
                        "textDocumentSync": 1
                    },
                    "serverInfo": { "name": "deckmaste-ron-lsp", "version": env!("CARGO_PKG_VERSION") }
                })))
            }
            "shutdown" => id.map(|id| response(&id, &Value::Null)),
            "textDocument/didOpen" => {
                let doc = &params["textDocument"];
                if let (Some(uri), Some(text)) = (doc["uri"].as_str(), doc["text"].as_str()) {
                    self.documents.insert(uri.to_owned(), text.to_owned());
                }
                None
            }
            "textDocument/didChange" => {
                let uri = params["textDocument"]["uri"].as_str()?;
                let text = params["contentChanges"].as_array()?.last()?["text"].as_str()?;
                self.documents.insert(uri.to_owned(), text.to_owned());
                None
            }
            "textDocument/definition" => id.map(|id| {
                let result = self.definition(params).unwrap_or(Value::Null);
                response(&id, &result)
            }),
            "textDocument/documentHighlight" => id.map(|id| {
                let result = self.highlights(params).unwrap_or(Value::Null);
                response(&id, &result)
            }),
            _ => id.map(|id| error(&id, -32601, "method not found")),
        }
    }

    fn definition(&self, params: &Value) -> Option<Value> {
        let uri = params["textDocument"]["uri"].as_str()?;
        let text = self.documents.get(uri)?;
        let position = Position::from_json(&params["position"])?;
        let name = word_at(text, position)?;
        let locations = self.index.as_ref()?.definitions(name);
        Some(Value::Array(
            locations
                .iter()
                .map(crate::workspace::Location::to_json)
                .collect(),
        ))
    }

    fn highlights(&self, params: &Value) -> Option<Value> {
        let uri = params["textDocument"]["uri"].as_str()?;
        let text = self.documents.get(uri)?;
        let position = Position::from_json(&params["position"])?;
        Some(Value::Array(document_highlights(text, position)))
    }
}

fn workspace_root(params: &Value) -> Option<PathBuf> {
    params["workspaceFolders"]
        .as_array()
        .and_then(|folders| folders.first())
        .and_then(|folder| folder["uri"].as_str())
        .or_else(|| params["rootUri"].as_str())
        .and_then(file_uri_path)
}

fn file_uri_path(uri: &str) -> Option<PathBuf> {
    let path = uri.strip_prefix("file://")?;
    Some(PathBuf::from(percent_decode(path)))
}

fn percent_decode(value: &str) -> String {
    let mut result = Vec::with_capacity(value.len());
    let bytes = value.as_bytes();
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

fn response(id: &Value, result: &Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn error(id: &Value, code: i32, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

fn read_message(input: &mut impl BufRead) -> io::Result<Option<Value>> {
    let mut content_length = None;
    loop {
        let mut header = String::new();
        if input.read_line(&mut header)? == 0 {
            return Ok(None);
        }
        if header == "\r\n" || header == "\n" {
            break;
        }
        if let Some(value) = header.strip_prefix("Content-Length:") {
            content_length = value.trim().parse::<usize>().ok();
        }
    }
    let length = content_length
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing Content-Length"))?;
    let mut body = vec![0; length];
    input.read_exact(&mut body)?;
    serde_json::from_slice(&body)
        .map(Some)
        .map_err(io::Error::other)
}

fn write_message(output: &mut impl Write, message: &Value) -> io::Result<()> {
    let body = serde_json::to_vec(message).map_err(io::Error::other)?;
    write!(output, "Content-Length: {}\r\n\r\n", body.len())?;
    output.write_all(&body)?;
    output.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_file_uri() {
        assert_eq!(
            file_uri_path("file:///tmp/a%20b"),
            Some(PathBuf::from("/tmp/a b"))
        );
    }
}
