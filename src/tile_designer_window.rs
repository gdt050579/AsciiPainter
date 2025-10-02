use appcui::prelude::*;
use crate::tile_editor::TileEditor;

#[Window()]
pub struct TileDesignerWindow {}

impl TileDesignerWindow {
    pub fn new(tile_size: Size) -> Self {
        let mut w = Self {
            base: window!("'Tile Editor', a:c, w:50, h:15, flags: Sizeable"),
        };
        w.add(TileEditor::new(tile_size));
        w
    }
}
