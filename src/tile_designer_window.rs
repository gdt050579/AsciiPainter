use crate::tile_editor::TileEditor;
use crate::tile_editor::tileeditor;
use appcui::prelude::*;

#[Window(custom_events = TileEditorEvents)]
pub struct TileDesignerWindow {
    h_hash: Handle<TextField>,
    h_preview: Handle<Canvas>,
}

impl TileDesignerWindow {
    pub fn new(tile_size: Size) -> Self {
        let mut w = Self {
            base: window!("'Tile Editor', a:c, w:50, h:15, flags: Sizeable"),
            h_hash: Handle::None,
            h_preview: Handle::None,
        };
        let mut vs = vsplitter!("pos: 75%,d:f, resize: PreserveRightPanelSize");
        vs.add(vsplitter::Panel::Left, TileEditor::new(tile_size));

        vs.add(vsplitter::Panel::Right, label!("Hash,x:1,y:1,w:10"));
        w.h_hash = vs.add(
            vsplitter::Panel::Right,
            textfield!("l:1,t:2,r:1,h:3,flags: ReadOnly"),
        );

        let mut preview_panel = panel!("Preview,l:1,r:1,t:8,b:1");
        w.h_preview = preview_panel.add(canvas!("50x50,d:f"));
        vs.add(vsplitter::Panel::Right, preview_panel);

        w.add(vs);
        w
    }
}

impl TileEditorEvents for TileDesignerWindow {
    fn on_event(
        &mut self,
        handle: Handle<TileEditor>,
        _: tileeditor::Events,
    ) -> EventProcessStatus {
        if let Some(tile) = self.control(handle).map(|c| c.tile()) {
            //let hash_str = format!("0x{:X}", tile.to_u128());
            let hash_str = "?".to_string();
            let h = self.h_hash;
            if let Some(txh) = self.control_mut(h) {
                txh.set_text(&hash_str);
            }
            let h = self.h_preview;
            if let Some(cnv) = self.control_mut(h) {
                let surface = cnv.drawing_surface_mut();
                surface.clear(char!("' ',white,black"));
                surface.draw_tile(0,0,&tile, Color::White, Color::Black, BitTileRenderMethod::Braille);
            }
        }
        EventProcessStatus::Processed
    }
}
