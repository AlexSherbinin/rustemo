use indexmap::IndexMap;

use rustemort::lexer::Token;

pub type StrConst = String;
pub fn str_const<'i>(token: Token<&'i str>) -> StrConst {
    token.value.trim_matches('\'').trim_matches('"').into()
}

pub type Name = String;
pub fn name<'a>(token: Token<&'a str>) -> Name {
    token.value.into()
}

pub type IntConst = u32;
pub fn int_const<'a>(token: Token<&'a str>) -> IntConst {
    token.value.parse().unwrap()
}

pub type FloatConst = f32;
pub fn float_const<'a>(token: Token<&'a str>) -> FloatConst {
    token.value.parse().unwrap()
}

pub type BoolConst = bool;
pub fn bool_const<'a>(token: Token<&'a str>) -> BoolConst {
    if token.value == "true" {
        true
    } else {
        false
    }
}

pub type Action = String;
pub fn action<'a>(token: Token<&'a str>) -> Action {
    token.value.into()
}

pub type RegExTerm = String;
pub fn reg_ex_term<'a>(token: Token<&'a str>) -> RegExTerm {
    token.value.trim_matches('/').into()
}

pub type WS = String;
pub fn ws<'a>(token: Token<&'a str>) -> WS {
    token.value.into()
}

pub type CommentLine = String;
pub fn comment_line<'a>(token: Token<&'a str>) -> CommentLine {
    token.value.into()
}

pub type NotComment = String;
pub fn not_comment<'a>(token: Token<&'a str>) -> NotComment {
    token.value.into()
}

pub type STOP = ();
pub fn stop<'a>(token: Token<&'a str>) -> STOP {}

#[derive(Debug)]
pub struct PGFile {
    pub imports: Option<Imports>,
    pub rules: Option<GrammarRules>,
    pub terminals: Option<Terminals>,
}
pub fn pgfile_p0(rules: GrammarRules) -> PGFile {
    PGFile {
        imports: None,
        rules: Some(rules),
        terminals: None,
    }
}
pub fn pgfile_p1(imports: Imports, rules: GrammarRules) -> PGFile {
    PGFile {
        imports: Some(imports),
        rules: Some(rules),
        terminals: None,
    }
}
pub fn pgfile_p2(rules: GrammarRules, terminals: Terminals) -> PGFile {
    PGFile {
        imports: None,
        rules: Some(rules),
        terminals: Some(terminals),
    }
}
pub fn pgfile_p3(
    imports: Imports,
    rules: GrammarRules,
    terminals: Terminals,
) -> PGFile {
    PGFile {
        imports: Some(imports),
        rules: Some(rules),
        terminals: Some(terminals),
    }
}
pub fn pgfile_p4(terminals: Terminals) -> PGFile {
    PGFile {
        imports: None,
        rules: None,
        terminals: Some(terminals),
    }
}

pub type Imports = Vec<Import>;
pub fn imports_p0(mut imports: Imports, import: Import) -> Imports {
    imports.push(import);
    imports
}
pub fn imports_p1(import: Import) -> Imports {
    vec![import]
}

#[derive(Debug)]
pub struct Import {
    pub path: String,
    pub name: Option<String>,
}
pub fn import_p0(path: StrConst) -> Import {
    Import { path, name: None }
}
pub fn import_p1(path: StrConst, name: Name) -> Import {
    Import {
        path,
        name: Some(name),
    }
}

pub type GrammarRules = Vec<GrammarRule>;
pub type ProductionRules = GrammarRules;
pub fn production_rules_p0(
    mut rules: GrammarRules,
    rule: GrammarRule,
) -> GrammarRules {
    rules.push(rule);
    rules
}
pub fn production_rules_p1(rule: GrammarRule) -> GrammarRules {
    vec![rule]
}

#[derive(Debug)]
pub struct GrammarRule {
    pub action: Option<String>,
    pub name: String,
    pub rhs: GrammarRuleRHS,
    pub meta: ProductionMetaDatas,
}
pub fn production_rule_with_action_p0(
    action: String,
    mut rule: GrammarRule,
) -> GrammarRule {
    rule.action = Some(action);
    rule
}
pub fn production_rule_with_action_p1(rule: GrammarRule) -> GrammarRule {
    rule
}

pub type ProductionRule = GrammarRule;
pub type ProductionRuleWithAction = GrammarRule;
pub fn production_rule_p0(name: String, rhs: GrammarRuleRHS) -> GrammarRule {
    GrammarRule {
        name,
        rhs,
        action: None,
        meta: ProductionMetaDatas::new(),
    }
}
pub fn production_rule_p1(
    name: String,
    meta: ProductionMetaDatas,
    rhs: GrammarRuleRHS,
) -> GrammarRule {
    GrammarRule { name, rhs, meta, action: None}
}

