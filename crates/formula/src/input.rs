//! Localized aliases accepted at user-entry boundaries; file syntax stays English.

use crate::{Expr, ParseError, parser};

/// Formula-entry syntax selected by the interface language. Other interface languages
/// currently use the canonical English function table until a translation table is supplied.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormulaLocale {
    #[default]
    En,
    Es,
    It,
}

const SPANISH: &[(&str, &str)] = &[
    ("SUM", "SUMA"),
    ("AVERAGE", "PROMEDIO"),
    ("IF", "SI"),
    ("COUNT", "CONTAR"),
    ("SUMIF", "SUMAR.SI"),
    ("COUNTIF", "CONTAR.SI"),
    ("VLOOKUP", "BUSCARV"),
    ("TODAY", "HOY"),
    ("TRUE", "VERDADERO"),
    ("FALSE", "FALSO"),
];

/// Italian function names as Excel ships them (canonical English name, Italian name).
const ITALIAN: &[(&str, &str)] = &[
    ("SUM", "SOMMA"),
    ("SUMIF", "SOMMA.SE"),
    ("SUMIFS", "SOMMA.PIÙ.SE"),
    ("SUMPRODUCT", "MATR.SOMMA.PRODOTTO"),
    ("AVERAGE", "MEDIA"),
    ("AVERAGEIF", "MEDIA.SE"),
    ("AVERAGEIFS", "MEDIA.PIÙ.SE"),
    ("COUNT", "CONTA.NUMERI"),
    ("COUNTA", "CONTA.VALORI"),
    ("COUNTBLANK", "CONTA.VUOTE"),
    ("COUNTIF", "CONTA.SE"),
    ("COUNTIFS", "CONTA.PIÙ.SE"),
    ("MEDIAN", "MEDIANA"),
    ("MODE", "MODA"),
    ("LARGE", "GRANDE"),
    ("SMALL", "PICCOLO"),
    ("RANK", "RANGO"),
    ("STDEV.S", "DEV.ST.C"),
    ("STDEV.P", "DEV.ST.P"),
    ("VAR.S", "VAR.C"),
    ("VAR.P", "VAR.P"),
    ("IF", "SE"),
    ("IFS", "PIÙ.SE"),
    ("IFERROR", "SE.ERRORE"),
    ("IFNA", "SE.NON.DISP."),
    ("AND", "E"),
    ("OR", "O"),
    ("NOT", "NON"),
    ("TRUE", "VERO"),
    ("FALSE", "FALSO"),
    ("VLOOKUP", "CERCA.VERT"),
    ("HLOOKUP", "CERCA.ORIZZ"),
    ("LOOKUP", "CERCA"),
    ("XLOOKUP", "CERCA.X"),
    ("MATCH", "CONFRONTA"),
    ("INDEX", "INDICE"),
    ("CHOOSE", "SCEGLI"),
    ("OFFSET", "SCARTO"),
    ("INDIRECT", "INDIRETTO"),
    ("ROW", "RIF.RIGA"),
    ("COLUMN", "RIF.COLONNA"),
    ("ROWS", "RIGHE"),
    ("COLUMNS", "COLONNE"),
    ("TRANSPOSE", "MATR.TRASPOSTA"),
    ("UNIQUE", "UNICI"),
    ("SORT", "ORDINA"),
    ("FILTER", "FILTRO"),
    ("SEQUENCE", "SEQUENZA"),
    ("TODAY", "OGGI"),
    ("NOW", "ADESSO"),
    ("DATE", "DATA"),
    ("YEAR", "ANNO"),
    ("MONTH", "MESE"),
    ("DAY", "GIORNO"),
    ("HOUR", "ORA"),
    ("MINUTE", "MINUTO"),
    ("SECOND", "SECONDO"),
    ("WEEKDAY", "GIORNO.SETTIMANA"),
    ("EDATE", "DATA.MESE"),
    ("EOMONTH", "FINE.MESE"),
    ("DATEDIF", "DATA.DIFF"),
    ("NETWORKDAYS", "GIORNI.LAVORATIVI.TOT"),
    ("WORKDAY", "GIORNO.LAVORATIVO"),
    ("ABS", "ASS"),
    ("ROUND", "ARROTONDA"),
    ("ROUNDUP", "ARROTONDA.PER.ECC"),
    ("ROUNDDOWN", "ARROTONDA.PER.DIF"),
    ("CEILING", "ARROTONDA.ECCESSO"),
    ("FLOOR", "ARROTONDA.DIFETTO"),
    ("TRUNC", "TRONCA"),
    ("SQRT", "RADQ"),
    ("POWER", "POTENZA"),
    ("MOD", "RESTO"),
    ("SIGN", "SEGNO"),
    ("FACT", "FATTORIALE"),
    ("COMBIN", "COMBINAZIONE"),
    ("PI", "PI.GRECO"),
    ("RAND", "CASUALE"),
    ("RANDBETWEEN", "CASUALE.TRA"),
    ("LEN", "LUNGHEZZA"),
    ("LEFT", "SINISTRA"),
    ("RIGHT", "DESTRA"),
    ("MID", "STRINGA.ESTRAI"),
    ("UPPER", "MAIUSC"),
    ("LOWER", "MINUSC"),
    ("PROPER", "MAIUSC.INIZ"),
    ("TRIM", "ANNULLA.SPAZI"),
    ("CONCATENATE", "CONCATENA"),
    ("TEXTJOIN", "TESTO.UNISCI"),
    ("TEXT", "TESTO"),
    ("VALUE", "VALORE"),
    ("FIND", "TROVA"),
    ("SEARCH", "RICERCA"),
    ("SUBSTITUTE", "SOSTITUISCI"),
    ("REPLACE", "RIMPIAZZA"),
    ("REPT", "RIPETI"),
    ("EXACT", "IDENTICO"),
    ("CHAR", "CODICE.CARATT"),
    ("CODE", "CODICE"),
    ("ISBLANK", "VAL.VUOTO"),
    ("ISNUMBER", "VAL.NUMERO"),
    ("ISTEXT", "VAL.TESTO"),
    ("ISERROR", "VAL.ERRORE"),
    ("ISERR", "VAL.ERR"),
    ("ISNA", "VAL.NON.DISP"),
    ("ISODD", "VAL.DISPARI"),
    ("ISEVEN", "VAL.PARI"),
    ("PMT", "RATA"),
    ("PV", "VA"),
    ("FV", "VAL.FUT"),
    ("NPV", "VAN"),
    ("IRR", "TIR.COST"),
];

