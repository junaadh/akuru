use akuru::{lexer::Lexer, source::SourceMap, tokens::TokenKind};

fn main() {
    let mut map = SourceMap::fresh();

    let file = map.with_content(
        "a.ak",
        "let am: f32 = 990.100; 1..10 1..=20 rest..., index[1..] index[..20] self.hello self.method(1, 2).method2(10)[1..2] *self &self &mut self .69e20 .89e 12e3 0675 0b1001 10e 0xAe2 \"hel\nlo\" \"lmao\" 'g' '\\n' '0' , : ;? () & || |= += ++ [] {}",
    );

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
