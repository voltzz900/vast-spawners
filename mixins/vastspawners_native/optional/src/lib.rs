//! Vast Spawners' integrations are compiled in only when the mod they work with is in the same build. A native mod
//! is built on the player's computer with the mods they have, so this is decided each time the game is built: the
//! staged `src/game.rs` of the crate being compiled holds a `mod <name> {` for each bundle's module.

use proc_macro::{Delimiter, TokenStream, TokenTree};

fn game_rs() -> String {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    std::fs::read_to_string(format!("{dir}/src/game.rs")).unwrap_or_default()
}

/// `"text", { kept } [else { otherwise }]`.
fn parts(input: TokenStream, name: &str) -> (String, TokenStream, TokenStream) {
    let mut tokens = input.into_iter();
    let text = match tokens.next() {
        Some(TokenTree::Literal(l)) => l.to_string().trim_matches('"').replace("\\\"", "\""),
        _ => panic!("{name}!: a string first"),
    };
    assert!(matches!(tokens.next(), Some(TokenTree::Punct(p)) if p.as_char() == ','), "{name}!: a comma after the string");
    let kept = match tokens.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => g.stream(),
        _ => panic!("{name}!: a block after the string"),
    };
    let otherwise = match (tokens.next(), tokens.next()) {
        (Some(TokenTree::Ident(i)), Some(TokenTree::Group(g))) if i.to_string() == "else" && g.delimiter() == Delimiter::Brace => g.stream(),
        (None, None) => TokenStream::new(),
        _ => panic!("{name}!: only `else {{ }}` may follow the block"),
    };
    (text, kept, otherwise)
}

/// `with_mod!("jade", { items } else { items })`: the first items when module `jade` is in the game, else the second.
#[proc_macro]
pub fn with_mod(input: TokenStream) -> TokenStream {
    let (module, kept, otherwise) = parts(input, "with_mod");
    if game_rs().contains(&format!("mod {module} {{")) { kept } else { otherwise }
}

/// `with_source!("pub fn register_category(", { items } else { items })`: the first items when that text is in the
/// game's source (a mod's API that only some of its versions have), else the second.
#[proc_macro]
pub fn with_source(input: TokenStream) -> TokenStream {
    let (text, kept, otherwise) = parts(input, "with_source");
    if game_rs().contains(&text) { kept } else { otherwise }
}
