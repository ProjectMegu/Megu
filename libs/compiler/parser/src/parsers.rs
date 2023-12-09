use crate::tokens::MeguToken;
use ast::*;

pub use megu_parser::megu_parse;

peg::parser! {
    grammar megu_parser<'a>() for [MeguToken<'a>] {
        // entry point
        pub rule megu_parse() -> Vec<AstDef> =
            n() defs:(p_def() ** mn()) n() {
                defs
            }

        // Wrapper
        // keyword
        rule t_fn() -> () = [MeguToken::DefFN] {}
        rule t_nspace() -> () = [MeguToken::DefNSpace] {}

        // regexs
        rule t_ident() -> String = [MeguToken::Ident(s)] { s.to_string() }

        // Parens
        rule t_lparen() -> () = [MeguToken::LParen] {}
        rule t_rparen() -> () = [MeguToken::RParen] {}
        rule t_lbrack() -> () = [MeguToken::LBrack] {}
        rule t_rbrack() -> () = [MeguToken::RBrack] {}

        // corons
        rule t_colon() -> () = [MeguToken::Colon] {}
        rule t_scolon() -> () = [MeguToken::Semicolon] {}

        // dot
        rule t_dot() -> () = [MeguToken::Dot] {}
        rule t_comma() -> () = [MeguToken::Comma] {}

        // must newline
        rule mn() -> () = [MeguToken::NewLine] {}

        // parse rules

        /// optional newline
        rule n() -> () = mn()? {}
        /// coron ref
        rule ref_coron() -> Vec<String> =
            name:(t_ident() ** (n() t_dot() n())) {
                name
            }

        /// type
        pub(super) rule p_type() -> AstType =
            r:ref_coron() {
                AstType {
                    refs: r
                }
            }

        /// def
        pub(super) rule p_def() -> AstDef =
            func:p_func() { return AstDef::Func(func); }
            / block_nspace:p_block_namespace() { return AstDef::NSpace(block_nspace); }
            / line_nspace:p_line_namespace() { return AstDef::LineNSpace(line_nspace); }
            

        // funcs
        /// fn
        pub(super) rule p_func() -> AstDefFunc =
            // fn ( name: type ): type [ expr ]
            t_fn() n() name:t_ident() n() args:p_func_args() n() ret:p_func_ret()? n() inner:p_func_inner()
            {
                AstDefFunc {
                    name: name.to_string(),
                    args,
                    ret,
                    inner,
                }
            }

        pub(super) rule p_func_inner() -> Vec<AstStmt> =
            t_lbrack() n() stmts:(p_stmt() ** mn()) n() t_rbrack() {
                stmts
            }

        pub(super) rule p_func_ret() -> AstType =
            t_colon() n() ret_type:p_type() {
                ret_type
            }

        pub(super) rule p_func_args() -> Vec<AstDefFuncArg> =
            t_lparen() n() args:(p_func_arg() ** (n() t_comma() n())) n() t_rparen() {
                args
            }

        pub(super) rule p_func_arg() -> AstDefFuncArg =
            name:t_ident() n() t_colon() n() arg_type:p_type() {
                AstDefFuncArg {
                    name,
                    arg_type,
                }
            }

        // namespaces
        /// namespace tree
        pub(super) rule p_namespace_tree() -> AstNameSpaceTree =
            relative: t_dot()? n() name:(t_ident() ** (n() t_dot() n())) {
                AstNameSpaceTree {
                    name,
                    relative: relative.is_some(),
                }
            }


        /// Line namespace
        pub(super) rule p_line_namespace() -> AstLineNamespace =
            t_nspace() n() tree:p_namespace_tree() {
                AstLineNamespace {
                    tree,
                }
            }
        
        /// Block namespace
        pub(super) rule p_block_namespace() -> AstBlockNamespace =
            t_nspace() n() tree:p_namespace_tree() n() t_lbrack() n() inner:(p_def() ** mn()) n() t_rbrack() {
                AstBlockNamespace {
                    tree,
                    inner,
                }
            }

        

        

        /// stmt
        pub(super) rule p_stmt() -> AstStmt =
            expr:p_expr() { AstStmt::Expr(expr) }

        /// expr
        pub(super) rule p_expr() -> AstExpr =
            expr:p_call_func() { AstExpr::CallFunc(expr) }

        /// call func
        pub(super) rule p_call_func() -> CallFunc =
            name:ref_coron() n() t_lparen() n() args:(p_expr() ** (n() t_comma() n())) n() t_rparen() {
                CallFunc {
                    name,
                    args,
                }
            }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parsers::megu_parser, tokens};

    mod types {
        use super::*;

        /// Test case for parsing a type.
        #[test]
        fn p_type_test() {
            // Input string representing a type
            let input = r#"Type1.Ref.Type2"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the type and get the result
            let result = megu_parser::p_type(&tokens);

            // Expected AST representation of the type
            let expect = ast::AstType {
                refs: vec!["Type1".to_string(), "Ref".to_string(), "Type2".to_string()],
            };

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }
    }

    mod func {
        use super::*;

        /// Test case for parsing a function definition.
        #[test]
        fn p_func_test() {
            // Input string representing a function definition
            let input = r#"fn test(aaa: Int) []"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the function definition and get the result
            let result = megu_parser::p_func(&tokens);

            // Expected AST representation of the function definition
            let expect = ast::AstDefFunc {
                name: "test".to_string(),
                args: vec![ast::AstDefFuncArg {
                    name: "aaa".to_string(),
                    arg_type: ast::AstType {
                        refs: vec!["Int".to_string()],
                    },
                }],
                ret: None,
                inner: vec![],
            };

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }

        /// Test case for parsing a function definition with a return type.
        #[test]
        fn p_func_ret_test() {
            // Input string representing a function definition with a return type
            let input = r#"fn test(): Int []"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the function definition and get the result
            let result = megu_parser::p_func(&tokens);

            // Expected AST representation of the function definition
            let expect = ast::AstDefFunc {
                name: "test".to_string(),
                args: vec![],
                ret: Some(ast::AstType {
                    refs: vec!["Int".to_string()],
                }),
                inner: vec![],
            };

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }

        /// Test case for parsing function arguments.
        #[test]
        fn p_func_args_test() {
            // Input string representing function arguments
            let input = r#"(arg1: Type1.Ref, arg2: Type2)"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the function arguments and get the result
            let result = megu_parser::p_func_args(&tokens);

            // Expected AST representation of the function arguments
            let expect = vec![
                ast::AstDefFuncArg {
                    name: "arg1".to_string(),
                    arg_type: ast::AstType {
                        refs: vec!["Type1".to_string(), "Ref".to_string()],
                    },
                },
                ast::AstDefFuncArg {
                    name: "arg2".to_string(),
                    arg_type: ast::AstType {
                        refs: vec!["Type2".to_string()],
                    },
                },
            ];

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }

        /// Test case for parsing function inner statements.
        #[test]
        fn p_func_inner_test() {
            // Input string representing function inner statements
            let input = r#"[
                call_func(call_func())
                test.call_func(call_func())
            ]"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the function inner statements and get the result
            let result = megu_parser::p_func_inner(&tokens);

            // Expected AST representation of the function inner statements
            let expect = vec![
                ast::AstStmt::Expr(ast::AstExpr::CallFunc(ast::CallFunc {
                    name: vec!["call_func".to_string()],
                    args: vec![ast::AstExpr::CallFunc(ast::CallFunc {
                        name: vec!["call_func".to_string()],
                        args: vec![],
                    })],
                })),
                ast::AstStmt::Expr(ast::AstExpr::CallFunc(ast::CallFunc {
                    name: vec!["test".to_string(), "call_func".to_string()],
                    args: vec![ast::AstExpr::CallFunc(ast::CallFunc {
                        name: vec!["call_func".to_string()],
                        args: vec![],
                    })],
                })),
            ];

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }
    }
}
