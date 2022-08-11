pub(crate) mod actions;

use quote::format_ident;
use rustemo_rt::{
    index::{NonTermIndex, StateVec, TermIndex},
    lr::parser::Action,
};
use std::{
    fmt::Debug,
    iter::repeat,
    path::{Path, PathBuf},
};
use syn::parse_quote;

use crate::{
    error::{Error, Result},
    grammar::{types::{variant_name, to_snake_case, to_pascal_case}, Grammar, Production, NonTerminal},
    lang::rustemo_actions::Recognizer,
    settings::Settings,
    table::{lr_states_for_grammar, LRState},
};

use self::actions::generate_parser_actions;

fn action_name(
    nonterminal: &NonTerminal,
    prod: &Production,
) -> syn::Ident {
    format_ident!(
        "{}",
        to_snake_case(format!("{}_{}", nonterminal.name, variant_name(prod)))
    )
}

fn prod_kind(grammar: &Grammar, prod: &Production) -> syn::Ident {
    format_ident!(
        "{}{}",
        prod.nonterminal(grammar).name,
        if let Some(ref kind) = prod.kind {
            kind.clone()
        } else {
            format!("P{}", prod.ntidx + 1)
        }
    )
}

pub fn generate_parser<F>(
    grammar_path: F,
    out_dir: Option<F>,
    settings: &Settings,
) -> Result<()>
where
    F: AsRef<Path> + Debug,
{
    let file_name = grammar_path
        .as_ref()
        .file_name()
        .ok_or(Error::Error("Invalid grammar file name.".to_string()))?;

    let out_dir = match out_dir {
        Some(dir) => PathBuf::from(dir.as_ref()),
        None => PathBuf::from(
            grammar_path
                .as_ref()
                .parent()
                .expect("Cannot deduce parent directory of the grammar file."),
        ),
    };

    let grammar_input = std::fs::read_to_string(grammar_path.as_ref())?;
    let grammar = Grammar::from_string(grammar_input)?;

    let states = lr_states_for_grammar(&grammar, &Settings::default());

    // Generate parser definition
    let out_file = out_dir.join(file_name).with_extension("rs");
    if !settings.force && out_file.exists() {
        return Err(Error::Error(
            "Parser file already exists. Use --force to override.".to_string(),
        ));
    }

    let file_name = grammar_path
        .as_ref()
        .file_stem()
        .ok_or(Error::Error(format!(
            "Cannot deduce base file name from {:?}",
            grammar_path
        )))?
        .to_str()
        .ok_or(Error::Error(format!(
            "Cannot deduce base file name from {:?}",
            grammar_path
        )))?;
    let parser_name = to_pascal_case(file_name);
    let parser = format!("{}Parser", parser_name);
    let builder = format!("{}Builder", parser_name);
    let parser_definition = format!("{}Definition", parser);
    let lexer = format!("{}Lexer", parser_name);
    let lexer_definition = format!("{}Definition", lexer);
    let actions_file = format!("{}_actions", file_name);
    let root_symbol =
        grammar.symbol_name(grammar.nonterm_to_symbol_index(NonTermIndex(2)));

    let mut ast: syn::File =
        generate_parser_header(&grammar, &states, &actions_file)?;

    ast.items
        .extend(generate_parser_types(&grammar, &actions_file)?);

    ast.items.extend(generate_parser_definition(
        &states,
        &parser,
        &parser_definition,
        &builder,
    )?);

    ast.items.extend(generate_lexer_definition(
        &grammar,
        &states,
        &lexer_definition,
    )?);

    ast.items.extend(generate_builder(
        &grammar,
        &builder,
        &actions_file,
        &root_symbol,
    )?);

    // Generate actions
    if settings.actions {
        generate_parser_actions(&grammar, &grammar_path)?;
    }

    std::fs::write(out_file, prettyplease::unparse(&ast))?;

    Ok(())
}

