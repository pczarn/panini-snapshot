#![feature(plugin, plugin_registrar, rustc_private)]
#![plugin(quasi_macros)]

extern crate syntax;
extern crate rustc_plugin;

extern crate aster;
extern crate quasi;

extern crate cfg;
extern crate gearley;

extern crate enum_stream_codegen;
extern crate panini_codegen;
extern crate panini;

mod enum_stream_parser;
mod grammar_parser;
mod util;

pub use self::enum_stream_parser::Parser as EnumStreamParser;
pub use self::grammar_parser::Parser;
pub use panini_codegen::front::ast::{Stmts, Stmt, Rhs, RhsElement, RhsAst, Action, Sequence};

pub type Name = rs::Spanned<rs::Name>;

use panini_codegen::rs;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut ::rustc_plugin::Registry) {
    reg.register_macro("grammar", expand_grammar);
    reg.register_macro("enum_stream", expand_enum_stream);
}

fn expand_grammar<'cx>(ecx: &'cx mut rs::ExtCtxt,
                       sp: rs::Span,
                       tts: &[rs::TokenTree]) -> Box<rs::MacResult + 'cx> {
    let stmts = Parser::new().parse_grammar_from_tts(ecx, tts);
    panini_codegen::codegen(ecx, sp, stmts)
}

fn expand_enum_stream<'cx>(ecx: &'cx mut rs::ExtCtxt,
                           sp: rs::Span,
                           tts: &[rs::TokenTree]) -> Box<rs::MacResult + 'cx> {
    let stmts = EnumStreamParser::new().parse_from_tts(ecx, tts);
    enum_stream_codegen::codegen(ecx, sp, stmts)
}

#[test]
fn test_simple_parse() {
    rs::with_fake_extctxt(|ecx| {
        let (start, a, b) = (
            rs::str_to_ident("start"),
            rs::str_to_ident("a"),
            rs::str_to_ident("b"),
        );
        let mut parser = Parser::new();
        let tokens = quote_tokens!(ecx, $start ::= $a $b;);
        let result = parser.parse_grammar_from_tts(ecx,
            // &[
            //     rs::Token::Ident(start, rs::Plain),
            //     rs::Token::ModSep,
            //     rs::Token::Eq,
            //     rs::Token::Ident(a, rs::Plain),
            //     rs::Token::Ident(b, rs::Plain),
            //     rs::Token::Semi,
            // ],
            // &[
            //     rs::DUMMY_SP; 6
            // ]
            &tokens[..],
        );
        let (start, a, b) = (
            rs::dummy_spanned(start.name),
            rs::dummy_spanned(a.name),
            rs::dummy_spanned(b.name),
        );
        let after = Stmts {
            attrs: vec![],
            stmts: vec![
                Stmt {
                    lhs: start,
                    rhs: vec![(
                        Rhs(vec![
                            RhsElement {
                                bind: None,
                                elem: RhsAst::Symbol(a),
                            },
                            RhsElement {
                                bind: None,
                                elem: RhsAst::Symbol(b),
                            }
                        ]),
                        Action {
                            expr: None,//Some(quote_expr!(ecx, nothing)),
                        },
                    )],
                    ty: None,//Some(quote_ty!(ecx, X))//Ty::None,
                    span: rs::DUMMY_SP,
                },
            ],
            lexer: None
        };
        assert_eq!(result, after);
        // let result = result.into_hir();
        // let after = hir::Stmts {
        //     stmts: vec![
        //         hir::Stmt {
        //             lhs: "start",
        //             rhs: Rhs {
        //                 rhs: vec!["a", "b"],
        //                 binds: vec![],
        //             },
        //             action: Action {
        //                 expr: quote_expr!(ecx, nothing),
        //             },
        //             ty: Ty::None,
        //         }
        //     ]
        // };
        // assert_eq!(result, after);
    });
}

