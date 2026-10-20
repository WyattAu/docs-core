//! The renderer: CommonMark+GFM to static HTML, with island fences routed
//! through [`crate::island`]. M1 scope is the element set the estate's docs
//! actually use; the rest of the spec (search indexing, versioning, i18n)
//! is the Wave-2 roadmap in the crate README, not silently absent.

use crate::error::DocsError;
use crate::island::Island;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use std::fmt::Write as _;

/// Render Markdown to HTML. Islands in ```` ```island ```` fences become
/// astro-island-compatible custom elements (`base` prefixes component URLs).
///
/// # Errors
/// [`DocsError`] for malformed island fences; Markdown itself never fails —
/// unknown constructs render their inner content (documented M1 behaviour).
pub fn render(markdown: &str, base: &str) -> Result<String, DocsError> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);

    let mut out = String::new();
    let mut headings: Vec<u8> = Vec::new();
    // Open lists, innermost last, with their ordered flags — the closing tag
    // must match the element actually opened, and paragraphs are suppressed
    // only while *some* list is open.
    let mut list_stack: Vec<bool> = Vec::new();
    let mut in_code_block = false;
    let mut parser = Parser::new_ext(markdown, options);

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                // pulldown's `level` Display already renders "h1".."h6".
                headings.push(level as u8);
                let _ = write!(out, "<{level}>");
            }
            Event::End(TagEnd::Heading(_)) => {
                let level = headings.pop().unwrap_or(1);
                let _ = write!(out, "</h{level}>");
            }
            Event::Start(Tag::Paragraph) => {
                if list_stack.is_empty() {
                    out.push_str("<p>");
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if list_stack.is_empty() {
                    out.push_str("</p>");
                }
            }
            Event::Start(Tag::Emphasis) => out.push_str("<em>"),
            Event::End(TagEnd::Emphasis) => out.push_str("</em>"),
            Event::Start(Tag::Strong) => out.push_str("<strong>"),
            Event::End(TagEnd::Strong) => out.push_str("</strong>"),
            Event::Start(Tag::Strikethrough) => out.push_str("<del>"),
            Event::End(TagEnd::Strikethrough) => out.push_str("</del>"),
            Event::Code(code) => {
                let _ = write!(out, "<code>{}</code>", escape_html(&code));
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                let _ = write!(out, "<a href=\"{}\">", escape_attr(&dest_url));
            }
            Event::End(TagEnd::Link) => out.push_str("</a>"),
            Event::Start(Tag::Image {
                dest_url, title, ..
            }) => {
                // The alt text arrives as a Text event between Start and End;
                // capture it into the attribute instead of leaking it into
                // the output as visible body text.
                let mut alt = String::new();
                for inner in parser.by_ref() {
                    match inner {
                        Event::Text(t) => alt.push_str(&t),
                        Event::End(TagEnd::Image) => break,
                        _ => {}
                    }
                }
                let _ = write!(
                    out,
                    "<img src=\"{}\" alt=\"{}\"{}>",
                    escape_attr(&dest_url),
                    escape_attr(&alt),
                    if title.is_empty() {
                        String::new()
                    } else {
                        format!(" title=\"{}\"", escape_attr(&title))
                    }
                );
            }
            Event::End(TagEnd::Image) => {}
            Event::Start(Tag::BlockQuote(_)) => out.push_str("<blockquote>"),
            Event::End(TagEnd::BlockQuote(_)) => out.push_str("</blockquote>"),
            Event::Start(Tag::CodeBlock(kind)) => match kind {
                CodeBlockKind::Fenced(info) if info.starts_with("island") => {
                    // Island props are raw JSON, not prose: consume the fence
                    // body verbatim, emit the custom element, and swallow the
                    // matching End event.
                    let mut body = String::new();
                    for inner in parser.by_ref() {
                        match inner {
                            Event::Text(t) => {
                                body.push_str(&t);
                                break;
                            }
                            Event::End(TagEnd::CodeBlock) => break,
                            _ => {}
                        }
                    }
                    let island = Island::parse(&info, &body)?;
                    out.push_str(&island.to_html(base));
                    let _ = parser.next(); // the End(CodeBlock) we did not consume
                }
                _ => {
                    in_code_block = true;
                    out.push_str("<pre><code>");
                }
            },
            Event::End(TagEnd::CodeBlock) => {
                if in_code_block {
                    in_code_block = false;
                    out.push_str("</code></pre>");
                }
            }
            Event::Start(Tag::List(start)) => {
                let ordered = start.is_some();
                let _ = write!(out, "<{}>", if ordered { "ol" } else { "ul" });
                list_stack.push(ordered);
            }
            Event::End(TagEnd::List(_)) => {
                // Close the element that was actually opened — the event does
                // not carry the ordered flag, so the stack is the source of
                // truth. Empty pop = unbalanced end; emit nothing.
                match list_stack.pop() {
                    Some(true) => out.push_str("</ol>"),
                    Some(false) => out.push_str("</ul>"),
                    None => {}
                }
            }
            Event::Start(Tag::Item) => out.push_str("<li>"),
            Event::End(TagEnd::Item) => out.push_str("</li>"),
            Event::TaskListMarker(checked) => {
                let _ = write!(
                    out,
                    "<input type=\"checkbox\" disabled{}>",
                    if checked { " checked" } else { "" }
                );
            }
            Event::Rule => out.push_str("<hr>"),
            // Standard Markdown policy: raw HTML (block *and* inline) passes
            // through — docs content is first-party repository content, the
            // same contract mdBook/Zola/Starlight ship. Text nodes are
            // escaped. If content ever becomes untrusted, sanitize the
            // *output* with `ammonia` (Wave 2, §Scope in lib.rs) — do not
            // silently narrow here.
            Event::Text(text) => out.push_str(&escape_html(&text)),
            Event::Html(text) | Event::InlineHtml(text) => out.push_str(&text),
            Event::SoftBreak => out.push('\n'),
            Event::HardBreak => out.push_str("<br>"),
            _ => {}
        }
    }
    Ok(out)
}

fn escape_html(src: &str) -> String {
    src.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(src: &str) -> String {
    escape_html(src).replace('"', "&quot;")
}
