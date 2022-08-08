//! Inferring types from rustemo grammars.
//! This is a base support for auto AST inference.

use convert_case::{Case, Casing};

use super::Grammar;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) struct SymbolTypes {
    symbol_types: Vec<SymbolType>,
}

impl SymbolTypes {
    pub fn new(grammar: &Grammar) -> Self {
        Self {
            symbol_types: Self::symbol_types(grammar),
        }
    }

    pub(crate) fn get_type(&self, ty: &str) -> &SymbolType {
        self.symbol_types.iter().find(|t| t.name == ty).unwrap()
    }

    /// Returns a vector of all types inferred from the provided grammar.
    pub(crate) fn symbol_types(grammar: &Grammar) -> Vec<SymbolType> {
        let mut types = vec![];
        for terminal in &grammar.terminals {
            // Each terminal produces `Terminal` kind which maps to String by default
            types.push(SymbolType {
                name: terminal.name.clone(),
                kind: SymbolTypeKind::Terminal,
                optional: false,
            })
        }

        // Each non-terminal produces Enum type
        for nonterminal in &grammar.nonterminals {
            let mut variants = vec![];

            for production in &nonterminal.productions(grammar) {
                if production.rhs.len() == 0 {
                    // Empty production
                    continue;
                }
                let variant_name = format!(
                    "{}_{}",
                    nonterminal.name,
                    production
                        .kind
                        .as_ref()
                        .map_or((production.ntidx + 1).to_string(), |k| k.clone())
                );

                // Enum variants are deduced by the following rules:
                // - No content references => plain variant without inner content
                // - A single content. ref and no assig LHS => variant with
                //   a referred NT type as its content
                // - Multiple content. refs => Variant with a new struct type
                //   where fields types are types of the referred symbols.
                let rhs = production.rhs_with_content(grammar);
                variants.push(match rhs.iter().count() {
                    0 => Variant {
                        name: variant_name,
                        kind: VariantKind::Plain,
                    },
                    1 if rhs[0].name.is_none() => {
                        let ref_type = grammar.symbol_name(rhs[0].symbol);
                        Variant {
                            name: variant_name,
                            kind: VariantKind::Ref(ref_type.clone()),
                        }
                    }
                    _ => {
                        let mut fields = vec![];
                        for assign in rhs {
                            let ref_type = grammar.symbol_name(assign.symbol);
                            let name = assign.name.unwrap_or(format!(
                                "{}_{}",
                                ref_type.to_case(Case::Snake),
                                assign.idx + 1
                            ));
                            fields.push(Field {
                                name,
                                ty: ref_type.clone(),
                                recursive: ref_type == nonterminal.name,
                            })
                        }
                        Variant {
                            name: variant_name.clone(),
                            kind: VariantKind::Struct(fields),
                        }
                    }
                });
            }

            // If NT has empty production type is optional
            let type_optional = nonterminal
                .productions(grammar)
                .iter()
                .find(|p| p.rhs.len() == 0)
                .is_some();
            types.push(SymbolType {
                name: nonterminal.name.clone(),
                kind: SymbolTypeKind::Enum(variants),
                optional: type_optional,
            });
        }
        types
    }
}


#[derive(Debug)]
pub(crate) struct SymbolType {
    pub name: String,
    pub kind: SymbolTypeKind,
    pub optional: bool,
}

#[derive(Debug)]
pub(crate) enum SymbolTypeKind {
    Enum(Vec<Variant>),
    Terminal,
}

#[derive(Debug)]
pub(crate) struct Variant {
    pub name: String,
    pub kind: VariantKind,
}

#[derive(Debug)]
pub(crate) enum VariantKind {
    Plain,
    Struct(Vec<Field>),
    Ref(String),
}

#[derive(Debug)]
pub(crate) struct Field {
    pub name: String,
    pub ty: String,
    pub recursive: bool,
}