fn generate_parser_header(
    grammar: &Grammar,
    states: &StateVec<LRState>,
    actions_file: &str,
) -> Result<syn::File> {
    let max_actions = states
        .iter()
        .map(|x| x.actions.iter().filter(|x| !x.is_empty()).count())
        .max()
        .unwrap();

    let term_count = grammar.terminals.len();
    let nonterm_count = grammar.nonterminals.len();
    let states_count = states.len();
    let actions_file = format_ident!("{}", actions_file);

    let header: syn::File = parse_quote! {
        /// Generated by rustemo. Do not edit manually!
        use regex::Regex;
        use num_enum::TryFromPrimitive;
        use std:: {
            convert::TryFrom,
            fmt::Debug,
        };

        use rustemo_rt::lexer::{Lexer, Token};
        use rustemo_rt::parser::Parser;
        use rustemo_rt::builder::Builder;
        use rustemo_rt::Result;
        use rustemo_rt::lr::lexer::{LRStringLexer, LRContext, LexerDefinition, RecognizerIterator};
        use rustemo_rt::lr::builder::LRBuilder;
        use rustemo_rt::lr::parser::{LRParser, ParserDefinition};
        use rustemo_rt::lr::parser::Action::{self, Shift, Reduce, Accept, Error};
        use rustemo_rt::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
        use rustemo_rt::grammar::{TerminalInfo, TerminalInfos, TerminalsState};
        use rustemo_rt::debug::{log, logn};

        use super::#actions_file;

        const TERMINAL_NO: usize = #term_count;
        const NONTERMINAL_NO: usize = #nonterm_count;
        const STATE_NO: usize = #states_count;
        const MAX_ACTIONS: usize = #max_actions;
    };

    Ok(header)
}