impl FormulaLocale {
    fn table(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::En => &[],
            Self::Es => SPANISH,
            Self::It => ITALIAN,
        }
    }
    pub fn list_separator(self) -> char {
        if self == Self::En { ',' } else { ';' }
    }
    pub fn decimal_separator(self) -> char {
        if self == Self::En { '.' } else { ',' }
    }
    pub fn array_column_separator(self) -> char {
        if self == Self::En { ',' } else { '\\' }
    }
    pub fn function_name(self, canonical: &str) -> &str {
        self.table().iter().find(|(en, _)| en.eq_ignore_ascii_case(canonical)).map(|(_, local)| *local).unwrap_or(canonical)
    }
    pub fn canonical_name(self, localized: &str) -> &str {
        let lower = localized.to_lowercase();
        self.table().iter().find(|(_, local)| local.to_lowercase() == lower).map(|(en, _)| *en).unwrap_or(localized)
    }
    fn true_name(self) -> &'static str {
        if self == Self::It { "VERO" } else { "VERDADERO" }
    }
    fn false_name(self) -> &'static str {
        "FALSO"
    }
    /// The logical constant a bare word spells in this locale's entry syntax.
    pub(crate) fn boolean_alias(self, name: &str) -> Option<bool> {
        if self == Self::En {
            None
        } else if name.eq_ignore_ascii_case(self.true_name()) {
            Some(true)
        } else if name.eq_ignore_ascii_case(self.false_name()) {
            Some(false)
        } else {
            None
        }
    }
}

/// Parses UI input using the selected function table and separators. Existing names and
/// LET/LAMBDA bindings take precedence over aliases. File and API syntax uses [`crate::parse`].
pub fn parse_input(src: &str, locale: FormulaLocale, known_name: impl Fn(&str) -> bool) -> Result<Expr, ParseError> {
    let mut expr = parser::parse_syntax(src, locale)?;
    if locale != FormulaLocale::En {
        normalize(&mut expr, locale, &known_name, &mut Vec::new(), 0, false)?;
    }
    Ok(expr)
}

