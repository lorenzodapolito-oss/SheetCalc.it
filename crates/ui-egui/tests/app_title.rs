//! The title bar and the window title name the app and its owner.

#[test]
fn app_title_names_the_owner() {
    assert_eq!(gridcraft_ui_egui::app_title(), "SheetCalc.it di Lorenzo D'Apolito");
}
