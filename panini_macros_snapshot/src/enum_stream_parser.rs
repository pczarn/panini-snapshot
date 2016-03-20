use std::mem;

use panini_codegen::rs;
use enum_stream_codegen::front::{Stmts, Stmt, StmtRhs, RhsElem};

use util::delimit;

fn quote_pat(cx: &mut rs::ExtCtxt, tok: &[rs::TokenTree]) -> rs::P<rs::Pat> {
    quote_pat!(cx, $tok)
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse_from_tts(&mut self, cx: &mut rs::ExtCtxt, tts: &[rs::TokenTree]) -> Stmts {
        let sess = cx.parse_sess();
        let mut trdr = rs::lexer::new_tt_reader(&sess.span_diagnostic, None, None, tts.to_vec());

        let mut spans = vec![];
        let mut tokens = vec![];
        let mut token_and_span = rs::transcribe::tt_next_token(&mut trdr);
        while token_and_span.tok != rs::Token::Eof {
            let t = mem::replace(&mut token_and_span, rs::transcribe::tt_next_token(&mut trdr));
            spans.push(t.sp);
            tokens.push(t.tok);
        }
        self.parse_grammar(cx, &tokens[..], &spans[..])
    }

    pub fn parse_grammar(&mut self, cx: &mut rs::ExtCtxt, tokens: &[rs::Token], spans: &[rs::Span]) -> Stmts {
        let mut parser =
// BEGIN GRAMMAR
{
    use ::panini::*;
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    enum Value0<I> where I: Infer0 {
        Start_133(I::V0),
        G_1072(Vec<I::V3>),
        G_1074(Vec<I::V2>),
        Stmt_136(I::V2),
        Name_137(I::V5),
        Matches_138(I::T),
        Rhs_139(I::V8),
        Semi(I::T),
        Conjunction_143(I::V7),
        Ident_145(I::V6),
        Tt_147(I::V1),
        G_1076(Vec<I::V4>),
        Elem_155(I::V4),
        ElemAnd_154(I::V4),
        AndAnd(I::T),
        Tts_95(Vec<I::V1>),
        Not(I::T),
        InnerAttr_135(I::V3),
        Pound(I::T),
        LBracket(I::T),
        MetaItem_165(I::V9),
        RBracket(I::T),
        LParen(I::T),
        MetaItemList_180(Vec<I::V9>),
        RParen(I::T),
        MetaItemComma_183(I::V9),
        Comma(I::T),
        G_1078(Vec<I::V9>),
        G_1080(Vec<I::V9>),
        Eq(I::T),
        AnyToken(I::T),
        LBrace(I::T),
        RBrace(I::T),
        G_1082(Vec<I::V1>),
        IdentTokWithSpan(I::T),
    }
    struct TerminalAccessor0;
    #[allow(non_snake_case)]
    impl TerminalAccessor0 {
        #[inline]
        fn semi(&self) -> Symbol { Symbol::from(5usize as u32) }
        #[inline]
        fn and_and(&self) -> Symbol { Symbol::from(17usize as u32) }
        #[inline]
        fn not(&self) -> Symbol { Symbol::from(19usize as u32) }
        #[inline]
        fn r_bracket(&self) -> Symbol { Symbol::from(22usize as u32) }
        #[inline]
        fn l_bracket(&self) -> Symbol { Symbol::from(26usize as u32) }
        #[inline]
        fn pound(&self) -> Symbol { Symbol::from(27usize as u32) }
        #[inline]
        fn r_paren(&self) -> Symbol { Symbol::from(29usize as u32) }
        #[inline]
        fn l_paren(&self) -> Symbol { Symbol::from(32usize as u32) }
        #[inline]
        fn comma(&self) -> Symbol { Symbol::from(34usize as u32) }
        #[inline]
        fn eq(&self) -> Symbol { Symbol::from(37usize as u32) }
        #[inline]
        fn any_token(&self) -> Symbol { Symbol::from(38usize as u32) }
        #[inline]
        fn r_brace(&self) -> Symbol { Symbol::from(42usize as u32) }
        #[inline]
        fn l_brace(&self) -> Symbol { Symbol::from(43usize as u32) }
        #[inline]
        fn ident_tok_with_span(&self) -> Symbol {
            Symbol::from(45usize as u32)
        }
    }
    macro_rules! G_1072(( $ x : expr ) => {
                        { let mut _cont = $ x ; { _cont(Vec::new()); } } });
    macro_rules! G_1074(( $ x : expr ) => {
                        { let mut _cont = $ x ; { _cont(Vec::new()); } } });
    macro_rules! G_1076(( $ x : expr ) => {
                        { let mut _cont = $ x ; { _cont(Vec::new()); } } });
    macro_rules! G_1078(( $ x : expr ) => {
                        { let mut _cont = $ x ; { _cont(Vec::new()); } } });
    macro_rules! G_1080(( $ x : expr ) => {
                        { let mut _cont = $ x ; { _cont(Vec::new()); } } });
    macro_rules! G_1082(( $ x : expr ) => {
                        { let mut _cont = $ x ; { _cont(Vec::new()); } } });
    macro_rules! start_133(( $ x : expr ) => {
                           {
                           let mut _cont = $ x ;
                           {
    G_1072!(|attrs| { G_1074!(|stmts| { _cont({ Stmts::new(attrs, stmts) }); }); });
}
                           } });
    macro_rules! meta_item_list_180(( $ x : expr ) => {
                                    {
                                    let mut _cont = $ x ;
                                    { G_1078!(|arg0| { _cont(arg0); }); } }
                                    });
    macro_rules! tts_95(( $ x : expr ) => {
                        {
                        let mut _cont = $ x ;
                        { G_1082!(|arg0| { _cont(arg0); }); } } });
    macro_rules! elem_155(( $ x : expr ) => {
                          {
                          let mut _cont = $ x ;
                          {
    tts_95!(|tts| {
_cont({
          let tts: Vec<rs::TokenTree> = tts;
          RhsElem{pattern: quote_pat(cx , &tts[..]), positive: true,}
      }); });
}
                          } });
    macro_rules! conjunction_143(( $ x : expr ) => {
                                 {
                                 let mut _cont = $ x ;
                                 {
    G_1076!(|elems| {
elem_155!(|elem| {
_cont({ let mut elems: Vec<RhsElem> = elems; elems.push(elem); elems }); });
});
}
                                 } });
    macro_rules! rhs_139(( $ x : expr ) => {
                         {
                         let mut _cont = $ x ;
                         {
    conjunction_143!(|c| { _cont({ StmtRhs{conjunction: c, guard: None,} }); });
}
                         } });
    static SERIALIZED_GRAMMAR: &'static [u8] =
        b"_!\xb0\n\x00\xe0\x00\x00\x02\x00\xb0\n\x00@\x00\x00\\!\x00\x00\x00\xa0\x00\x00X!\x00\x00\x00 \x00\x00P!\x00\x00\x00 \x00\x00 \x00\x00\x00\x00\x00\x00\x00@!\x00\x00\x00 \x00\x00\x80\xfc\x0f\x04\xc1;\t\x00\x00!\x00\x00\x00 \x00\x00\x00\x02\x00\x00 \x00\x00\x00\x00\xd4\x0f\x04\xc1\x1b\t\x00\x00\xfc\x0f\x04\xc1;\t\x00\x00\x10\x00\x04\xc1\x0b\x00\x00\x00 \x00\x00\x00 \x00\x00\x00\xd0\x0f\x04\xc1\x1b\t\x00\x00\x90\x0c\x04\xc1\x1b\x08\x00\x00\x90\x0f\x04\xc1\x1b\x08\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x10\x04\x04\xc1\x1b\x08\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\xb0\n\x00\x00\x00\x00\x00\x00\xa0\n\x00\x00\x00\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00\x80\n\x00\x00\x00\x00\x00!\x00Q\x00 \x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00!\x00P\x00 \x00\x00\x00\x00\x00 \x00\x00\x00\x00\x00!\x00@\x00 \x00\x00\x00!\x00\xd1\x1a \x06\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00!\x00Q\x02 \x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00!\x00Q\n \x02\x00\x00!\x00Q\x12 \x04\x00\x00\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x04\x80\x00\x00\x00\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x10\x00\x04\xc1\x1b\x08\x00\x00\x00\x00\x00\x00 \x00\x00\x00\x00\xb0\n\x00@\x00\x00X!\x00\x00\x00\xa0\x00\x00\x00\x90\x0f\x04\xc1\x1b\t\x00\x00!\x00Q\x02 \x02\x00\x00!\x00Q\x02 \x04\x00\x00\x10\x00\x04\xc1\x0b\x08\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00.\x00\x00\x00\x01\x00\x00\x00/\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x01\x00\x00\x00\x0b\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x01\x00\x00\x00%\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x01\x00\x00\x00&\x00\x00\x00\x01\x00\x00\x00\'\x00\x00\x00\x01\x00\x00\x00(\x00\x00\x00\x01\x00\x00\x00)\x00\x00\x00\x01\x00\x00\x00-\x00\x00\x00\x01\x00\x00\x000\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x01\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x00,\x00\x00\x00\x01\x00\x00\x00\x15\x00\x00\x00\x01\x00\x00\x00\x17\x00\x00\x00\x01\x00\x00\x00\x19\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x01\x00\x00\x00\x1c\x00\x00\x00\x01\x00\x00\x00\x1b\x00\x00\x00\x01\x00\x00\x00\x1e\x00\x00\x00\x01\x00\x00\x00\x1e\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x01\x00\x00\x00#\x00\x00\x00\x01\x00\x00\x00$\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x01\x00\x00\x001\x00\x00\x00\x01\x00\x00\x002\x00\x00\x00\x01\x00\x00\x00\x1a\x00\x00\x00\x01\x00\x00\x00\x1a\x00\x00\x00\x01\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00 \x00\x00\x00\x01\x00\x00\x00+\x00\x00\x00\x01\x00\x00\x00+\x00\x00\x00\x01\x00\x00\x003\x00\x00\x00\x01\x00\x00\x00\x14\x00\x00\x00\x01\x00\x00\x00.\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00/\x00\x00\x00\x01\x00\x00\x00\x10\x00\x00\x00\x01\x00\x00\x000\x00\x00\x00\x01\x00\x00\x00!\x00\x00\x00\x01\x00\x00\x001\x00\x00\x00\x01\x00\x00\x00!\x00\x00\x00\x01\x00\x00\x002\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x01\x00\x00\x003\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x05\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x16\x00\x00\x00\x01\x00\x00\x00\x1d\x00\x00\x00\x01\x00\x00\x00*\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x16\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x01\x00\x00\x00\x1a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x1d\x00\x00\x00\x01\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x1f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x14\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x01\x00\x00\x00\t\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x01\x00\x00\x00\x0b\x00\x00\x00\x01\x00\x00\x00\x0b\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\x10\x00\x00\x00\x01\x00\x00\x00\x10\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x01\x00\x00\x00\x14\x00\x00\x00\x01\x00\x00\x00\x15\x00\x00\x00\x01\x00\x00\x00\x17\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x01\x00\x00\x00\x19\x00\x00\x00\x01\x00\x00\x00\x1c\x00\x00\x00\x01\x00\x00\x00\x1c\x00\x00\x00\x01\x00\x00\x00\x1e\x00\x00\x00\x01\x00\x00\x00\x1f\x00\x00\x00\x01\x00\x00\x00\x1f\x00\x00\x00\x01\x00\x00\x00\x1f\x00\x00\x00\x01\x00\x00\x00!\x00\x00\x00\x01\x00\x00\x00#\x00\x00\x00\x01\x00\x00\x00$\x00\x00\x00\x01\x00\x00\x00\'\x00\x00\x00\x01\x00\x00\x00\'\x00\x00\x00\x01\x00\x00\x00(\x00\x00\x00\x01\x00\x00\x00(\x00\x00\x00\x01\x00\x00\x00)\x00\x00\x00\x01\x00\x00\x00)\x00\x00\x00\x01\x00\x00\x00,\x00\x00\x00\x01\x00\x00\x00.\x00\x00\x00\x01\x00\x00\x00.\x00\x00\x00\x01\x00\x00\x00/\x00\x00\x00\x01\x00\x00\x00/\x00\x00\x00\x01\x00\x00\x000\x00\x00\x00\x01\x00\x00\x000\x00\x00\x00\x01\x00\x00\x001\x00\x00\x00\x01\x00\x00\x001\x00\x00\x00\x01\x00\x00\x002\x00\x00\x00\x01\x00\x00\x002\x00\x00\x00\x01\x00\x00\x003\x00\x00\x00\x01\x00\x00\x003\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x16\x00\x00\x00\x01\x00\x00\x00\x17\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\x10\x00\x00\x00\x01\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x01\x00\x00\x00\x15\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x05\x00\x00\x00\x01\x00\x00\x00\x05\x00\x00\x00\x01\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\t\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x01\x00\x00\x00\x0b\x00\x00\x00\x01\x00\x00\x00\x19\x00\x00\x00\x01\x00\x00\x00\x1a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x1b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x7f\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x01\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x01\x7f\x00\x00\x01\x00\x00\x00\x0b\x00\x00\x00\x00\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x00\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x00\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x17\x00\x00\x00\x01\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x1c\x00\x00\x00\x00\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00\x04\x00\x00\x00\x05\x00\x00\x00\x05\x00\x00\x00\x07\x00\x00\x00\x07\x00\x00\x00\n\x00\x00\x00\n\x00\x00\x00\x0c\x00\x00\x00\r\x00\x00\x00\x0e\x00\x00\x00\x10\x00\x00\x00\x12\x00\x00\x00\x14\x00\x00\x00\x15\x00\x00\x00\x16\x00\x00\x00\x17\x00\x00\x00\x19\x00\x00\x00\x1a\x00\x00\x00\x1b\x00\x00\x00\x1b\x00\x00\x00\x1c\x00\x00\x00\x1e\x00\x00\x00\x1f\x00\x00\x00!\x00\x00\x00\"\x00\x00\x00#\x00\x00\x00#\x00\x00\x00%\x00\x00\x00%\x00\x00\x00\'\x00\x00\x00)\x00\x00\x00)\x00\x00\x00*\x00\x00\x00+\x00\x00\x00,\x00\x00\x00-\x00\x00\x00.\x00\x00\x00/\x00\x00\x000\x00\x00\x000\x00\x00\x002\x00\x00\x003\x00\x00\x004\x00\x00\x006\x00\x00\x008\x00\x00\x00:\x00\x00\x00<\x00\x00\x00>\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x80\x00\x00\x00\x00\x02\x00\x00\x80/\x00\x00\x006\x00\x00\x80\x03\x00\x00\x00\x05\x00\x00\x00\x04\x00\x00\x00\x06\x00\x00\x00\x04\x00\x00\x00\x07\x00\x00\x80\x06\x00\x00\x00\x08\x00\x00\x00\x18\x00\x00\x00!\x00\x00\x80\x1e\x00\x00\x00&\x00\x00\x00\x07\x00\x00\x00\t\x00\x00\x80\x0b\x00\x00\x00\x10\x00\x00\x00\x07\x00\x00\x00\n\x00\x00\x003\x00\x00\x00>\x00\x00\x80\x08\x00\x00\x00\x0b\x00\x00\x80\x0b\x00\x00\x00\x11\x00\x00\x80\n\x00\x00\x00\r\x00\x00\x00\n\x00\x00\x00\x0e\x00\x00\x80\n\x00\x00\x00\x0f\x00\x00\x80\x10\x00\x00\x00\x1b\x00\x00\x000\x00\x00\x008\x00\x00\x80\x10\x00\x00\x00\x1c\x00\x00\x80\x0f\x00\x00\x00\x18\x00\x00\x80\x0f\x00\x00\x00\x19\x00\x00\x00\x0f\x00\x00\x00\x1a\x00\x00\x80.\x00\x00\x004\x00\x00\x80\x14\x00\x00\x00\x1e\x00\x00\x00\x15\x00\x00\x00\x1f\x00\x00\x00\x1f\x00\x00\x00)\x00\x00\x80!\x00\x00\x00*\x00\x00\x00\x17\x00\x00\x00 \x00\x00\x00\'\x00\x00\x00-\x00\x00\x00\'\x00\x00\x00.\x00\x00\x80\x19\x00\x00\x00#\x00\x00\x00\x18\x00\x00\x00\"\x00\x00\x00\x1c\x00\x00\x00$\x00\x00\x00\x1c\x00\x00\x00%\x00\x00\x80(\x00\x00\x00/\x00\x00\x00(\x00\x00\x000\x00\x00\x801\x00\x00\x00:\x00\x00\x802\x00\x00\x00<\x00\x00\x80\x1f\x00\x00\x00\'\x00\x00\x80\x1f\x00\x00\x00(\x00\x00\x00\t\x00\x00\x00\x0c\x00\x00\x80\x0c\x00\x00\x00\x12\x00\x00\x80\x0c\x00\x00\x00\x13\x00\x00\x00\x0c\x00\x00\x00\x14\x00\x00\x00\x0c\x00\x00\x00\x15\x00\x00\x00)\x00\x00\x001\x00\x00\x00)\x00\x00\x002\x00\x00\x80\x12\x00\x00\x00\x1d\x00\x00\x80\r\x00\x00\x00\x16\x00\x00\x80\x01\x00\x00\x00\x03\x00\x00\x80.\x00\x00\x005\x00\x00\x00\x02\x00\x00\x00\x04\x00\x00\x80/\x00\x00\x007\x00\x00\x00\x0e\x00\x00\x00\x17\x00\x00\x800\x00\x00\x009\x00\x00\x00#\x00\x00\x00+\x00\x00\x801\x00\x00\x00;\x00\x00\x00$\x00\x00\x00,\x00\x00\x802\x00\x00\x00=\x00\x00\x00,\x00\x00\x003\x00\x00\x803\x00\x00\x00?\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x14\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x13\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x16\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x17\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x19\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x1a\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x1b\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x10\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x11\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x05\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x05\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0b\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x10\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x10\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x16\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x17\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x19\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x1a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x1b\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x03\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x14\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x04\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x10\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x11\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x12\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x15\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x07\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x05\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x05\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x08\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\t\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\n\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x0c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\r\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x0b\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x16\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x16\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x17\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x17\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x18\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x19\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x19\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x1a\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x1a\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x1b\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x1b\x00\x00\x00\x01\x00\x00\x00";
    static TRACE_INFO: TraceInfo =
        TraceInfo{ids:
                      &[0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32,
                        9u32, 10u32, 11u32, 12u32, 13u32, 14u32, 15u32, 16u32,
                        17u32, 18u32, 19u32, 20u32, 21u32, 0u32, 0u32, 4u32,
                        12u32, 13u32, 19u32],
                  map:
                      &[&[0u32, 2u32, 4u32], &[0u32, 1u32, 2u32, 3u32, 4u32],
                        &[0u32, 1u32], &[0u32, 1u32, 2u32, 3u32],
                        &[0u32, 2u32, 3u32], &[0u32, 1u32, 2u32],
                        &[0u32, 1u32], &[0u32, 1u32, 2u32],
                        &[0u32, 1u32, 2u32, 3u32, 4u32, 5u32], &[0u32, 1u32],
                        &[0u32, 1u32, 2u32, 3u32, 4u32], &[0u32, 1u32, 2u32],
                        &[0u32, 2u32], &[0u32, 2u32, 3u32], &[0u32, 1u32],
                        &[0u32, 1u32], &[0u32, 1u32, 2u32, 3u32],
                        &[0u32, 1u32, 2u32, 3u32], &[0u32, 1u32, 2u32, 3u32],
                        &[0u32, 2u32], &[0u32, 1u32], &[0u32, 1u32],
                        &[0u32, 1u32], &[2u32, 3u32], &[0u32, 1u32],
                        &[0u32, 1u32], &[0u32, 1u32], &[0u32, 1u32]],
                  tokens:
                      &[&["start", "inner_attr", "*", "stmt", "*"],
                        &["stmt", "name", "matches", "rhs", "semi"],
                        &["rhs", "conjunction"],
                        &["rhs", "conjunction", "ident", "tt"],
                        &["conjunction", "elem_and", "*", "elem"],
                        &["elem_and", "elem", "and_and"], &["elem", "tts"],
                        &["elem", "not", "tts"],
                        &["inner_attr", "pound", "not", "l_bracket",
                          "meta_item", "r_bracket"], &["meta_item", "name"],
                        &["meta_item", "name", "l_paren", "meta_item_list",
                          "r_paren"],
                        &["meta_item_comma", "meta_item", "comma"],
                        &["meta_item_list", "meta_item_comma", "*"],
                        &["meta_item_list", "meta_item_comma", "*",
                          "meta_item"], &["matches", "eq"],
                        &["tt", "any_token"],
                        &["tt", "l_bracket", "tts", "r_bracket"],
                        &["tt", "l_paren", "tts", "r_paren"],
                        &["tt", "l_brace", "tts", "r_brace"],
                        &["tts", "tt", "*"], &["name", "ident"],
                        &["ident", "ident_tok_with_span"]],};
    static SYM_NAMES: &'static [&'static str] =
        &["start", "G", "G", "stmt", "g4", "semi", "g6", "rhs", "name",
          "matches", "conjunction", "g11", "tt", "ident", "G", "elem",
          "elem_and", "and_and", "tts", "not", "inner_attr", "g21",
          "r_bracket", "g23", "meta_item", "g25", "l_bracket", "pound", "g28",
          "r_paren", "g30", "meta_item_list", "l_paren", "meta_item_comma",
          "comma", "G", "G", "eq", "any_token", "g39", "g40", "g41",
          "r_brace", "l_brace", "G", "ident_tok_with_span", "g46", "g47",
          "g48", "g49", "g50", "g51"];
    struct ParseFactory0 {
        grammar: grammar::InternalGrammar,
    }
    impl ParseFactory0 {
        fn new() -> ParseFactory0 {
            let grammar =
                grammar::InternalGrammar::from_parts(grammar::InternalGrammarParts{storage:
                                                                                       ::std::borrow::Cow::Borrowed(SERIALIZED_GRAMMAR),
                                                                                   num_syms:
                                                                                       52usize,
                                                                                   num_rules:
                                                                                       64usize,
                                                                                   num_external_syms:
                                                                                       0usize,
                                                                                   num_internal_syms:
                                                                                       0usize,
                                                                                   num_nulling_intermediate:
                                                                                       0usize,
                                                                                   start_sym:
                                                                                       Symbol::from(0usize),
                                                                                   trivial_derivation:
                                                                                       true,});
            ParseFactory0{grammar: grammar,}
        }
        fn terminal_accessor(&self) -> TerminalAccessor0 { TerminalAccessor0 }
        fn new_parse<'g, I>(&'g mut self) -> Parse0<'g, I> where
         I: InferTree0<'g> + 'g {
            let bocage = Box::new(Bocage::new(&self.grammar));
            let bocage_ref: &'g Bocage<'g, 'g, 'g, I::Node, Value0<I::Infer>>;
            unsafe { bocage_ref = &*(&*bocage as *const _); }
            let recognizer = Recognizer::new(&self.grammar, bocage_ref);
            let traversal = Traversal::new(bocage_ref, NullOrder::new());
            Parse0{store: Arena::new(),
                   recognizer: recognizer,
                   bocage: bocage,
                   traversal: traversal,
                   finished_node: None,
                   result: [].iter(),}
        }
    }
    trait Infer0 {
        type
        T: Copy;
        type
        V0;
        type
        V1;
        type
        V2;
        type
        V3;
        type
        V4;
        type
        V5;
        type
        V6;
        type
        V7;
        type
        V8;
        type
        V9;
    }
    #[derive(Clone, Copy)]
    struct ValueInfer0<T, V0, V1, V2, V3, V4, V5, V6, V7, V8,
                       V9>(::std::marker::PhantomData<(T, V0, V1, V2, V3, V4,
                                                       V5, V6, V7, V8, V9)>);
    impl <T, V0, V1, V2, V3, V4, V5, V6, V7, V8, V9> Infer0 for
     ValueInfer0<T, V0, V1, V2, V3, V4, V5, V6, V7, V8, V9> where T: Copy {
        type
        T
        =
        T;
        type
        V0
        =
        V0;
        type
        V1
        =
        V1;
        type
        V2
        =
        V2;
        type
        V3
        =
        V3;
        type
        V4
        =
        V4;
        type
        V5
        =
        V5;
        type
        V6
        =
        V6;
        type
        V7
        =
        V7;
        type
        V8
        =
        V8;
        type
        V9
        =
        V9;
    }
    trait InferTree0<'g> {
        type
        Node: Copy;
        type
        Infer: Infer0;
    }
    struct InferTreeVal0<Node, I>(::std::marker::PhantomData<(Node, I)>);
    impl <'g, Node, I> InferTree0<'g> for InferTreeVal0<Node, I> where
     I: Infer0 + InferConstraint1<'g, Node> + 'g, Node: Copy + 'g {
        type
        Node
        =
        Node;
        type
        Infer
        =
        I;
    }
    struct Parse0<'g, I> where I: InferTree0<'g> + 'g {
        store: Arena<Value0<I::Infer>>,
        recognizer: Recognizer<'g, 'g,
                    Bocage<'g, 'g, 'g, I::Node, Value0<I::Infer>>>,
        finished_node: Option<NodeRef<'g, 'g, I::Node, Value0<I::Infer>>>,
        #[allow(dead_code)]
        bocage: Box<Bocage<'g, 'g, 'g, I::Node, Value0<I::Infer>>>,
        traversal: TraversalUnordered<'g, I::Node, Value0<I::Infer>>,
        result: ::std::slice::Iter<'g, Value0<I::Infer>>,
    }
    #[allow(dead_code)]
    impl <'g, I> Parse0<'g, I> where I: InferTree0<'g> + 'g {
        fn begin_earleme(&mut self) { }
        fn traced_begin_earleme(&mut self) { self.begin_earleme(); }
        fn scan_tok(&mut self, token: Symbol, value: I::Node) {
            self.recognizer.scan(token, value);
        }
        fn advance(&mut self) -> bool { self.recognizer.advance() }
        fn traced_advance(&mut self) -> bool {
            if self.recognizer.is_exhausted() {
                false
            } else {
                let mut finished_node = None;
                let mut completion_items = vec!();
                {
                    let start_sym = self.recognizer.grammar().start_sym();
                    let mut completions = self.recognizer.completions();
                    while let Some(mut completion) =
                              completions.next_completion() {
                        while let Some(item) = completion.next() {
                            completion_items.push(item);
                            completion.push(item);
                        }
                        let node = completion.complete();
                        if completion.origin() == 0 &&
                               completion.symbol() == start_sym {
                            finished_node = Some(node);
                        }
                    }
                };
                self.finished_node = finished_node;
                self.recognizer.advance_without_completion();
                print_trace(&self.recognizer, &completion_items[..],
                            TRACE_INFO);
                true
            }
        }
        fn end_of_input(&mut self) {
            self.finished_node = Some(self.recognizer.finished_node());
        }
        fn traced_end_of_input(&mut self) { }
        fn fmt_exhaustion(&self, fmt: &mut fmt::Formatter, input_pos: usize)
         -> Result<(), fmt::Error> {
            try!(write ! ( fmt , "Parse error at {}:\nexpected" , input_pos
                 ));
            let mut terminals = self.recognizer.expected_terminals();
            let last = terminals.next();
            for terminal in terminals {
                try!(write ! (
                     fmt , " `{}`," , SYM_NAMES [ terminal . usize (  ) ] ));
            }
            if let Some(last) = last {
                write!(fmt , " or `{}`." , SYM_NAMES [ last . usize (  ) ])
            } else { write!(fmt , "end of input.") }
        }
    }
    impl <'g, I> Iterator for Parse0<'g, I> where I: InferTree0<'g> + 'g {
        type
        Item
        =
        &'g <I::Infer as Infer0>::V0;
        fn next(&mut self) -> Option<Self::Item> {
            match self.result.next() {
                Some(&Value0::Start_133(ref value)) => Some(value),
                _ => None,
            }
        }
    }
