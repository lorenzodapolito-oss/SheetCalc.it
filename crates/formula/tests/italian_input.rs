use gridcraft_formula::{FormulaLocale, parse, parse_input, print, print_input};

const IT: FormulaLocale = FormulaLocale::It;

fn input(src: &str) -> String {
    print(&parse_input(src, IT, |_| false).unwrap())
}

#[test]
fn italian_aliases_and_argument_separators() {
    for (src, expected) in [
        ("=somma(A1:A10)", "SUM(A1:A10)"),
        ("MEDIA(A1:A10)", "AVERAGE(A1:A10)"),
        ("SE(A1>0;\"Sì\";\"No\")", "IF(A1>0,\"Sì\",\"No\")"),
        ("CONTA.NUMERI(A1:A10)", "COUNT(A1:A10)"),
        ("SOMMA.SE(A1:A10;\">0\")", "SUMIF(A1:A10,\">0\")"),
        ("CONTA.SE(A1:A10;\">0\")", "COUNTIF(A1:A10,\">0\")"),
        ("CERCA.VERT(A2;D:E;2;FALSO)", "VLOOKUP(A2,D:E,2,FALSE)"),
        ("somma.più.se(A:A;B:B;\"x\")", "SUMIFS(A:A,B:B,\"x\")"),
        ("OGGI()", "TODAY()"),
        ("=vero", "TRUE"),
        ("=falso", "FALSE"),
        ("VERO()", "TRUE()"),
        ("E(VERO;FALSO)", "AND(TRUE,FALSE)"),
        ("SE.ERRORE(1/0;0)", "IFERROR(1/0,0)"),
        ("SOMMA(1;SUM(2;3);MEDIA(4;6))", "SUM(1,SUM(2,3),AVERAGE(4,6))"),
        ("ARROTONDA(2,5;0)", "ROUND(2.5,0)"),
        ("{1\\2;3\\4}", "{1,2;3,4}"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
    assert!(parse_input("SOMMA(1.5;2)", IT, |_| false).is_err());
}

#[test]
fn italian_display_round_trips() {
    let expr = parse("IF(TRUE,SUM(1.5,2.25),COUNTIFS(A:A,\">0\",B:B,\"x\"))").unwrap();
    let shown = print_input(&expr, IT, |_| false).unwrap();
    assert_eq!(shown, "SE(VERO;SOMMA(1,5;2,25);CONTA.PIÙ.SE(A:A;\">0\";B:B;\"x\"))");
    assert_eq!(print(&parse_input(&shown, IT, |_| false).unwrap()), print(&expr));
}

#[test]
fn italian_keeps_defined_names() {
    let known = |name: &str| name.eq_ignore_ascii_case("somma");
    assert_eq!(print(&parse_input("SOMMA(2;3)+VERO", IT, known).unwrap()), "SOMMA(2,3)+TRUE");
}