/// Prints canonical expressions for editing. Name collisions retain the canonical spelling
/// so displaying then committing a formula never changes a defined-name reference.
pub fn print_input(expr: &Expr, locale: FormulaLocale, known_name: impl Fn(&str) -> bool) -> Result<String, ParseError> {
    // Check before cloning/printing: a flat operator chain can produce a deep AST even
    // though the parser itself did not recurse deeply.
    let mut pending = vec![(expr, 0usize)];
    while let Some((node, depth)) = pending.pop() {
        if depth > parser::MAX_DEPTH {
            return Err(ParseError { msg: "formula is nested too deeply".into(), pos: 0 });
        }
        match node {
            Expr::Unary(_, child) | Expr::Paren(child) => pending.push((child, depth + 1)),
            Expr::Binary(_, left, right) => {
                pending.push((left, depth + 1));
                pending.push((right, depth + 1));
            }
            Expr::Call(_, args) => pending.extend(args.iter().map(|e| (e, depth + 1))),
            Expr::Invoke(callee, args) => {
                pending.push((callee, depth + 1));
                pending.extend(args.iter().map(|e| (e, depth + 1)));
            }
            Expr::Array(rows) => pending.extend(rows.iter().flatten().map(|e| (e, depth + 1))),
            _ => {}
        }
    }
    if locale == FormulaLocale::En {
        return Ok(crate::print(expr));
    }
    let mut expr = expr.clone();
    normalize(&mut expr, locale, &known_name, &mut Vec::new(), 0, true)?;
    let canonical = crate::print(&expr);
    let tokens = crate::lexer::tokenize(&canonical).map_err(|e| ParseError { msg: e.msg, pos: e.pos })?;
    let mut out = String::new();
    let mut array_depth = 0usize;
    for token in tokens {
        use crate::lexer::Tok;
        let text = canonical.get(token.start..token.end).unwrap_or("");
        match token.tok {
            Tok::LBrace => {
                array_depth += 1;
                out.push_str(text);
            }
            Tok::RBrace => {
                array_depth = array_depth.saturating_sub(1);
                out.push_str(text);
            }
            Tok::Number(_) => out.push_str(&text.replace('.', &locale.decimal_separator().to_string())),
            Tok::Comma => out.push(if array_depth > 0 { locale.array_column_separator() } else { locale.list_separator() }),
            _ => out.push_str(text),
        }
    }
    Ok(out)
}

fn normalize(
    expr: &mut Expr,
    locale: FormulaLocale,
    known_name: &impl Fn(&str) -> bool,
    bindings: &mut Vec<String>,
    depth: usize,
    display: bool,
) -> Result<(), ParseError> {
    if depth > parser::MAX_DEPTH {
        return Err(ParseError { msg: "formula is nested too deeply".into(), pos: 0 });
    }
    let bound = |name: &str, bindings: &[String]| bindings.iter().rev().any(|n| n.eq_ignore_ascii_case(name)) || known_name(name);
    match expr {
        Expr::Bool(value) if display => {
            let alias = if *value { locale.true_name() } else { locale.false_name() };
            if !bound(alias, bindings) {
                *expr = Expr::Name(alias.into());
            }
        }
        Expr::Name(name) if !display && !bound(name, bindings) => {
            if let Some(value) = locale.boolean_alias(name) {
                *expr = Expr::Bool(value);
            }
        }
        Expr::Call(name, args) if name == "LET" && args.len() >= 3 && !args.len().is_multiple_of(2) => {
            let saved = bindings.len();
            if let Some((body, pairs)) = args.split_last_mut() {
                for pair in pairs.chunks_mut(2) {
                    if let [declaration, value] = pair {
                        // The new name is not in scope in its own value expression.
                        normalize(value, locale, known_name, bindings, depth + 1, display)?;
                        if let Expr::Name(name) = declaration {
                            bindings.push(name.clone());
                        }
                    }
                }
                normalize(body, locale, known_name, bindings, depth + 1, display)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) if name == "LAMBDA" => {
            let saved = bindings.len();
            if let Some((body, params)) = args.split_last_mut() {
                for param in params {
                    if let Expr::Name(name) = param {
                        bindings.push(name.clone());
                    }
                }
                normalize(body, locale, known_name, bindings, depth + 1, display)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) => {
            let alias = if display { locale.function_name(name) } else { locale.canonical_name(name) };
            if !bound(name, bindings) && (!display || !bound(alias, bindings)) {
                *name = alias.to_string();
            }
            for arg in args {
                normalize(arg, locale, known_name, bindings, depth + 1, display)?;
            }
        }
        Expr::Invoke(callee, args) => {
            normalize(callee, locale, known_name, bindings, depth + 1, display)?;
            for arg in args {
                normalize(arg, locale, known_name, bindings, depth + 1, display)?;
            }
        }
        Expr::Unary(_, child) | Expr::Paren(child) => normalize(child, locale, known_name, bindings, depth + 1, display)?,
        Expr::Binary(_, left, right) => {
            normalize(left, locale, known_name, bindings, depth + 1, display)?;
            normalize(right, locale, known_name, bindings, depth + 1, display)?;
        }
        Expr::Array(rows) if display => {
            for row in rows {
                for child in row {
                    normalize(child, locale, known_name, bindings, depth + 1, display)?;
                }
            }
        }
        // Array elements have already been parsed as literals. Sheet/structured references
        // contain user-defined labels and must not be translated.
        _ => {}
    }
    Ok(())
}
