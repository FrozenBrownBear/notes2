use tui_editor::sidebar::{ICON_HOME_ART, ICON_SEARCH_ART, ICON_ADD_ART, ICON_SETTINGS_ART, ICON_BACK_ART};

#[test]
fn icons_have_lines() {
    assert!(!ICON_HOME_ART.is_empty());
    assert!(!ICON_SEARCH_ART.is_empty());
    assert!(!ICON_ADD_ART.is_empty());
    assert!(!ICON_SETTINGS_ART.is_empty());
    assert!(!ICON_BACK_ART.is_empty());
}
