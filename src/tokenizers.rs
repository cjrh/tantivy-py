use std::str::CharIndices;
use tantivy::tokenizer::BoxTokenStream;
use tantivy::tokenizer::{Token, TokenStream, Tokenizer, TokenizerManager};

// pub fn register_tokenizer(manager: &TokenizerManager, tokenizer: Box<dyn Tokenizer>) {
//     manager.register("kapiche", tokenizer);
// }

/// Tokenize the text by splitting on whitespaces.
#[derive(Clone)]
pub struct WhitespacePuncTokenizer;

pub struct WhitespacePuncTokenStream<'a> {
    text: &'a str,
    chars: CharIndices<'a>,
    token: Token,
}

impl Tokenizer for WhitespacePuncTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        BoxTokenStream::from(WhitespacePuncTokenStream {
            text,
            chars: text.char_indices(),
            token: Token::default(),
        })
    }
}

impl<'a> WhitespacePuncTokenStream<'a> {
    // search for the end of the current token.
    fn search_token_end(&mut self) -> usize {
        (&mut self.chars)
            .filter(|(_, c)| c.is_ascii_whitespace())
            .map(|(offset, _)| offset)
            .next()
            .unwrap_or_else(|| self.text.len())
    }
}

impl<'a> TokenStream for WhitespacePuncTokenStream<'a> {
    fn advance(&mut self) -> bool {
        self.token.text.clear();
        self.token.position = self.token.position.wrapping_add(1);
        while let Some((offset_from, c)) = self.chars.next() {
            if c.is_ascii_whitespace() {
                continue;
            }

            let offset_to = self.search_token_end();
            self.token.offset_from = offset_from;
            self.token.offset_to = offset_to;
            self.token.text.push_str(&self.text[offset_from..offset_to]);
            return true;
        }
        false
    }

    fn token(&self) -> &Token {
        &self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    /// This is a function that can be used in tests and doc tests
    /// to assert a token's correctness.
    pub fn assert_token(
        token: &Token,
        position: usize,
        text: &str,
        from: usize,
        to: usize,
    ) {
        assert_eq!(
            token.position, position,
            "expected position {} but {:?}",
            position, token
        );
        assert_eq!(token.text, text, "expected text {} but {:?}", text, token);
        assert_eq!(
            token.offset_from, from,
            "expected offset_from {} but {:?}",
            from, token
        );
        assert_eq!(
            token.offset_to, to,
            "expected offset_to {} but {:?}",
            to, token
        );
    }
    #[test]
    fn test_whitespace_tokenizer() {
        let tokenizer_manager = TokenizerManager::default();
        let tokenizer = WhitespacePuncTokenizer{};
        tokenizer_manager.register("wpt", tokenizer);
        let ws_tokenizer = tokenizer_manager.get("wpt").unwrap();
        let mut tokens: Vec<Token> = vec![];
        {
            let mut add_token = |token: &Token| {
                tokens.push(token.clone());
            };
            ws_tokenizer
                .token_stream("Hello, happy tax payer!")
                .process(&mut add_token);
        }

        println!("{:?}", &tokens);
        assert_eq!(tokens.len(), 6);
        assert_token(&tokens[0], 0, "Hello,", 0, 5);
        assert_token(&tokens[1], 1, ",", 5, 6);
        assert_token(&tokens[2], 2, "happy", 7, 12);
        assert_token(&tokens[3], 3, "tax", 13, 16);
        assert_token(&tokens[4], 4, "payer", 17, 22);
        assert_token(&tokens[5], 5, "!", 22, 23);
    }
}
