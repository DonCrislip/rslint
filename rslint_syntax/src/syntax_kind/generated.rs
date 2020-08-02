//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT_DEF`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    TILDE,
    QUESTION,
    AMP,
    PIPE,
    PLUS,
    PLUS2,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    DOT,
    COLON,
    EQ,
    EQ2,
    EQ3,
    FAT_ARROW,
    BANG,
    NEQ,
    NEQ2,
    MINUS,
    MINUS2,
    LTEQ,
    GTEQ,
    PLUSEQ,
    MINUSEQ,
    PIPEEQ,
    AMPEQ,
    CARETEQ,
    SLASHEQ,
    STAREQ,
    PERCENTEQ,
    AMP2,
    PIPE2,
    SHL,
    SHR,
    USHR,
    SHLEQ,
    SHREQ,
    USHREQ,
    BREAK_KW,
    CASE_KW,
    CATCH_KW,
    CLASS_KW,
    CONST_KW,
    CONTINUE_KW,
    DEBUGGER_KW,
    DEFAULT_KW,
    DELETE_KW,
    DO_KW,
    ELSE_KW,
    ENUM_KW,
    EXPORT_KW,
    EXTENDS_KW,
    FINALLY_KW,
    FOR_KW,
    FUNCTION_KW,
    IF_KW,
    IN_KW,
    INSTANCEOF_KW,
    INTERFACE_KW,
    IMPORT_KW,
    IMPLEMENTS_KW,
    LET_KW,
    NEW_KW,
    PACKAGE_KW,
    PRIVATE_KW,
    PROTECTED_KW,
    PUBLIC_KW,
    RETURN_KW,
    STATIC_KW,
    SUPER_KW,
    SWITCH_KW,
    THIS_KW,
    THROW_KW,
    TRY_KW,
    TYPEOF_KW,
    VAR_KW,
    VOID_KW,
    WHILE_KW,
    WITH_KW,
    YIELD_KW,
    NULL,
    BOOL,
    NUMBER,
    STRING,
    REGEX,
    ERROR,
    IDENT,
    WHITESPACE,
    COMMENT,
    SHEBANG,
    PROGRAM,
    BLOCK_STMT,
    VAR_STMT,
    DECLARATOR,
    EMPTY_STMT,
    EXPR_STMT,
    IF_STMT,
    DO_WHILE_STMT,
    WHILE_STMT,
    FOR_STMT,
    FOR_IN_STMT,
    CONTINUE_STMT,
    BREAK_STMT,
    RETURN_STMT,
    WITH_STMT,
    SWITCH_STMT,
    CASE_CLAUSE,
    LABELLED_STMT,
    THROW_STMT,
    TRY_STMT,
    CATCH_CLAUSE,
    DEBUGGER_STMT,
    FN_DECL,
    NAME,
    FN_BODY,
    PARAMETER_LIST,
    THIS_EXPR,
    ARRAY_EXPR,
    OBJECT_EXPR,
    LITERAL_PROP,
    GETTER_PROP,
    SETTER_PROP,
    GROUPING_EXPR,
    NEW_EXPR,
    FN_EXPR,
    BRACKET_EXPR,
    DOT_EXPR,
    CALL_EXPR,
    POSTFIX_EXPR,
    UNARY_EXPR,
    BIN_EXPR,
    COND_EXPR,
    ASSIGN_EXPR,
    SEQUENCE_EXPR,
    ARG_LIST,
    LITERAL,
    CONDITION,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        match self {
            BREAK_KW | CASE_KW | CATCH_KW | CLASS_KW | CONST_KW | CONTINUE_KW | DEBUGGER_KW
            | DEFAULT_KW | DELETE_KW | DO_KW | ELSE_KW | ENUM_KW | EXPORT_KW | EXTENDS_KW
            | FINALLY_KW | FOR_KW | FUNCTION_KW | IF_KW | IN_KW | INSTANCEOF_KW | INTERFACE_KW
            | IMPORT_KW | IMPLEMENTS_KW | LET_KW | NEW_KW | PACKAGE_KW | PRIVATE_KW
            | PROTECTED_KW | PUBLIC_KW | RETURN_KW | STATIC_KW | SUPER_KW | SWITCH_KW | THIS_KW
            | THROW_KW | TRY_KW | TYPEOF_KW | VAR_KW | VOID_KW | WHILE_KW | WITH_KW | YIELD_KW => {
                true
            }
            _ => false,
        }
    }
    pub fn is_punct(self) -> bool {
        match self {
            SEMICOLON | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK
            | L_ANGLE | R_ANGLE | TILDE | QUESTION | AMP | PIPE | PLUS | PLUS2 | STAR | SLASH
            | CARET | PERCENT | DOT | COLON | EQ | EQ2 | EQ3 | FAT_ARROW | BANG | NEQ | NEQ2
            | MINUS | MINUS2 | LTEQ | GTEQ | PLUSEQ | MINUSEQ | PIPEEQ | AMPEQ | CARETEQ
            | SLASHEQ | STAREQ | PERCENTEQ | AMP2 | PIPE2 | SHL | SHR | USHR | SHLEQ | SHREQ
            | USHREQ => true,
            _ => false,
        }
    }
    pub fn is_literal(self) -> bool {
        match self {
            NULL | BOOL | NUMBER | STRING | REGEX => true,
            _ => false,
        }
    }
    pub fn is_before_expr(self) -> bool {
        match self {
            BANG | L_PAREN | L_BRACK | L_CURLY | SEMICOLON | COMMA | COLON | QUESTION | PLUS2
            | MINUS2 | TILDE | CASE_KW | DEFAULT_KW | DO_KW | ELSE_KW | RETURN_KW | THROW_KW
            | NEW_KW | EXTENDS_KW | YIELD_KW | IN_KW | TYPEOF_KW | VOID_KW | DELETE_KW | PLUSEQ
            | MINUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AMP2 | PIPE2
            | SHLEQ | SHREQ | USHREQ | EQ | FAT_ARROW => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "break" => BREAK_KW,
            "case" => CASE_KW,
            "catch" => CATCH_KW,
            "class" => CLASS_KW,
            "const" => CONST_KW,
            "continue" => CONTINUE_KW,
            "debugger" => DEBUGGER_KW,
            "default" => DEFAULT_KW,
            "delete" => DELETE_KW,
            "do" => DO_KW,
            "else" => ELSE_KW,
            "enum" => ENUM_KW,
            "export" => EXPORT_KW,
            "extends" => EXTENDS_KW,
            "finally" => FINALLY_KW,
            "for" => FOR_KW,
            "function" => FUNCTION_KW,
            "if" => IF_KW,
            "in" => IN_KW,
            "instanceof" => INSTANCEOF_KW,
            "interface" => INTERFACE_KW,
            "import" => IMPORT_KW,
            "implements" => IMPLEMENTS_KW,
            "let" => LET_KW,
            "new" => NEW_KW,
            "package" => PACKAGE_KW,
            "private" => PRIVATE_KW,
            "protected" => PROTECTED_KW,
            "public" => PUBLIC_KW,
            "return" => RETURN_KW,
            "static" => STATIC_KW,
            "super" => SUPER_KW,
            "switch" => SWITCH_KW,
            "this" => THIS_KW,
            "throw" => THROW_KW,
            "try" => TRY_KW,
            "typeof" => TYPEOF_KW,
            "var" => VAR_KW,
            "void" => VOID_KW,
            "while" => WHILE_KW,
            "with" => WITH_KW,
            "yield" => YIELD_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            ';' => SEMICOLON,
            ',' => COMMA,
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_CURLY,
            '}' => R_CURLY,
            '[' => L_BRACK,
            ']' => R_BRACK,
            '<' => L_ANGLE,
            '>' => R_ANGLE,
            '~' => TILDE,
            '?' => QUESTION,
            '&' => AMP,
            '|' => PIPE,
            '+' => PLUS,
            '*' => STAR,
            '/' => SLASH,
            '^' => CARET,
            '%' => PERCENT,
            '.' => DOT,
            ':' => COLON,
            '=' => EQ,
            '!' => BANG,
            '-' => MINUS,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules ! T { [ ; ] => { $ crate :: SyntaxKind :: SEMICOLON } ; [ , ] => { $ crate :: SyntaxKind :: COMMA } ; [ '(' ] => { $ crate :: SyntaxKind :: L_PAREN } ; [ ')' ] => { $ crate :: SyntaxKind :: R_PAREN } ; [ '{' ] => { $ crate :: SyntaxKind :: L_CURLY } ; [ '}' ] => { $ crate :: SyntaxKind :: R_CURLY } ; [ '[' ] => { $ crate :: SyntaxKind :: L_BRACK } ; [ ']' ] => { $ crate :: SyntaxKind :: R_BRACK } ; [ < ] => { $ crate :: SyntaxKind :: L_ANGLE } ; [ > ] => { $ crate :: SyntaxKind :: R_ANGLE } ; [ ~ ] => { $ crate :: SyntaxKind :: TILDE } ; [ ? ] => { $ crate :: SyntaxKind :: QUESTION } ; [ & ] => { $ crate :: SyntaxKind :: AMP } ; [ | ] => { $ crate :: SyntaxKind :: PIPE } ; [ + ] => { $ crate :: SyntaxKind :: PLUS } ; [ ++ ] => { $ crate :: SyntaxKind :: PLUS2 } ; [ * ] => { $ crate :: SyntaxKind :: STAR } ; [ / ] => { $ crate :: SyntaxKind :: SLASH } ; [ ^ ] => { $ crate :: SyntaxKind :: CARET } ; [ % ] => { $ crate :: SyntaxKind :: PERCENT } ; [ . ] => { $ crate :: SyntaxKind :: DOT } ; [ : ] => { $ crate :: SyntaxKind :: COLON } ; [ = ] => { $ crate :: SyntaxKind :: EQ } ; [ == ] => { $ crate :: SyntaxKind :: EQ2 } ; [ === ] => { $ crate :: SyntaxKind :: EQ3 } ; [ => ] => { $ crate :: SyntaxKind :: FAT_ARROW } ; [ ! ] => { $ crate :: SyntaxKind :: BANG } ; [ != ] => { $ crate :: SyntaxKind :: NEQ } ; [ !== ] => { $ crate :: SyntaxKind :: NEQ2 } ; [ - ] => { $ crate :: SyntaxKind :: MINUS } ; [ -- ] => { $ crate :: SyntaxKind :: MINUS2 } ; [ <= ] => { $ crate :: SyntaxKind :: LTEQ } ; [ >= ] => { $ crate :: SyntaxKind :: GTEQ } ; [ += ] => { $ crate :: SyntaxKind :: PLUSEQ } ; [ -= ] => { $ crate :: SyntaxKind :: MINUSEQ } ; [ |= ] => { $ crate :: SyntaxKind :: PIPEEQ } ; [ &= ] => { $ crate :: SyntaxKind :: AMPEQ } ; [ ^= ] => { $ crate :: SyntaxKind :: CARETEQ } ; [ /= ] => { $ crate :: SyntaxKind :: SLASHEQ } ; [ *= ] => { $ crate :: SyntaxKind :: STAREQ } ; [ %= ] => { $ crate :: SyntaxKind :: PERCENTEQ } ; [ && ] => { $ crate :: SyntaxKind :: AMP2 } ; [ || ] => { $ crate :: SyntaxKind :: PIPE2 } ; [ << ] => { $ crate :: SyntaxKind :: SHL } ; [ >> ] => { $ crate :: SyntaxKind :: SHR } ; [ >>> ] => { $ crate :: SyntaxKind :: USHR } ; [ <<= ] => { $ crate :: SyntaxKind :: SHLEQ } ; [ >>= ] => { $ crate :: SyntaxKind :: SHREQ } ; [ >>>= ] => { $ crate :: SyntaxKind :: USHREQ } ; [ break ] => { $ crate :: SyntaxKind :: BREAK_KW } ; [ case ] => { $ crate :: SyntaxKind :: CASE_KW } ; [ catch ] => { $ crate :: SyntaxKind :: CATCH_KW } ; [ class ] => { $ crate :: SyntaxKind :: CLASS_KW } ; [ const ] => { $ crate :: SyntaxKind :: CONST_KW } ; [ continue ] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; [ debugger ] => { $ crate :: SyntaxKind :: DEBUGGER_KW } ; [ default ] => { $ crate :: SyntaxKind :: DEFAULT_KW } ; [ delete ] => { $ crate :: SyntaxKind :: DELETE_KW } ; [ do ] => { $ crate :: SyntaxKind :: DO_KW } ; [ else ] => { $ crate :: SyntaxKind :: ELSE_KW } ; [ enum ] => { $ crate :: SyntaxKind :: ENUM_KW } ; [ export ] => { $ crate :: SyntaxKind :: EXPORT_KW } ; [ extends ] => { $ crate :: SyntaxKind :: EXTENDS_KW } ; [ finally ] => { $ crate :: SyntaxKind :: FINALLY_KW } ; [ for ] => { $ crate :: SyntaxKind :: FOR_KW } ; [ function ] => { $ crate :: SyntaxKind :: FUNCTION_KW } ; [ if ] => { $ crate :: SyntaxKind :: IF_KW } ; [ in ] => { $ crate :: SyntaxKind :: IN_KW } ; [ instanceof ] => { $ crate :: SyntaxKind :: INSTANCEOF_KW } ; [ interface ] => { $ crate :: SyntaxKind :: INTERFACE_KW } ; [ import ] => { $ crate :: SyntaxKind :: IMPORT_KW } ; [ implements ] => { $ crate :: SyntaxKind :: IMPLEMENTS_KW } ; [ let ] => { $ crate :: SyntaxKind :: LET_KW } ; [ new ] => { $ crate :: SyntaxKind :: NEW_KW } ; [ package ] => { $ crate :: SyntaxKind :: PACKAGE_KW } ; [ private ] => { $ crate :: SyntaxKind :: PRIVATE_KW } ; [ protected ] => { $ crate :: SyntaxKind :: PROTECTED_KW } ; [ public ] => { $ crate :: SyntaxKind :: PUBLIC_KW } ; [ return ] => { $ crate :: SyntaxKind :: RETURN_KW } ; [ static ] => { $ crate :: SyntaxKind :: STATIC_KW } ; [ super ] => { $ crate :: SyntaxKind :: SUPER_KW } ; [ switch ] => { $ crate :: SyntaxKind :: SWITCH_KW } ; [ this ] => { $ crate :: SyntaxKind :: THIS_KW } ; [ throw ] => { $ crate :: SyntaxKind :: THROW_KW } ; [ try ] => { $ crate :: SyntaxKind :: TRY_KW } ; [ typeof ] => { $ crate :: SyntaxKind :: TYPEOF_KW } ; [ var ] => { $ crate :: SyntaxKind :: VAR_KW } ; [ void ] => { $ crate :: SyntaxKind :: VOID_KW } ; [ while ] => { $ crate :: SyntaxKind :: WHILE_KW } ; [ with ] => { $ crate :: SyntaxKind :: WITH_KW } ; [ yield ] => { $ crate :: SyntaxKind :: YIELD_KW } ; [ ident ] => { $ crate :: SyntaxKind :: IDENT } ; }