pub type GrammarRuleRHS = Vec<Production>;
pub type ProductionRuleRHS = GrammarRuleRHS;
pub fn production_rule_rhsp0(mut rhs: GrammarRuleRHS, prod: Production) -> GrammarRuleRHS {
    rhs.push(prod);
    rhs
}
pub fn production_rule_rhsp1(prod: Production) -> GrammarRuleRHS {
    vec![prod]
}

#[derive(Debug)]
pub struct Production {
    pub assignments: Assignments,
    pub meta: ProductionMetaDatas,
}
pub fn production_p0(assignments: Assignments) -> Production {
    Production {
        assignments,
        meta: ProductionMetaDatas::new(),
    }
}
pub fn production_p1(assignments: Assignments, meta: ProductionMetaDatas) -> Production {
    Production { assignments, meta }
}

pub type Terminals = Vec<Terminal>;
pub type TerminalRules = Terminals;
pub type TerminalRuleWithAction = Terminal;
pub fn terminal_rules_p0(mut rules: Terminals, rule: Terminal) -> Terminals {
    rules.push(rule);
    rules
}
pub fn terminal_rules_p1(rule: Terminal) -> Terminals {
    vec![rule]
}

pub fn terminal_rule_with_action_p0(action: String, mut rule: Terminal) -> Terminal {
    rule.action = Some(action);
    rule
}
pub fn terminal_rule_with_action_p1(rule: Terminal) -> Terminal {
    rule
}

#[derive(Debug)]
pub struct Terminal {
    pub name: String,
    pub action: Option<String>,
    pub recognizer: Option<Recognizer>,
    pub meta: TerminalMetaDatas,
}
pub type TerminalRule = Terminal;
pub fn terminal_rule_p0(name: String, recognizer: Recognizer) -> Terminal {
    Terminal {
        name,
        action: None,
        recognizer: Some(recognizer),
        meta: TerminalMetaDatas::new(),
    }
}
pub fn terminal_rule_p1(name: String) -> Terminal {
    Terminal {
        name,
        action: None,
        recognizer: None,
        meta: TerminalMetaDatas::new(),
    }
}
pub fn terminal_rule_p2(
    name: String,
    recognizer: Recognizer,
    meta: TerminalMetaDatas,
) -> Terminal {
    Terminal {
        name,
        action: None,
        recognizer: Some(recognizer),
        meta,
    }
}
pub fn terminal_rule_p3(name: String, meta: TerminalMetaDatas) -> Terminal {
    Terminal {
        name,
        action: None,
        recognizer: None,
        meta,
    }
}

#[derive(Debug)]
pub enum Associativity {
    None,
    Left,
    Right,
}
pub type ProductionMetaData = IndexMap<String, Const>;

pub fn production_meta_data_p0() -> ProductionMetaData {
    ProductionMetaData::from([("left".into(), Const::Bool(true))])
}
pub fn production_meta_data_p1() -> ProductionMetaData {
    ProductionMetaData::from([("left".into(), Const::Bool(true))])
}
pub fn production_meta_data_p2() -> ProductionMetaData {
    ProductionMetaData::from([("right".into(), Const::Bool(true))])
}
pub fn production_meta_data_p3() -> ProductionMetaData {
    ProductionMetaData::from([("right".into(), Const::Bool(true))])
}
pub fn production_meta_data_p4() -> ProductionMetaData {
    ProductionMetaData::from([("dynamic".into(), Const::Bool(true))])
}
pub fn production_meta_data_p5() -> ProductionMetaData {
    ProductionMetaData::from([("nops".into(), Const::Bool(true))])
}
pub fn production_meta_data_p6() -> ProductionMetaData {
    ProductionMetaData::from([("nopse".into(), Const::Bool(true))])
}
pub fn production_meta_data_p7(prio: IntConst) -> ProductionMetaData {
    ProductionMetaData::from([("priority".into(), Const::Int(prio))])
}
pub fn production_meta_data_p8(user: UserMetaData) -> ProductionMetaData {
    ProductionMetaData::from([(user.name, user.value)])
}

pub type ProductionMetaDatas = ProductionMetaData;
pub fn production_meta_datas_p0(
    mut metas: ProductionMetaDatas,
    meta: ProductionMetaData,
) -> ProductionMetaDatas {
    metas.extend(meta);
    metas
}
pub fn production_meta_datas_p1(meta: ProductionMetaData) -> ProductionMetaDatas {
    meta
}

