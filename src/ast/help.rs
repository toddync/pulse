use super::types::TokenKind;

pub fn is_literal(kind: &TokenKind) -> bool {
    if kind == &TokenKind::String
        || kind == &TokenKind::Number
        || kind == &TokenKind::Float
        || kind == &TokenKind::Bool
    {
        return true;
    } else {
        return false;
    }
}
