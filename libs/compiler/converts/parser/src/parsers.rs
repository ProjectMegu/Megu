use crate::tokens::MeguToken;
use ast::*;

pub use megu_parser::megu_parse;

// parser utils

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UseTree {
    name: Vec<String>,
    list: Vec<UseTree>,
}

impl UseTree {
    fn into_use(self) -> Vec<Vec<String>> {
        if self.list.is_empty() {
            vec![self.name]
        } else {
            let mut list = vec![];
            for i in self.list {
                let items = i.into_use();
                for item in items {
                    list.push([self.name.clone(), item].concat())
                }
            }
            list
        }
    }
}

fn lit(s: &str) -> String {
    let byte_start = s.char_indices().nth(1).unwrap().0;
    let byte_end = s.char_indices().nth_back(0).unwrap().0;
    let s_sub: &str = &s[byte_start..byte_end];
    s_sub.to_string()
}

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
        rule t_use() -> () = [MeguToken::DefUse] {}
        rule t_let() -> () = [MeguToken::DefLet] {}
        rule t_mut() -> () = [MeguToken::DefMut] {}

        // regexs
        rule t_ident() -> String = [MeguToken::Ident(s)] { s.to_string() }
        rule t_number() -> f64 = [MeguToken::Number(n)] { n.parse().unwrap() }
        rule t_string() -> String = [MeguToken::String(s)] { lit(s) }

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

        // equal
        rule t_equal() -> () = [MeguToken::Equal] {}

        // must newline
        rule mn() -> () = [MeguToken::NewLine] {}

        // parse rules

        /// optional newline
        rule n() -> () = mn()? {}
        /// dot ref
        rule ref_dot() -> Vec<String> =
            name:(t_ident() ++ (n() t_dot() n())) {
                name
            }

        /// type
        pub(super) rule p_type() -> AstType =
            r:ref_dot() {
                AstType {
                    refs: r
                }
            }

        /// def
        pub(super) rule p_def() -> AstDef =
            func:p_func() { return AstDef::Func(func); }
            / block_nspace:p_block_namespace() { return AstDef::NSpace(block_nspace); }
            / line_nspace:p_line_namespace() { return AstDef::LineNSpace(line_nspace); }
            / use_:p_use() { return AstDef::Use(use_); }


        // funcs
        /// fn
        pub(super) rule p_func() -> AstDefFunc =
            // fn ( name: type ): type [ expr ]
            attr:p_attr()? n() t_fn() n() name:t_ident() n() args:p_func_args() n() ret:p_func_ret()? n() inner:p_func_inner()
            {
                AstDefFunc {
                    attr,
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
            attr:p_attr()? n() t_nspace() n() tree:p_namespace_tree() {
                AstLineNamespace {
                    attr,
                    tree,
                }
            }

        /// Block namespace
        pub(super) rule p_block_namespace() -> AstBlockNamespace =
            attr:p_attr()? n() t_nspace() n() tree:p_namespace_tree() n() t_lbrack() n() inner:(p_def() ** mn()) n() t_rbrack() {
                AstBlockNamespace {
                    attr,
                    tree,
                    inner,
                }
            }

        // use
        /// use
        pub(super) rule p_use() -> AstUse =
            t_use() n() relative: t_dot()? n() tree:p_use_tree() {
                let mut list = Vec::new();
                let a = tree.into_use();
                for i in a {
                    list.push(AstNameSpaceTree {
                        name: i,
                        relative: relative.is_some(),
                    })
                }
                list
            }

        pub(super) rule p_use_tree() -> UseTree =
            name:ref_dot() n() lists:(t_dot() n() t_lbrack() n() list:(p_use_tree() ** (n() t_comma() n()) ) t_comma()? n() t_rbrack() { list })? {
                UseTree {
                    name,
                    list: lists.unwrap_or(vec![]),
                }
            }
        
        /// Attribute
        /// [Attribute("a","b")]
        pub(super) rule p_attr() -> AstAttribute =
            t_lbrack() n() name:ref_dot() n() t_lparen() n() value:(p_expr() ** (n() t_comma() n())) n() t_rparen() n() t_rbrack() {
                AstAttribute {
                    name,
                    value,
                }
            }


        /// stmt
        pub(super) rule p_stmt() -> AstStmt =
            expr:p_expr() { AstStmt::Expr(expr) }
            / def_var:p_def_var() { AstStmt::DefV(def_var) }

        /// def valiable
        pub(super) rule p_def_var() -> AstDefValue =
            // immutable
            t_let() n() name:t_ident() n() t_colon() v_type:p_type() n() t_equal() n() value:p_expr() {
                AstDefValue {
                    is_mut: false,
                    name,
                    v_type,
                    value,
                }
            }
            // mutable
            / t_mut() n() name:t_ident() n() t_colon() v_type:p_type() n() t_equal() n() value:p_expr() {
                AstDefValue {
                    is_mut: true,
                    name,
                    v_type,
                    value,
                }
            }

        /// expr
        pub(super) rule p_expr() -> AstExpr =
            expr:p_call_func() { AstExpr::CallFunc(expr) }
            / lit:p_value() { AstExpr::Lit(lit) }

        /// call func
        pub(super) rule p_call_func() -> CallFunc =
            name:ref_dot() n() t_lparen() n() args:(p_expr() ** (n() t_comma() n())) n() t_rparen() {
                CallFunc {
                    name,
                    args,
                }
            }
        /// values
        pub(super) rule p_value() -> AstLitValues =
            number:t_number() { AstLitValues::Number(number) }
            / string:t_string() { AstLitValues::String(string) }
        
        
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
                attr: None,
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
                attr: None,
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

    mod nspace {
        use super::*;

        /// Test case for parsing a line namespace.
        #[test]
        fn p_line_namespace_test() {
            // Input string representing a line namespace
            let input = r#"nspace .test"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the line namespace and get the result
            let result = megu_parser::p_line_namespace(&tokens);

            // Expected AST representation of the line namespace
            let expect = ast::AstLineNamespace {
                attr: None,
                tree: ast::AstNameSpaceTree {
                    name: vec!["test".to_string()],
                    relative: true,
                },
            };

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }

        /// Test case for parsing a block namespace.
        #[test]
        fn p_block_namespace_test() {
            // Input string representing a block namespace
            let input = r#"nspace .test [
                fn test() []
            ]"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the block namespace and get the result
            let result = megu_parser::p_block_namespace(&tokens);

            // Expected AST representation of the block namespace
            let expect = ast::AstBlockNamespace {
                attr: None,
                tree: ast::AstNameSpaceTree {
                    name: vec!["test".to_string()],
                    relative: true,
                },
                inner: vec![ast::AstDef::Func(ast::AstDefFunc {
                    attr: None,
                    name: "test".to_string(),
                    args: vec![],
                    ret: None,
                    inner: vec![],
                })],
            };

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }
    }

    mod use_ {
        use super::*;

        /// Test case for parsing a use statement.
        #[test]
        fn p_use_test() {
            // Input string representing a use statement
            let input = r#"use .test"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the use statement and get the result
            let result = megu_parser::p_use(&tokens);

            // Expected AST representation of the use statement
            let expect = vec![ast::AstNameSpaceTree {
                name: vec!["test".to_string()],
                relative: true,
            }];

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }

        /// Test case for parsing a use statement with a list.
        #[test]
        fn p_use_list_test() {
            // Input string representing a use statement with a list
            let input = r#"use .test.[
                test1,
                test2,
                test3,
            ]"#;

            // Tokenize the input string
            let tokens = dbg!(tokens::lexer(input));

            // Parse the use statement and get the result
            let result = megu_parser::p_use(&tokens);

            // Expected AST representation of the use statement
            let expect = vec![
                ast::AstNameSpaceTree {
                    name: vec!["test".to_string(), "test1".to_string()],
                    relative: true,
                },
                ast::AstNameSpaceTree {
                    name: vec!["test".to_string(), "test2".to_string()],
                    relative: true,
                },
                ast::AstNameSpaceTree {
                    name: vec!["test".to_string(), "test3".to_string()],
                    relative: true,
                },
            ];

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }
    }

    mod stmt {
        use super::*;

        /// Test case for parsing a statement.
        #[test]
        fn p_stmt_test() {
            // Input string representing a statement
            let input = r#"call_func(call_func())"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the statement and get the result
            let result = megu_parser::p_stmt(&tokens);

            // Expected AST representation of the statement
            let expect = ast::AstStmt::Expr(ast::AstExpr::CallFunc(ast::CallFunc {
                name: vec!["call_func".to_string()],
                args: vec![ast::AstExpr::CallFunc(ast::CallFunc {
                    name: vec!["call_func".to_string()],
                    args: vec![],
                })],
            }));

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }

        /// Test case for parsing a variable definition.
        #[test]
        fn p_def_var_test() {
            // Input string representing a variable definition
            let input = r#"let test: Type1.Ref = call_func(call_func())"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the variable definition and get the result
            let result = megu_parser::p_def_var(&tokens);

            // Expected AST representation of the variable definition
            let expect = ast::AstDefValue {
                is_mut: false,
                name: "test".to_string(),
                v_type: ast::AstType {
                    refs: vec!["Type1".to_string(), "Ref".to_string()],
                },
                value: ast::AstExpr::CallFunc(ast::CallFunc {
                    name: vec!["call_func".to_string()],
                    args: vec![ast::AstExpr::CallFunc(ast::CallFunc {
                        name: vec!["call_func".to_string()],
                        args: vec![],
                    })],
                }),
            };

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }

        /// Test case for parsing a mutable variable definition.
        #[test]
        fn p_def_var_mut_test() {
            // Input string representing a mutable variable definition
            let input = r#"mut test: Type1.Ref = call_func(call_func())"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the mutable variable definition and get the result
            let result = megu_parser::p_def_var(&tokens);

            // Expected AST representation of the mutable variable definition
            let expect = ast::AstDefValue {
                is_mut: true,
                name: "test".to_string(),
                v_type: ast::AstType {
                    refs: vec!["Type1".to_string(), "Ref".to_string()],
                },
                value: ast::AstExpr::CallFunc(ast::CallFunc {
                    name: vec!["call_func".to_string()],
                    args: vec![ast::AstExpr::CallFunc(ast::CallFunc {
                        name: vec!["call_func".to_string()],
                        args: vec![],
                    })],
                }),
            };

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }
    }

    mod attr {
        use super::*;

        /// Test case for parsing an attribute.
        #[test]
        fn p_attribute_test() {
            // Input string representing an attribute
            let input = r#"[Attribute("a", "b")]"#;

            // Tokenize the input string
            let tokens = tokens::lexer(input);

            // Parse the attribute and get the result
            let result = megu_parser::p_attr(&tokens);

            // Expected AST representation of the attribute
            let expect = ast::AstAttribute {
                name: vec!["Attribute".to_string()],
                value: vec![
                    ast::AstExpr::Lit(ast::AstLitValues::String("a".to_string())),
                    ast::AstExpr::Lit(ast::AstLitValues::String("b".to_string())),
                ],
            };

            // Assert that the result matches the expected AST
            assert_eq!(result, Ok(expect));
        }
    }
}
