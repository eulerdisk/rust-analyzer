use super::*;

pub(super) fn item_first(p: &Parser) -> bool {
    match p.current() {
        STRUCT_KW | FN_KW => true,
        _ => false,
    }
}

pub(super) fn item(p: &mut Parser) {
    attributes::outer_attributes(p);
    visibility(p);
    node_if(p, STRUCT_KW, STRUCT_ITEM, struct_item)
        || node_if(p, FN_KW, FN_ITEM, fn_item);
}

fn struct_item(p: &mut Parser) {
    p.expect(IDENT)
        && p.curly_block(|p| comma_list(p, EOF, struct_field));
}

fn struct_field(p: &mut Parser) -> bool {
    node_if(p, IDENT, STRUCT_FIELD, |p| {
        p.expect(COLON) && p.expect(IDENT);
    })
}

fn fn_item(p: &mut Parser) {
    p.expect(IDENT) && p.expect(L_PAREN) && p.expect(R_PAREN)
        && p.curly_block(|p| ());
}

