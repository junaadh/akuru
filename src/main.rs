use akuru::{lexer::Lexer, source::SourceMap, tokens::TokenKind};

fn main() {
    let mut map = SourceMap::fresh();

    let file = map.with_content("a.ak", "990 , : ;? () & || |= += ++ [] {}");

    let mut lex = Lexer::new(file, &map[file]);
    let mut tokens = Vec::new();

    loop {
        let token = lex.next_token();

        if token.kind == TokenKind::Eof {
            break;
        }

        tokens.push(token);
    }

    lex.bag.render_all(&map);
    for t in tokens {
        println!("{t:?}");
    }
}