// #[test]
// fn test_seq_parse() {
//     rs::with_fake_extctxt(|ecx| {
//         let (start, a, b) = (
//             rs::str_to_ident("start"),
//             rs::str_to_ident("a"),
//             rs::str_to_ident("b"),
//         );
//         let mut parser = Parser::new();
//         let tokens = quote_tokens!(ecx,
//             $start ::= $b*;// | $a $b $b $a;
//             $a ::= $b $b $a;
//         );
//         let result = parser.parse_grammar_from_tts(ecx,
//             &tokens[..],
//         );
//         let after = Stmts {
//             stmts: vec![
//                 Stmt {
//                     lhs: start.name,
//                     rhs: Rhs(vec![
//                         RhsElement {
//                             bind: None,
//                             elem: RhsAst::Sequence(Sequence {
//                                 rhs: Rhs(vec![
//                                     RhsElement {
//                                         bind: None,
//                                         elem: RhsAst::Symbol(b.name),
//                                     },
//                                 ]),
//                                 min: 0,
//                                 max: None,
//                             }),
//                         },
//                     ]),
//                     action: Action { expr: None },
//                     ty: None,
//                     span: rs::DUMMY_SP,
//                 },
//                 Stmt {
//                     lhs: a.name,
//                     rhs: Rhs(vec![
//                         RhsElement {
//                             bind: None,
//                             elem: RhsAst::Product(Rhs(vec![
//                                 RhsElement {
//                                     bind: None,
//                                     elem: RhsAst::Symbol(b.name),
//                                 },
//                                 RhsElement {
//                                     bind: None,
//                                     elem: RhsAst::Symbol(b.name),
//                                 },
//                                 RhsElement {
//                                     bind: None,
//                                     elem: RhsAst::Symbol(a.name),
//                                 },
//                             ]))
//                         }
//                     ]),
//                     action: Action {
//                         expr: None,
//                     },
//                     ty: None,
//                     span: rs::DUMMY_SP,
//                 },
//             ],
//         };
//         assert_eq!(result, after);
//     });
// }

/*
#[test]
fn test_bind_parse() {
    rs::with_fake_extctxt(|ecx| {
        let (start, a, b) = (
            rs::str_to_ident("start"),
            rs::str_to_ident("a"),
            rs::str_to_ident("b"),
        );
        let mut parser = Parser::new();
        let tokens = quote_tokens!(ecx,
            $start ::= $a:$a $b:$b;
        );
        let result = parser.parse_grammar_from_tts(ecx,
            &tokens[..],
        );
        let after = Stmts {
            stmts: vec![
                Stmt {
                    lhs: start.name,
                    rhs: Rhs {
                        rhs: vec![
                            RhsElement {
                                bind: None,
                                elem: RhsAst::AnyAlt(Rhs {
                                    rhs: vec![
                                        RhsElement {
                                            bind: Some(quote_pat!(ecx, $a)),
                                            elem: RhsAst::Symbol(a.name),
                                        },
                                        RhsElement {
                                            bind: Some(quote_pat!(ecx, $b)),
                                            elem: RhsAst::Symbol(b.name),
                                        }
                                    ]
                                })
                            }
                        ]
                    },
                    action: Action { expr: None },
                    ty: None,
                },
            ],
        };
        assert_eq!(result, after);
    });
}
*/

#[test]
fn test_full_parse() {
    rs::with_fake_extctxt(|ecx| {
        let (start, a, b) = (
            rs::str_to_ident("start"),
            rs::str_to_ident("a"),
            rs::str_to_ident("b"),
        );
        let mut parser = Parser::new();
        let tokens = quote_tokens!(ecx,
            $start -> Test ::= $a $b;
        );
        let result = parser.parse_grammar_from_tts(ecx,
            &tokens[..],
        );
        let (start, a, b) = (
            rs::dummy_spanned(start.name),
            rs::dummy_spanned(a.name),
            rs::dummy_spanned(b.name),
        );
        let after = Stmts {
            attrs: vec![],
            stmts: vec![
                Stmt {
                    lhs: start,
                    rhs: vec![(
                        Rhs(vec![
                            RhsElement {
                                bind: None,
                                elem: RhsAst::Symbol(a),
                            },
                            RhsElement {
                                bind: None,
                                elem: RhsAst::Symbol(b),
                            }
                        ]),
                        Action { expr: None },
                    )],
                    ty: Some(quote_ty!(ecx, Test)),
                    span: rs::DUMMY_SP,
                },
            ],
            lexer: None
        };
        assert_eq!(result, after);
    });
}

// #[test]
// fn test_transform() {
//     // let before = Stmts {
//     //     stmts: vec![
//     //         Stmt {
//     //             lhs: "start",
//     //             rhs: Rhs {
//     //                 rhs: vec![
//     //                     RhsElement {
//     //                         bind: None,
//     //                         elem: RhsAst::Name("abc"),
//     //                     }
//     //                 ]
//     //             },
//     //             action: Action {
//     //                 expr: quote_expr!(nothing),
//     //             },
//     //             ty: Ty::None,
//     //         },
//     //     ],
//     // };
//     // let after = ir::Stmts {
//     let before = hir::Stmts {
//         stmts: vec![
//             ir::Stmt {
//                 lhs: "start",
//                 rhs: Rhs {
//                     rhs: vec!["a", "b"],
//                     binds: vec![],
//                 },
//                 action: Action {
//                     expr: quote_expr!(nothing),
//                 },
//                 ty: Ty::None,
//             }
//         ]
//     };
//     let rules_after = []
//     let result = middle::transform(before);
//     result.rules
//     assert_eq!(, after);
// }