struct EnumStream<C> {
    closure: C,
}struct EnumStreamParser<C, D> {
    closure: C,
    eval_closure: D,
    builder: ParseFactory0,
}struct Parse<'g, I> where I: InferTree0<'g> + 'g {
    parse: Box<Parse0<'g, I>>,
}struct ExhaustedParse<'g, I> where I: InferTree0<'g> + 'g {
    parse: Box<Parse0<'g, I>>,
    input_pos: usize,
}impl <C> EnumStream<C> {
    fn new(closure: C) -> Self { EnumStream{closure: closure,} }
    fn with_parse_builder_and_eval_closure<'g, D,
                                           I>(self, builder: ParseFactory0,
                                              eval_closure: D)
     -> EnumStreamParser<C, D> where D: FnMut(&'g mut Parse0<'g, I>),
     I: InferTree0<'g> + 'g {
        EnumStreamParser{closure: self.closure,
                         eval_closure: eval_closure,
                         builder: builder,}
    }
}#[allow(dead_code)]
impl <C, D> EnumStreamParser<C, D> {
    fn parse<'g, I, Iter>(&'g mut self, into_iter: Iter)
     -> Result<Parse<'g, I>, ExhaustedParse<'g, I>> where
     C: Fn(usize, Iter::Item) -> bool, D: FnMut(&'g mut Parse0<'g, I>),
     Iter: IntoIterator<Item = I::Node>, Iter::Item: Copy, I: InferTree0<'g> +
     'g {
        self.common_parse(into_iter, false)
    }
    fn traced_parse<'g, I, Iter>(&'g mut self, into_iter: Iter)
     -> Result<Parse<'g, I>, ExhaustedParse<'g, I>> where
     C: Fn(usize, Iter::Item) -> bool, D: FnMut(&'g mut Parse0<'g, I>),
     Iter: IntoIterator<Item = I::Node>, Iter::Item: Copy, I: InferTree0<'g> +
     'g {
        self.common_parse(into_iter, true)
    }
    #[inline]
    fn common_parse<'g, I, Iter>(&'g mut self, into_iter: Iter, traced: bool)
     -> Result<Parse<'g, I>, ExhaustedParse<'g, I>> where
     C: Fn(usize, Iter::Item) -> bool, D: FnMut(&'g mut Parse0<'g, I>),
     Iter: IntoIterator<Item = I::Node>, Iter::Item: Copy, I: InferTree0<'g> +
     'g {
        let tokens =
            &[self.builder.terminal_accessor().not(),
              self.builder.terminal_accessor().pound(),
              self.builder.terminal_accessor().comma(),
              self.builder.terminal_accessor().and_and(),
              self.builder.terminal_accessor().eq(),
              self.builder.terminal_accessor().semi(),
              self.builder.terminal_accessor().ident_tok_with_span(),
              self.builder.terminal_accessor().l_brace(),
              self.builder.terminal_accessor().r_brace(),
              self.builder.terminal_accessor().l_bracket(),
              self.builder.terminal_accessor().r_bracket(),
              self.builder.terminal_accessor().l_paren(),
              self.builder.terminal_accessor().r_paren(),
              self.builder.terminal_accessor().any_token()];
        let mut parse_box = Box::new(self.builder.new_parse());
        let parse: &'g mut Parse0<'g, I>;
        unsafe { parse = &mut *(&mut *parse_box as *mut _); }
        let iter = into_iter.into_iter();
        for (i, elem) in iter.enumerate() {
            if traced {
                parse.traced_begin_earleme();
            } else { parse.begin_earleme(); }
            scan_elem(&mut self.closure, tokens, parse, elem);
            let success =
                if traced { parse.traced_advance() } else { parse.advance() };
            if !success {
                return Err(ExhaustedParse{parse: parse_box, input_pos: i,});
            }
        }
        if traced {
            parse.traced_end_of_input();
        } else { parse.end_of_input(); }
        (self.eval_closure)(parse);
        Ok(Parse{parse: parse_box,})
    }
}fn scan_elem<'g, C,
             I>(closure: &mut C, tokens: &[Symbol], parse: &mut Parse0<'g, I>,
                elem: I::Node) where C: Fn(usize, I::Node) -> bool,
 I: InferTree0<'g> + 'g {
    if closure(0usize, elem) {
        let token = tokens[0usize];
        parse.scan_tok(token, elem);
    }
    if closure(1usize, elem) {
        let token = tokens[1usize];
        parse.scan_tok(token, elem);
    }
    if closure(2usize, elem) {
        let token = tokens[2usize];
        parse.scan_tok(token, elem);
    }
    if closure(3usize, elem) {
        let token = tokens[3usize];
        parse.scan_tok(token, elem);
    }
    if closure(4usize, elem) {
        let token = tokens[4usize];
        parse.scan_tok(token, elem);
    }
    if closure(5usize, elem) {
        let token = tokens[5usize];
        parse.scan_tok(token, elem);
    }
    if closure(6usize, elem) {
        let token = tokens[6usize];
        parse.scan_tok(token, elem);
    }
    if closure(7usize, elem) {
        let token = tokens[7usize];
        parse.scan_tok(token, elem);
    }
    if closure(8usize, elem) {
        let token = tokens[8usize];
        parse.scan_tok(token, elem);
    }
    if closure(9usize, elem) {
        let token = tokens[9usize];
        parse.scan_tok(token, elem);
    }
    if closure(10usize, elem) {
        let token = tokens[10usize];
        parse.scan_tok(token, elem);
    }
    if closure(11usize, elem) {
        let token = tokens[11usize];
        parse.scan_tok(token, elem);
    }
    if closure(12usize, elem) {
        let token = tokens[12usize];
        parse.scan_tok(token, elem);
    }
    if closure(13usize, elem) {
        let token = tokens[13usize];
        parse.scan_tok(token, elem);
    }
}impl <'g, I> Iterator for Parse<'g, I> where I: InferTree0<'g> + 'g {
    type
    Item
    =
    <Parse0<'g, I> as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> { self.parse.next() }
}trait InferConstraint1<'g, Node>: Infer0<T = Node> where Node: Copy { }impl <'g, Node, T> InferConstraint1<'g, Node> for T where Node: Copy,
 T: Infer0<T = Node> {
}impl <'g, I> fmt::Debug for ExhaustedParse<'g, I> where I: InferTree0<'g> + 'g
 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.parse.fmt_exhaustion(fmt, self.input_pos)
    }
}macro_rules! layer_macro1((
                          @ closure $ upper_builder : expr , $ ignored__parse
                          : expr , $ node : expr ; ) => (
                          {
                          let terminal_accessor = TerminalAccessor0 ; let
                          upper_builder = & mut $ upper_builder ; let sym = (
                          $ node ) . terminal ; let value = ( $ node ) . value
                          ; let tokens = & [
                          terminal_accessor . not (  ) , terminal_accessor .
                          pound (  ) , terminal_accessor . comma (  ) ,
                          terminal_accessor . and_and (  ) , terminal_accessor
                          . eq (  ) , terminal_accessor . semi (  ) ,
                          terminal_accessor . ident_tok_with_span (  ) ,
                          terminal_accessor . l_brace (  ) , terminal_accessor
                          . r_brace (  ) , terminal_accessor . l_bracket (  )
                          , terminal_accessor . r_bracket (  ) ,
                          terminal_accessor . l_paren (  ) , terminal_accessor
                          . r_paren (  ) , terminal_accessor . any_token (  )
                          , ] ; let value = if sym == tokens [ 0usize ] {
                          Value0 :: Not ( value ) } else if sym == tokens [
                          1usize ] { Value0 :: Pound ( value ) } else if sym
                          == tokens [ 2usize ] { Value0 :: Comma ( value ) }
                          else if sym == tokens [ 3usize ] {
                          Value0 :: AndAnd ( value ) } else if sym == tokens [
                          4usize ] { Value0 :: Eq ( value ) } else if sym ==
                          tokens [ 5usize ] { Value0 :: Semi ( value ) } else
                          if sym == tokens [ 6usize ] {
                          Value0 :: IdentTokWithSpan ( value ) } else if sym
                          == tokens [ 7usize ] { Value0 :: LBrace ( value ) }
                          else if sym == tokens [ 8usize ] {
                          Value0 :: RBrace ( value ) } else if sym == tokens [
                          9usize ] { Value0 :: LBracket ( value ) } else if
                          sym == tokens [ 10usize ] {
                          Value0 :: RBracket ( value ) } else if sym == tokens
                          [ 11usize ] { Value0 :: LParen ( value ) } else if
                          sym == tokens [ 12usize ] {
                          Value0 :: RParen ( value ) } else if sym == tokens [
                          13usize ] { Value0 :: AnyToken ( value ) } else {
                          unreachable ! (  ) } ; upper_builder . reserve ( 1 )
                          ; upper_builder . push ( value ) ; } ) ; (
                          @ builder @ factory [ $ factory : expr ] @ closure [
                          $ closure : expr ] ) => (
                          EnumStream :: new (
                          | id : usize , item | {
                          if id == 0usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item { Some((&rs::Token::Not, _)) => return true, _ => (), }
                          } if id == 1usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item { Some((&rs::Token::Pound, _)) => return true, _ => (), }
                          } if id == 2usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item { Some((&rs::Token::Comma, _)) => return true, _ => (), }
                          } if id == 3usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item { Some((&rs::Token::AndAnd, _)) => return true, _ => (), }
                          } if id == 4usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item { Some((&rs::Token::Eq, _)) => return true, _ => (), }
                          } if id == 5usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item { Some((&rs::Token::Semi, _)) => return true, _ => (), }
                          } if id == 6usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item { Some((&rs::Token::Ident(_), _)) => return true, _ => (), }
                          } if id == 7usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item {
    Some((&rs::Token::OpenDelim(rs::DelimToken::Brace), _)) => return true,
    _ => (),
}
                          } if id == 8usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item {
    Some((&rs::Token::CloseDelim(rs::DelimToken::Brace), _)) => return true,
    _ => (),
}
                          } if id == 9usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item {
    Some((&rs::Token::OpenDelim(rs::DelimToken::Bracket), _)) => return true,
    _ => (),
}
                          } if id == 10usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item {
    Some((&rs::Token::CloseDelim(rs::DelimToken::Bracket), _)) => return true,
    _ => (),
}
                          } if id == 11usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item {
    Some((&rs::Token::OpenDelim(rs::DelimToken::Paren), _)) => return true,
    _ => (),
}
                          } if id == 12usize {
                          let item = Some ( item ) ; match item { _ => {  } }
                          match item {
    Some((&rs::Token::CloseDelim(rs::DelimToken::Paren), _)) => return true,
    _ => (),
}
                          } if id == 13usize {
                          let item = Some ( item ) ; match item {
                          Some ( (&rs::Token::OpenDelim(_), _) ) => return
                          false , Some ( (&rs::Token::CloseDelim(_), _) ) =>
                          return false , Some ( (&rs::Token::Semi, _) ) =>
                          return false , Some ( (&rs::Token::Not, _) ) =>
                          return false , Some ( (&rs::Token::AndAnd, _) ) =>
                          return false ,
                          Some((&rs::Token::Ident(ident), _)) if ident.name.as_str() == "if" => return false,
                          _ => {  } }
                          match item { Some(_) => return true, _ => (), } }
                          false } ) . with_parse_builder_and_eval_closure (
                          $ factory , $ closure ) ) ; ( @ get $ parse : expr )
                          => ( $ parse ));
    layer_macro1!(@ builder @ factory [ ParseFactory0 :: new (  ) ] @ closure
                  [
                  | mut parse | {
                  {
                  let _ : & Parse0 < InferTreeVal0 < _ , ValueInfer0 < _ , _ ,
                  _ , _ , _ , _ , _ , _ , _ , _ , _ >> > = & * layer_macro1 !
                  ( @ get parse ) ; } ; let & mut Parse0 {
                  ref mut traversal , ref store , finished_node , ref mut
                  result , .. } = & mut * layer_macro1 ! ( @ get parse ) ; let
                  root = finished_node . unwrap (  ) ; let mut
                  cartesian_product = CartesianProduct :: new (  ) ; traversal
                  . traverse ( root ) ; loop {
                  if let Some ( deps ) = traversal . traverse_deps (  ) {
                  for node in deps {
                  match node {
                  TraversalBottom :: Leaf ( node ) => {
                  let mut builder = SliceBuilder :: new ( & store , 0 ) ;
                  layer_macro1 ! ( @ closure builder , parse , node ; ) ; node
                  . result ( builder . into_slice (  ) ) ; } TraversalBottom
                  :: Null ( nulling ) => {
                  let mut _builder = SliceBuilder :: new ( & store , 0 ) ;
                  match nulling . symbol . usize (  ) {
                  1usize => {
                  _builder . reserve ( 1usize ) ; G_1072 ! (
                  | result | {
                  _builder . push ( Value0 :: G_1072 ( result ) ) ; } ) ; }
                  2usize => {
                  _builder . reserve ( 1usize ) ; G_1074 ! (
                  | result | {
                  _builder . push ( Value0 :: G_1074 ( result ) ) ; } ) ; }
                  11usize => {
                  _builder . reserve ( 1usize ) ; G_1076 ! (
                  | result | {
                  _builder . push ( Value0 :: G_1076 ( result ) ) ; } ) ; }
                  27usize => {
                  _builder . reserve ( 1usize ) ; G_1078 ! (
                  | result | {
                  _builder . push ( Value0 :: G_1078 ( result ) ) ; } ) ; }
                  28usize => {
                  _builder . reserve ( 1usize ) ; G_1080 ! (
                  | result | {
                  _builder . push ( Value0 :: G_1080 ( result ) ) ; } ) ; }
                  33usize => {
                  _builder . reserve ( 1usize ) ; G_1082 ! (
                  | result | {
                  _builder . push ( Value0 :: G_1082 ( result ) ) ; } ) ; }
                  0usize => {
                  _builder . reserve ( 1usize ) ; start_133 ! (
                  | result | {
                  _builder . push ( Value0 :: Start_133 ( result ) ) ; } ) ; }
                  23usize => {
                  _builder . reserve ( 1usize ) ; meta_item_list_180 ! (
                  | result | {
                  _builder . push ( Value0 :: MetaItemList_180 ( result ) ) ;
                  } ) ; } 15usize => {
                  _builder . reserve ( 1usize ) ; tts_95 ! (
                  | result | {
                  _builder . push ( Value0 :: Tts_95 ( result ) ) ; } ) ; }
                  12usize => {
                  _builder . reserve ( 1usize ) ; elem_155 ! (
                  | result | {
                  _builder . push ( Value0 :: Elem_155 ( result ) ) ; } ) ; }
                  8usize => {
                  _builder . reserve ( 1usize ) ; conjunction_143 ! (
                  | result | {
                  _builder . push ( Value0 :: Conjunction_143 ( result ) ) ; }
                  ) ; } 6usize => {
                  _builder . reserve ( 1usize ) ; rhs_139 ! (
                  | result | {
                  _builder . push ( Value0 :: Rhs_139 ( result ) ) ; } ) ; }
                  id => unreachable ! ( "nulling id {}" , id ) } nulling .
                  result ( _builder . into_slice (  ) ) ; } } } } else {
                  break ; } for node in traversal . traverse_sum (  ) {
                  let count = node . iter (  ) . map ( | alt | alt . len (  )
                  ) . fold ( 0 , | acc , elem | acc + elem ) ; let mut
                  slice_builder = SliceBuilder :: new ( & store , count ) ;
                  for alt in node . iter (  ) {
                  cartesian_product . from_production ( & alt ) ; loop {
                  let result = {
                  let args = cartesian_product . as_slice (  ) ; match alt .
                  action (  ) {
                  0u32 => {
                  let val = (
                  true , args [ 0usize ] . clone (  ) , args [ 1usize ] .
                  clone (  ) , ) ; if let (
                  true , Value0 :: G_1072 ( attrs ) , Value0 :: G_1074 ( stmts
                  ) , ) = val {
                  Value0 :: Start_133 ( { Stmts::new(attrs, stmts) } ) } else
                  { unreachable ! (  ) } } 1u32 => {
                  let val = (
                  true , args [ 0usize ] . clone (  ) , args [ 2usize ] .
                  clone (  ) , ) ; if let (
                  true , Value0 :: Name_137 ( name ) , Value0 :: Rhs_139 ( rhs
                  ) , ) = val {
                  Value0 :: Stmt_136 (
                  { let name: rs::Spanned<rs::Name> = name; Stmt{name: name.node, rhs: rhs,} }
                  ) } else { unreachable ! (  ) } } 2u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Conjunction_143 ( c ) , ) = val {
                  Value0 :: Rhs_139 (
                  { StmtRhs{conjunction: c, guard: None,} } ) } else {
                  unreachable ! (  ) } } 3u32 => {
                  let val = (
                  true , args [ 0usize ] . clone (  ) , args [ 1usize ] .
                  clone (  ) , args [ 2usize ] . clone (  ) , ) ; if let (
                  true , Value0 :: Conjunction_143 ( c ) , Value0 :: Ident_145
                  ( ident ) , Value0 :: Tt_147 ( guard_cond ) , ) = val {
                  Value0 :: Rhs_139 (
                  {
    let ident: rs::SpannedIdent = ident;
    assert_eq!(& * ident . node . name . as_str (  ) , "if");
    StmtRhs{conjunction: c, guard: Some(quote_expr!(cx , ( $ guard_cond ))),}
}
                  ) } else { unreachable ! (  ) } } 4u32 => {
                  let val = (
                  true , args [ 0usize ] . clone (  ) , args [ 1usize ] .
                  clone (  ) , ) ; if let (
                  true , Value0 :: G_1076 ( elems ) , Value0 :: Elem_155 (
                  elem ) , ) = val {
                  Value0 :: Conjunction_143 (
                  { let mut elems: Vec<RhsElem> = elems; elems.push(elem); elems }
                  ) } else { unreachable ! (  ) } } 5u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Elem_155 ( arg0 ) , ) = val {
                  Value0 :: ElemAnd_154 ( arg0 ) } else { unreachable ! (  ) }
                  } 6u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Tts_95 ( tts ) , ) = val {
                  Value0 :: Elem_155 (
                  {
    let tts: Vec<rs::TokenTree> = tts;
    RhsElem{pattern: quote_pat(cx , &tts[..]), positive: true,}
}
                  ) } else { unreachable ! (  ) } } 7u32 => {
                  let val = ( true , args [ 1usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Tts_95 ( tts ) , ) = val {
                  Value0 :: Elem_155 (
                  {
    let tts: Vec<rs::TokenTree> = tts;
    RhsElem{pattern: quote_pat(cx , &tts[..]), positive: false,}
}
                  ) } else { unreachable ! (  ) } } 8u32 => {
                  let val = ( true , args [ 3usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: MetaItem_165 ( value ) , ) = val {
                  Value0 :: InnerAttr_135 (
                  {
    rs::dummy_spanned(rs::Attribute_{id: rs::mk_attr_id(),
                                     style: rs::ast::AttrStyle::Inner,
                                     value: value,
                                     is_sugared_doc: false,})
}
                  ) } else { unreachable ! (  ) } } 9u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Name_137 ( name ) , ) = val {
                  Value0 :: MetaItem_165 (
                  {
    let name: rs::Spanned<rs::Name> = name;
    rs::P(rs::dummy_spanned(rs::ast::MetaItemKind::Word(name.node.as_str())))
}
                  ) } else { unreachable ! (  ) } } 10u32 => {
                  let val = (
                  true , args [ 0usize ] . clone (  ) , args [ 2usize ] .
                  clone (  ) , ) ; if let (
                  true , Value0 :: Name_137 ( name ) , Value0 ::
                  MetaItemList_180 ( items ) , ) = val {
                  Value0 :: MetaItem_165 (
                  {
    let name: rs::Spanned<rs::Name> = name;
    rs::P(rs::dummy_spanned(rs::ast::MetaItemKind::List(name.node.as_str(),
                                                        items)))
}
                  ) } else { unreachable ! (  ) } } 11u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: MetaItem_165 ( arg0 ) , ) = val {
                  Value0 :: MetaItemComma_183 ( arg0 ) } else {
                  unreachable ! (  ) } } 12u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: G_1078 ( arg0 ) , ) = val {
                  Value0 :: MetaItemList_180 ( arg0 ) } else {
                  unreachable ! (  ) } } 13u32 => {
                  let val = (
                  true , args [ 0usize ] . clone (  ) , args [ 1usize ] .
                  clone (  ) , ) ; if let (
                  true , Value0 :: G_1080 ( v ) , Value0 :: MetaItem_165 (
                  elem ) , ) = val {
                  Value0 :: MetaItemList_180 (
                  { let mut v = v; v.push(elem); v } ) } else {
                  unreachable ! (  ) } } 14u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Eq ( arg0 ) , ) = val {
                  Value0 :: Matches_138 ( arg0 ) } else { unreachable ! (  ) }
                  } 15u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: AnyToken ( t ) , ) = val {
                  Value0 :: Tt_147 (
                  {
    let (t, _): (&rs::Token, _) = t;
    rs::TokenTree::Token(rs::DUMMY_SP, (*t).clone())
}
                  ) } else { unreachable ! (  ) } } 16u32 => {
                  let val = ( true , args [ 1usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Tts_95 ( tts ) , ) = val {
                  Value0 :: Tt_147 ( { delimit(tts, rs::Bracket) } ) } else {
                  unreachable ! (  ) } } 17u32 => {
                  let val = ( true , args [ 1usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Tts_95 ( tts ) , ) = val {
                  Value0 :: Tt_147 ( { delimit(tts, rs::Paren) } ) } else {
                  unreachable ! (  ) } } 18u32 => {
                  let val = ( true , args [ 1usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Tts_95 ( tts ) , ) = val {
                  Value0 :: Tt_147 ( { delimit(tts, rs::Brace) } ) } else {
                  unreachable ! (  ) } } 19u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: G_1082 ( arg0 ) , ) = val {
                  Value0 :: Tts_95 ( arg0 ) } else { unreachable ! (  ) } }
                  20u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: Ident_145 ( i ) , ) = val {
                  Value0 :: Name_137 (
                  { let i: rs::SpannedIdent = i; rs::respan(i.span, i.node.name) }
                  ) } else { unreachable ! (  ) } } 21u32 => {
                  let val = ( true , args [ 0usize ] . clone (  ) , ) ; if let
                  ( true , Value0 :: IdentTokWithSpan ( i ) , ) = val {
                  Value0 :: Ident_145 (
                  {
    match i {
        (&rs::Token::Ident(ident), sp) => rs::respan(sp, ident),
        _ => loop  { },
    }
}
                  ) } else { unreachable ! (  ) } } 22u32 => {
                  let seq_vec = args . iter (  ) . map (
                  | arg | {
                  let val = ( true , ( * arg ) . clone (  ) ) ; if let (
                  true , Value0 :: InnerAttr_135 ( elem ) ) = val { elem }
                  else { unreachable ! (  ) } } ) . collect :: < Vec < _ >> (
                  ) ; Value0 :: G_1072 ( seq_vec ) } 23u32 => {
                  let seq_vec = args . iter (  ) . map (
                  | arg | {
                  let val = ( true , ( * arg ) . clone (  ) ) ; if let (
                  true , Value0 :: Stmt_136 ( elem ) ) = val { elem } else {
                  unreachable ! (  ) } } ) . collect :: < Vec < _ >> (  ) ;
                  Value0 :: G_1074 ( seq_vec ) } 24u32 => {
                  let seq_vec = args . iter (  ) . map (
                  | arg | {
                  let val = ( true , ( * arg ) . clone (  ) ) ; if let (
                  true , Value0 :: ElemAnd_154 ( elem ) ) = val { elem } else
                  { unreachable ! (  ) } } ) . collect :: < Vec < _ >> (  ) ;
                  Value0 :: G_1076 ( seq_vec ) } 25u32 => {
                  let seq_vec = args . iter (  ) . map (
                  | arg | {
                  let val = ( true , ( * arg ) . clone (  ) ) ; if let (
                  true , Value0 :: MetaItemComma_183 ( elem ) ) = val { elem }
                  else { unreachable ! (  ) } } ) . collect :: < Vec < _ >> (
                  ) ; Value0 :: G_1078 ( seq_vec ) } 26u32 => {
                  let seq_vec = args . iter (  ) . map (
                  | arg | {
                  let val = ( true , ( * arg ) . clone (  ) ) ; if let (
                  true , Value0 :: MetaItemComma_183 ( elem ) ) = val { elem }
                  else { unreachable ! (  ) } } ) . collect :: < Vec < _ >> (
                  ) ; Value0 :: G_1080 ( seq_vec ) } 27u32 => {
                  let seq_vec = args . iter (  ) . map (
                  | arg | {
                  let val = ( true , ( * arg ) . clone (  ) ) ; if let (
                  true , Value0 :: Tt_147 ( elem ) ) = val { elem } else {
                  unreachable ! (  ) } } ) . collect :: < Vec < _ >> (  ) ;
                  Value0 :: G_1082 ( seq_vec ) } _ => unreachable ! (
                  "rule id {}" , alt . action (  ) ) } } ; slice_builder .
                  push ( result ) ; if cartesian_product . next (  ) . is_none
                  (  ) { break ; } } } node . result (
                  slice_builder . advance_slice (  ) ) ; } } * result = match
                  root . get (  ) {
                  Evaluated { values } => values . iter (  ) , _ =>
                  unreachable ! (  ) } ; } ])
}
// END GRAMMAR
;

        let mut result = parser.parse(tokens.iter().zip(spans.iter().cloned())).unwrap();
        let stmts = result.next().unwrap().clone();
        assert!(result.next().is_none());
        stmts
    }
}