pub type TerminalMetaData = IndexMap<String, Const>;
pub fn terminal_meta_data_p0() -> TerminalMetaData {
    TerminalMetaData::from([("prefer".into(), Const::Bool(true))])
}
pub fn terminal_meta_data_p1() -> TerminalMetaData {
    TerminalMetaData::from([("finish".into(), Const::Bool(true))])
}
pub fn terminal_meta_data_p2() -> TerminalMetaData {
    TerminalMetaData::from([("nofinish".into(), Const::Bool(true))])
}
pub fn terminal_meta_data_p3() -> TerminalMetaData {
    TerminalMetaData::from([("dynamic".into(), Const::Bool(true))])
}
pub fn terminal_meta_data_p4(prio: IntConst) -> TerminalMetaData {
    TerminalMetaData::from([("priority".into(), Const::Int(prio))])
}
pub fn terminal_meta_data_p5(user: UserMetaData) -> TerminalMetaData {
    TerminalMetaData::from([(user.name, user.value)])
}

pub type TerminalMetaDatas = TerminalMetaData;
pub fn terminal_meta_datas_p0(
    mut metas: TerminalMetaDatas,
    meta: TerminalMetaData,
) -> TerminalMetaDatas {
    metas.extend(meta);
    metas
}
pub fn terminal_meta_datas_p1(meta: TerminalMetaData) -> TerminalMetaDatas {
    meta
}

#[derive(Debug)]
pub struct UserMetaData {
    name: Name,
    value: Const,
}
pub fn user_meta_data_p0(name: Name, value: Const) -> UserMetaData {
    UserMetaData { name, value }
}

#[derive(Debug)]
pub enum Const {
    Int(u32),
    Float(f32),
    Bool(bool),
    String(String),
}
pub fn const_p0(int_const: IntConst) -> Const {
    Const::Int(int_const)
}
pub fn const_p1(float_const: FloatConst) -> Const {
    Const::Float(float_const)
}
pub fn const_p2(bool_const: BoolConst) -> Const {
    Const::Bool(bool_const)
}
pub fn const_p3(str_const: StrConst) -> Const {
    Const::String(str_const)
}

#[derive(Debug)]
pub enum Assignment {
    PlainAssignment(PlainAssignment),
    BoolAssignment(BoolAssignment),
    GSymbolReference(GrammarSymbolReference),
}
pub fn assignment_p0(assig: PlainAssignment) -> Assignment {
    Assignment::PlainAssignment(assig)
}
pub fn assignment_p1(assig: BoolAssignment) -> Assignment {
    Assignment::BoolAssignment(assig)
}
pub fn assignment_p2(gsymref: GrammarSymbolReference) -> Assignment {
    Assignment::GSymbolReference(gsymref)
}

pub type Assignments = Vec<Assignment>;
pub fn assignments_p0(mut assigns: Assignments, assign: Assignment) -> Assignments {
    assigns.push(assign);
    assigns
}
pub fn assignments_p1(assign: Assignment) -> Assignments {
    vec![assign]
}

#[derive(Debug)]
pub struct PlainAssignment {
    pub name: Name,
    pub gsymref: GrammarSymbolReference,
}
pub fn plain_assignment_p0(name: Name, gsymref: GrammarSymbolReference) -> PlainAssignment {
    PlainAssignment { name, gsymref }
}

pub type BoolAssignment = PlainAssignment;
pub fn bool_assignment_p0(name: Name, gsymref: GrammarSymbolReference) -> BoolAssignment {
    BoolAssignment { name, gsymref }
}

#[derive(Debug)]
pub struct ProductionGroup(pub GrammarRuleRHS);
pub fn production_group_p0(prod_rule_rhs: GrammarRuleRHS) -> ProductionGroup {
    ProductionGroup(prod_rule_rhs)
}

#[derive(Debug)]
pub struct GrammarSymbolReference {
    pub gsymbol: Option<GrammarSymbol>,
    pub repeat_operator: OptRepeatOperator,
    pub production_group: Option<ProductionGroup>,
}
pub fn grammar_symbol_reference_p0(
    gsymbol: GrammarSymbol,
    repeat_operator: OptRepeatOperator,
) -> GrammarSymbolReference {
    GrammarSymbolReference {
        gsymbol: Some(gsymbol),
        repeat_operator,
        production_group: None,
    }
}
pub fn grammar_symbol_reference_p1(
    prod_group: ProductionGroup,
    repeat_operator: OptRepeatOperator,
) -> GrammarSymbolReference {
    GrammarSymbolReference {
        gsymbol: None,
        repeat_operator,
        production_group: Some(prod_group),
    }
}

pub type OptRepeatOperator = Option<RepeatOperator>;
pub fn opt_repeat_operator_p0(repop: RepeatOperator) -> OptRepeatOperator {
    Some(repop)
}
pub fn opt_repeat_operator_p1() -> OptRepeatOperator {
    None
}