fn generate_parser_types(
    grammar: &Grammar,
    actions_file: &str,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let actions_file = format_ident!("{}", actions_file);

    let term_kind_variants: Vec<syn::Variant> = grammar
        .terminals
        .iter()
        .map(|t| {
            let name = format_ident!("{}", t.name);
            let idx = t.idx.0;
            parse_quote! { #name = #idx }
        })
        .collect();

    ast.push(parse_quote! {
        #[derive(Debug, Copy, Clone, TryFromPrimitive)]
        #[repr(usize)]
        pub enum TermKind {
            #(#term_kind_variants),*
        }
    });

    ast.push(parse_quote! {
        #[derive(Debug)]
        pub enum Symbol {
            Terminal(Terminal),
            NonTerminal(NonTerminal)
        }
    });

    let term_variants: Vec<syn::Variant> = grammar
        .terminals
        .iter()
        .map(|t| {
            let name = format_ident!("{}", t.name);
            if t.has_content {
                parse_quote! {
                    #name(#actions_file::#name)
                }
            } else {
                parse_quote! {
                    #name
                }
            }
        })
        .collect();

    ast.push(parse_quote! {
        #[derive(Debug)]
        pub enum Terminal {
            #(#term_variants),*
        }
    });

    let nonterm_variants: Vec<syn::Variant> = grammar.nonterminals[2..]
        .iter()
        .map(|nt| {
            let name = format_ident!("{}", nt.name);
            parse_quote! {
                #name(#actions_file::#name)
            }
        })
        .collect();

    ast.push(parse_quote! {
        #[derive(Debug)]
        pub enum NonTerminal {
            #(#nonterm_variants),*
        }
    });

    let prodkind_variants: Vec<syn::Variant> = grammar.productions[1..]
        .iter()
        .map(|prod| {
            let prod_kind = prod_kind(grammar, prod);
            let idx = prod.idx.0;
            parse_quote! {
                #prod_kind = #idx
            }
        })
        .collect();

    ast.push(parse_quote! {
        #[derive(Copy, Clone, TryFromPrimitive)]
        #[repr(usize)]
        pub enum ProdKind {
            #(#prodkind_variants),*
        }
    });

    Ok(ast)
}

fn generate_parser_definition(
    states: &StateVec<LRState>,
    parser: &str,
    parser_definition: &str,
    builder: &str,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let parser = format_ident!("{}", parser);
    let parser_definition = format_ident!("{}", parser_definition);
    let builder = format_ident!("{}", builder);

    ast.push(parse_quote! {
        pub struct #parser_definition {
            actions: [[Action; TERMINAL_NO]; STATE_NO],
            gotos: [[Option<StateIndex>; NONTERMINAL_NO]; STATE_NO]
        }

    });

    let actions: Vec<syn::Expr> = states
        .iter()
        .map(|state| {
            let actions_for_state: Vec<syn::Expr> = state
                .actions
                .iter()
                .map(|action| match action.len() {
                    0 => parse_quote! { Error },
                    1 => action_to_syntax(&action[0]),
                    _ => panic!("Multiple actions for state {}", state.idx),
                })
                .collect();
            parse_quote! {
                [#(#actions_for_state),*]
            }
        })
        .collect();

    let gotos: Vec<syn::Expr> = states
        .iter()
        .map(|state| {
            let gotos_for_state: Vec<syn::Expr> = state
                .gotos
                .iter()
                .map(|x| match x {
                    Some(state) => {
                        let idx = state.0;
                        parse_quote! { Some(StateIndex(#idx))}
                    }
                    None => parse_quote! { None },
                })
                .collect();
            parse_quote! {
                [#(#gotos_for_state),*]
            }
        })
        .collect();

    ast.push(
        parse_quote! {
            pub(in crate) static PARSER_DEFINITION: #parser_definition = #parser_definition {
                actions: [#(#actions),*],
                gotos: [#(#gotos),*],
            };
        });

    ast.push(
        parse_quote! {
            impl ParserDefinition for #parser_definition {
                fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {
                    PARSER_DEFINITION.actions[state_index.0][term_index.0]
                }
                fn goto(&self, state_index: StateIndex, nonterm_index: NonTermIndex) -> StateIndex {
                    PARSER_DEFINITION.gotos[state_index.0][nonterm_index.0].unwrap()
                }
            }
        });

    // geni!(
    //     out,
    //     "// State {}:{}\n",
    //     state.idx,
    //     grammar.symbol_name(state.symbol)
    // );

    ast.push(parse_quote! {
        pub struct #parser(LRParser<#parser_definition>);
    });

    ast.push(
        parse_quote! {
            impl<I, L, B> Parser<I, LRContext<I>, L, B> for #parser
            where
                I: Debug,
                L: Lexer<I, LRContext<I>>,
                B: LRBuilder<I>,
            {
                fn parse(&mut self, context: LRContext<I>, lexer: L, builder: B) -> Result<B::Output> {
                    #parser::default().0.parse(context, lexer, builder)
                }
            }
        });

    ast.push(
        parse_quote! {
            #[allow(dead_code)]
            impl #parser
            {
                pub fn parse_str<'i>(input: &'i str) -> Result<<#builder as Builder>::Output> {
                    let context = LRContext::new("<str>".to_string(), input);
                    let lexer = LRStringLexer::new(&LEXER_DEFINITION);
                    let builder = #builder::new();
                    #parser::default().0.parse(context, lexer, builder)
                }
            }
        });

    ast.push(parse_quote! {
        impl Default for #parser {
            fn default() -> Self {
                Self(LRParser::new(&PARSER_DEFINITION))
            }
        }
    });

    Ok(ast)
}

fn generate_lexer_definition(
    grammar: &Grammar,
    states: &StateVec<LRState>,
    lexer_definition: &str,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let lexer_definition = format_ident!("{}", lexer_definition);

    ast.push(parse_quote! {
        pub struct #lexer_definition {
            terminals: TerminalInfos<TERMINAL_NO>,
            terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,
            recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO]
        }
    });

    let terminals: Vec<syn::Expr> = grammar
        .terminals
        .iter()
        .map(|t| {
            let terminal_idx = t.idx.0;
            let terminal_name = &t.name;
            parse_quote! {
                TerminalInfo {
                    id: TermIndex(#terminal_idx),
                    name: #terminal_name,
                    location: None,
                }
            }
        })
        .collect();

    let max_actions = states
        .iter()
        .map(|x| x.actions.iter().filter(|x| !x.is_empty()).count())
        .max()
        .unwrap();
    let terminals_for_state: Vec<syn::Expr> = states
        .iter()
        .map(|state| {
            let terminals: Vec<syn::Expr> = state
                .sorted_terminals
                .iter()
                .map(|x| {
                    let x = x.0;
                    parse_quote! { Some(#x) }
                })
                .chain(
                    // Fill the rest with "None"
                    repeat(parse_quote! {None})
                        .take(max_actions - &state.sorted_terminals.len()),
                )
                .collect();

            parse_quote! {
                [#(#terminals),*]
            }
        })
        .collect();

    let mut recognizers: Vec<syn::Expr> = vec![];
    for terminal in &grammar.terminals {
        let term_name = &terminal.name;
        if let Some(recognizer) = &terminal.recognizer {
            match recognizer {
                Recognizer::StrConst(str_match) => {
                    recognizers.push(parse_quote! {
                        |input: &str| {
                            logn!("Recognizing <{}> -- ", #term_name);
                            if input.starts_with(#str_match){
                                log!("recognized");
                                Some(#str_match)
                            } else {
                                log!("not recognized");
                                None
                            }
                        }
                    });
                }
                Recognizer::RegExTerm(regex_match) => {
                    recognizers.push(parse_quote! {
                        |input: &str| {
                            logn!("Recognizing <{}> -- ", #term_name);
                            let regex = Regex::new(concat!("^", #regex_match)).unwrap();
                            let match_str = regex.find(input);
                            match match_str {
                                Some(x) => {
                                    let x_str = x.as_str();
                                    log!("recognized <{}>", x_str);
                                    Some(x_str)
                                },
                                None => {
                                    log!("not recognized");
                                    None
                                }
                            }
                        }
                    });
                }
            }
        } else {
            if terminal.idx == TermIndex(0) {
                recognizers.push(parse_quote! {
                    |input: &str| {
                        logn!("Recognizing <STOP> -- ");
                        if input.len() == 0 {
                            log!("recognized");
                            Some("")
                        } else {
                            log!("not recognized");
                            None
                        }
                    }
                });
            } else {
                // TODO: Custom recognizers?
                unreachable!()
            }
        }
    }

    ast.push(
        parse_quote!{
            pub(in crate) static LEXER_DEFINITION: #lexer_definition = #lexer_definition {
                terminals: [#(#terminals),*],
                terminals_for_state: [#(#terminals_for_state),*],
                recognizers: [#(#recognizers),*],
            };
        }
    );

    ast.push(
        parse_quote!{
            impl LexerDefinition for #lexer_definition {
                type Recognizer = for<'i> fn(&'i str) -> Option<&'i str>;

                fn recognizers(&self, state_index: StateIndex) -> RecognizerIterator<Self::Recognizer> {
                    RecognizerIterator {
                        terminals: &LEXER_DEFINITION.terminals,
                        terminals_for_state: &LEXER_DEFINITION.terminals_for_state[state_index.0][..],
                        recognizers: &LEXER_DEFINITION.recognizers,
                        index: 0
                    }
                }
            }
        }
    );

    Ok(ast)
}

fn generate_builder(
    grammar: &Grammar,
    builder: &str,
    actions_file: &str,
    root_symbol: &str,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let builder = format_ident!("{}", builder);
    let actions_file = format_ident!("{}", actions_file);
    let root_symbol = format_ident!("{}", root_symbol);

    ast.push(parse_quote! {
        pub struct #builder {
            res_stack: Vec<Symbol>,
        }
    });

    ast.push(parse_quote! {
        impl Builder for #builder
        {
            type Output = #actions_file::#root_symbol;

            fn new() -> Self {
                Self {
                    res_stack: vec![],
                }
            }

            fn get_result(&mut self) -> Result<Self::Output> {
                match self.res_stack.pop().unwrap() {
                    Symbol::NonTerminal(NonTerminal::#root_symbol(r)) => Ok(r),
                    _ => panic!("Invalid result on the parsing stack!"),
                }
            }
        }
    });

    let shift_match_arms: Vec<syn::Arm> = grammar.terminals.iter().map(|terminal| {
        let action = format_ident!("{}", to_snake_case(&terminal.name));
        let term = format_ident!("{}", terminal.name);
        if let Some(Recognizer::RegExTerm(_)) = terminal.recognizer {
            parse_quote!{
                TermKind::#term => Terminal::#term(#actions_file::#action(token))
            }
        } else {
            parse_quote!{
                TermKind::#term => Terminal::#term
            }
        }
    }).collect();

    let reduce_match_arms: Vec<syn::Arm> = grammar.productions[1..].iter().map(|production| {
        let nonterminal = &grammar.nonterminals[production.nonterminal];
        let rhs_len = production.rhs.len();
        let action = action_name(nonterminal, production);
        let prod_kind = prod_kind(grammar, production);
        let nonterminal = format_ident!("{}", nonterminal.name);

        if rhs_len == 0 {
            // Handle EMPTY reduction
            parse_quote!{
                ProdKind::#prod_kind => NonTerminal::#nonterminal(#actions_file::#action())
            }
        } else {
            // Special handling of production with only str match terms in RHS
            if production.rhs_with_content(grammar).iter().count() == 0 {
                parse_quote! {
                    ProdKind::#prod_kind => {
                        let _ = self.res_stack.split_off(self.res_stack.len()-#rhs_len).into_iter();
                        NonTerminal::#nonterminal(#actions_file::#action())
                    }
                }
            } else {
                let mut next_rep: Vec<syn::Expr> = repeat(
                    parse_quote!{ i.next().unwrap() }
                ).take(rhs_len).collect();

                let match_expr: syn::Expr = if rhs_len > 1 {
                    parse_quote!{ (#(#next_rep),*) }
                } else {
                    next_rep.pop().unwrap()
                };

                let mut param_count = 0usize;
                let match_lhs_items: Vec<syn::Expr> = production.rhs_symbols()
                                          .iter()
                                          .map( |&symbol| {
                    let param = format_ident!("p{}", param_count);
                    if grammar.symbol_has_content(symbol) {
                        param_count += 1;
                        if grammar.is_term(symbol){
                            let terminal = format_ident!("{}", grammar.symbol_to_term(symbol).name);
                            parse_quote!{ Symbol::Terminal(Terminal::#terminal(#param)) }
                        } else {
                            let nonterminal = format_ident!("{}", grammar.symbol_to_nonterm(symbol).name);
                            parse_quote!{ Symbol::NonTerminal(NonTerminal::#nonterminal(#param)) }
                        }
                    } else {
                        parse_quote! { _ }
                    }
                }).collect();

                let match_lhs: syn::Expr;
                if rhs_len > 1 {
                    match_lhs = parse_quote! { (#(#match_lhs_items),*) };
                } else {
                    match_lhs = parse_quote! { #(#match_lhs_items),* };
                }

                let params: Vec<syn::Ident> = (0..production.rhs_with_content(grammar)
                                        .iter()
                                        .count())
                    .map( |idx| format_ident! { "p{}", idx }).collect();
                parse_quote! {
                    ProdKind::#prod_kind => {
                        let mut i = self.res_stack.split_off(self.res_stack.len()-#rhs_len).into_iter();
                        match #match_expr {
                            #match_lhs => NonTerminal::#nonterminal(#actions_file::#action(#(#params),*)),
                            _ => panic!("Invalid symbol parse stack data.")
                        }

                    }
                }
            }
        }
    }).collect();

    ast.push(
        parse_quote! {
            impl<'i> LRBuilder<&'i str> for #builder {

                fn shift_action(&mut self, term_idx: TermIndex, token: Token<&'i str>) {
                    let termval = match TermKind::try_from(term_idx.0).unwrap() {
                        #(#shift_match_arms),*
                    };
                    self.res_stack.push(Symbol::Terminal(termval));
                }

                fn reduce_action(&mut self, prod_kind: ProdIndex, _prod_len: usize, _prod_str: &'static str) {
                    let prod = match ProdKind::try_from(prod_kind.0).unwrap() {
                        #(#reduce_match_arms),*
                    };
                    self.res_stack.push(Symbol::NonTerminal(prod));
                }

            }
        }
    );

    Ok(ast)
}

fn action_to_syntax(action: &Action) -> syn::Expr {
    match action {
        Action::Shift(state, term) => {
            let state = state.0;
            let term = term.0;
            parse_quote! { Shift(StateIndex(#state), TermIndex(#term)) }
        }
        Action::Reduce(prod, len, nonterm, prod_desc) => {
            let prod = prod.0;
            let nonterm = nonterm.0;
            parse_quote! { Reduce(ProdIndex(#prod), #len, NonTermIndex(#nonterm), #prod_desc) }
        }
        Action::Accept => parse_quote! { Accept },
        Action::Error => parse_quote! { Error },
    }
}
