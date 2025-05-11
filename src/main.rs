use akuru::{
    diagnostics::{Diagnostic, DiagnosticsBag, Label, LabelStyle},
    source::SourceMap,
    span::Span,
};

fn main() {
    let mut map = SourceMap::fresh();
    let mut bag = DiagnosticsBag::new();

    let file = map.with_content("a.ak", "let x = 3;\nlet y = x + ;");
    let span = Span::new(file, 22, 23);
    let label = Label::primary(span, "expected expression after '+'");

    let diag = Diagnostic::error("syntax error").with_label(label);
    bag.push(diag);

    let file = map.with_content(
        "b.rs",
        "fn main() {\n  let x = 1;\n  let y =\n  3 +\n  ;\n}",
    );
    let span = Span::new(file, 27, 35); // covers over multiple lines

    let label = Label {
        style: LabelStyle::Primary,
        span,
        message: Some("incomplete expression".to_string()),
    };

    let diag = Diagnostic::error("parse error").with_label(label);
    bag.push(diag);

    let file = map.with_content("c.rs", "let x = 3;\nlet y = x + ;");
    let span = Span::new(file, 4, 5);
    let primary = Label::primary(span, "invalid identifier");
    let secondary = Label::secondary(span, "declared here");

    let diag = Diagnostic::error("bad variable name")
        .with_label(primary)
        .with_label(secondary);
    bag.push(diag);

    bag.render_all(&map);
}