#[derive(Debug)]
pub enum RepeatOperatorEnum {
    ZeroOrMore,
    ZeroOrMoreGreedy,
    OneOrMore,
    OneOrMoreGreedy,
    Optional,
    OptionalGreedy,
}
#[derive(Debug)]
pub struct RepeatOperator {
    pub operator: RepeatOperatorEnum,
    pub rep_modifiers: OptionalRepeatModifiersExpression,
}
pub fn repeat_operator_p0(rep_modifiers: OptionalRepeatModifiersExpression) -> RepeatOperator {
    RepeatOperator {
        operator: RepeatOperatorEnum::ZeroOrMore,
        rep_modifiers,
    }
}
pub fn repeat_operator_p1(rep_modifiers: OptionalRepeatModifiersExpression) -> RepeatOperator {
    RepeatOperator {
        operator: RepeatOperatorEnum::ZeroOrMoreGreedy,
        rep_modifiers,
    }
}
pub fn repeat_operator_p2(rep_modifiers: OptionalRepeatModifiersExpression) -> RepeatOperator {
    RepeatOperator {
        operator: RepeatOperatorEnum::OneOrMore,
        rep_modifiers,
    }
}
pub fn repeat_operator_p3(rep_modifiers: OptionalRepeatModifiersExpression) -> RepeatOperator {
    RepeatOperator {
        operator: RepeatOperatorEnum::OneOrMoreGreedy,
        rep_modifiers,
    }
}
pub fn repeat_operator_p4(rep_modifiers: OptionalRepeatModifiersExpression) -> RepeatOperator {
    RepeatOperator {
        operator: RepeatOperatorEnum::Optional,
        rep_modifiers,
    }
}
pub fn repeat_operator_p5(rep_modifiers: OptionalRepeatModifiersExpression) -> RepeatOperator {
    RepeatOperator {
        operator: RepeatOperatorEnum::OptionalGreedy,
        rep_modifiers,
    }
}

pub type OptionalRepeatModifiersExpression = Option<OptionalRepeatModifiers>;
pub fn optional_repeat_modifiers_expression_p0(
    opt_rep_modifiers: OptionalRepeatModifiers,
) -> OptionalRepeatModifiersExpression {
    Some(opt_rep_modifiers)
}
pub fn optional_repeat_modifiers_expression_p1() -> OptionalRepeatModifiersExpression {
    None
}

pub type OptionalRepeatModifiers = Vec<OptionalRepeatModifier>;
pub fn optional_repeat_modifiers_p0(
    mut modifiers: OptionalRepeatModifiers,
    modifier: OptionalRepeatModifier,
) -> OptionalRepeatModifiers {
    modifiers.push(modifier);
    modifiers
}
pub fn optional_repeat_modifiers_p1(modifier: OptionalRepeatModifier) -> OptionalRepeatModifiers {
    vec![modifier]
}

#[derive(Debug)]
pub struct OptionalRepeatModifier(pub Name);
pub fn optional_repeat_modifier_p0(name: Name) -> OptionalRepeatModifier {
    OptionalRepeatModifier(name)
}

#[derive(Debug)]
pub enum GrammarSymbol {
    Name(Name),
    StrConst(StrConst),
}
pub fn grammar_symbol_p0(name: Name) -> GrammarSymbol {
    GrammarSymbol::Name(name)
}
pub fn grammar_symbol_p1(str_const: StrConst) -> GrammarSymbol {
    GrammarSymbol::StrConst(str_const)
}

#[derive(Debug)]
pub enum Recognizer {
    StrConst(StrConst),
    RegExTerm(RegExTerm),
}
pub fn recognizer_p0(str_const: StrConst) -> Recognizer {
    Recognizer::StrConst(str_const)
}
pub fn recognizer_p1(regex: RegExTerm) -> Recognizer {
    Recognizer::RegExTerm(regex)
}

pub type LAYOUT = String;
pub fn layoutp0(item: LAYOUTITEM) -> LAYOUT {
    item
}
pub fn layoutp1(mut layout: LAYOUT, item: LAYOUTITEM) -> LAYOUT {
    layout.push_str(&item);
    layout
}
pub fn layoutp2() -> LAYOUT {
    "".into()
}

pub type LAYOUTITEM = String;
pub fn layoutitemp0(ws: WS) -> LAYOUTITEM {
    ws
}
pub fn layoutitemp1(comment: Comment) -> LAYOUTITEM {
    comment
}

pub type Comment = String;
pub fn comment_p0(s: CORNCS) -> Comment {
    s
}
pub fn comment_p1(s: CommentLine) -> Comment {
    s
}

pub type CORNCS = String;
pub fn corncsp0(s: CORNC) -> CORNCS {
    s
}
pub fn corncsp1(mut ss: CORNCS, s: CORNC) -> CORNCS {
    ss.push_str(&s);
    ss
}
pub fn corncsp2() -> CORNCS {
    "".into()
}

pub type CORNC = String;
pub fn corncp0(s: Comment) -> CORNC {
    s
}
pub fn corncp1(s: NotComment) -> CORNC {
    s
}
pub fn corncp2(s: WS) -> CORNC {
    s
}