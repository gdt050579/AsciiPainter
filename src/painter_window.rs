use std::path::Path;

use crate::drawing_object::DrawingObject;
use crate::drawing_object::FillRectangleObject;
use crate::drawing_object::LineObject;
use crate::drawing_object::MoveObject;
use crate::drawing_object::RectangleObject;
use crate::drawing_object::SelectionObject;
use crate::drawing_object::TextObject;
use appcui::graphics::LineType;
use appcui::prelude::*;

use super::painter_control::PainterControl;

#[Window(events = MenuEvents + ColorPickerEvents + SelectorEvents<LineType> + ButtonEvents + AccordionEvents + TextFieldEvents + RadioBoxEvents,
        commands = ForegroundColor + BackgroundColor + Char25 + Char50 + Char75 + Char100)]
pub struct PainterWindow {
    painter: Handle<PainterControl>,
    tmp_string: String,
    acc: Handle<Accordion>,
    menu: Handle<Menu>,
    // rectangle
    rectangle_fore: Handle<ColorPicker>,
    rectangle_back: Handle<ColorPicker>,
    rectangle_line_type: Handle<Selector<LineType>>,
    // fill rectangle
    fill_fore: Handle<ColorPicker>,
    fill_back: Handle<ColorPicker>,
    fill_char: Handle<TextField>,
    // line
    line_fore: Handle<ColorPicker>,
    line_back: Handle<ColorPicker>,
    line_type: Handle<Selector<LineType>>,
    line_vert: Handle<RadioBox>,
    // Text
    text_fore: Handle<ColorPicker>,
    text_back: Handle<ColorPicker>,
    text_content: Handle<TextArea>,
}

impl PainterWindow {
    fn inner_new(name: &str, path: Option<&Path>) -> Result<Self, String> {
        let mut w = Self {
            base: Window::new(name, Layout::new("d:c,w:60,h:20"), window::Flags::Sizeable),
            tmp_string: String::with_capacity(1024),
            painter: Handle::None,
            acc: Handle::None,
            menu: Handle::None,
            rectangle_fore: Handle::None,
            rectangle_back: Handle::None,
            rectangle_line_type: Handle::None,
            fill_fore: Handle::None,
            fill_back: Handle::None,
            fill_char: Handle::None,
            line_fore: Handle::None,
            line_back: Handle::None,
            line_type: Handle::None,
            line_vert: Handle::None,
            text_fore: Handle::None,
            text_back: Handle::None,
            text_content: Handle::None,
        };

        let mut vs = vsplitter!("pos: 75%,d:c");
        let mut acc = accordion!("d:c,w:100%,h:100%");

        // Selection panel
        let id = acc.add_panel("Selection");

        // Move panel
        let id = acc.add_panel("Move");

        // Rectangle panel
        let id = acc.add_panel("Rectangle");
        acc.add(id, label!("'Type:',x:1,y:1,w:5,h:1"));
        w.rectangle_line_type = acc.add(id, selector!("LineType,l:7,t:1,r:1,value:Single"));
        acc.add(id, label!("'Fore:',x:1,y:3,w:5,h:1"));
        w.rectangle_fore = acc.add(id, colorpicker!("White,l:7,t:3,r:1"));
        acc.add(id, label!("'Back:',x:1,y:5,w:5,h:1"));
        w.rectangle_back = acc.add(id, colorpicker!("Black,l:7,t:5,r:1"));

        // Filled rectangle panel
        let id = acc.add_panel("Filled Rectangle");
        acc.add(id, label!("'Char:',x:1,y:1,w:5,h:1"));
        w.fill_char = acc.add(id, textfield!("*,l:7,t:1,r:1,flags:ProcessEnter"));
        acc.add(id, label!("'Fore:',x:1,y:3,w:5,h:1"));
        w.fill_fore = acc.add(id, colorpicker!("White,l:7,t:3,r:1"));
        acc.add(id, label!("'Back:',x:1,y:5,w:5,h:1"));
        w.fill_back = acc.add(id, colorpicker!("Black,l:7,t:5,r:1"));

        // Line panel
        let id = acc.add_panel("Line");
        acc.add(id, label!("'Type:',x:1,y:1,w:5,h:1"));
        w.line_type = acc.add(id, selector!("LineType,l:7,t:1,r:1,value:Single"));
        acc.add(id, label!("'Fore:',x:1,y:3,w:5,h:1"));
        w.line_fore = acc.add(id, colorpicker!("White,l:7,t:3,r:1"));
        acc.add(id, label!("'Back:',x:1,y:5,w:5,h:1"));
        w.line_back = acc.add(id, colorpicker!("Black,l:7,t:5,r:1"));
        w.line_vert = acc.add(id, radiobox!("Vertical,l:1,t:7,r:1,h:1,selected:true"));
        acc.add(id, radiobox!("Horizontal,l:1,t:8,r:1,h:1,selected:false"));

        // Text panel
        let id = acc.add_panel("Text");
        acc.add(id, label!("'Fore:',x:1,y:1,w:5,h:1"));
        w.text_fore = acc.add(id, colorpicker!("White,l:7,t:1,r:1"));
        acc.add(id, label!("'Back:',x:1,y:3,w:5,h:1"));
        w.text_back = acc.add(id, colorpicker!("Black,l:7,t:3,r:1"));
        w.text_content = acc.add(
            id,
            textarea!("'Hello',l:1,t:5,r:1,b:0,flags:ShowLineNumber"),
        );
        //w.text_content = acc.add(id, textfield!("'',l:1,t:5,r:1,b:0,flags:ProcessEnter"));

        let p = if let Some(path) = path {
            if let Some(p) = PainterControl::from_path(path) {
                p
            } else {
                PainterControl::new(100, 100)
            }
        } else {
            PainterControl::new(100, 100)
        };
        w.painter = vs.add(vsplitter::Panel::Left, p);
        w.acc = vs.add(vsplitter::Panel::Right, acc);
        w.add(vs);

        // let m = menu!("
        //     &Options,class:PainterWindow,items:[
        //         {'&25% Block',1,cmd:Char25},
        //         {'&50% Block',2,cmd:Char50},
        //         {'&75% Block',3,cmd:Char75},
        //         {'&100% Block',4,cmd:Char100}
        //     ]
        // ");
        // w.menu = w.register_menu(m);
        // w.add(label!("'ForeColor:',t:0,l:0,w:10,h:1"));
        // w.add(label!("'BackColor:',t:0,l:23,w:10,h:1"));

        // w.fg_color_picker = w.add(ColorPicker::new(Color::White, Layout::new("t:0,l:10,w:8,h:1")));
        // w.bg_color_picker = w.add(ColorPicker::new(Color::Black, Layout::new("t:0,l:33,w:8,h:1")));

        // w.clear_button = w.add(button!("Clear,t:0,l:46,w:9,h:1,type: Flat"));

        // let mut p = PainterControl::new(Layout::new("t:1,l:0,r:0,b:0"));

        // if let Some(path) = path {
        //     p.load_from_file(path)?;

        // }
        // w.painter = w.add(p);
        Ok(w)
    }

    pub fn new(name: &str) -> Self {
        Self::inner_new(name, None).unwrap()
    }

    pub fn from_file(file: &Path) -> Result<Self, String> {
        Self::inner_new("Painter", Some(file))
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        let h = self.painter;
        if let Some(p) = self.control(h) {
            p.save_to_file(path)
        } else {
            Err("Painter control not found".to_string())
        }
    }

    pub fn clear_surface(&mut self) {
        let h = self.painter;
        if let Some(p) = self.control_mut(h) {
            p.clear_surface();
        }
    }

    fn update_proprties(&mut self) {
        // rect
        let rect_back = self.control(self.rectangle_back).unwrap().color();
        let rect_fore = self.control(self.rectangle_fore).unwrap().color();
        let rect_line_type = self.control(self.rectangle_line_type).unwrap().value();

        // fill
        let fill_back = self.control(self.fill_back).unwrap().color();
        let fill_fore = self.control(self.fill_fore).unwrap().color();
        let fill_char = self
            .control(self.fill_char)
            .unwrap()
            .text()
            .chars()
            .next()
            .unwrap_or(0 as char);

        // line
        let line_back = self.control(self.line_back).unwrap().color();
        let line_fore = self.control(self.line_fore).unwrap().color();
        let line_type = self.control(self.line_type).unwrap().value();
        let line_vert = self.control(self.line_vert).unwrap().is_selected();

        // text
        let text_fore = self.control(self.text_fore).unwrap().color();
        let text_back = self.control(self.text_back).unwrap().color();
        let text_content = self.control(self.text_content).unwrap().text().to_string();

        // update all properties
        let h = self.painter;
        if let Some(p) = self.control_mut(h) {
            p.update_rectangle_properties(rect_fore, rect_back, rect_line_type);
            p.update_fillrectangle_properties(fill_fore, fill_back, fill_char, CharFlags::None);
            p.update_line_properties(line_fore, line_back, line_type, line_vert);
            p.update_text_properties(text_content, text_fore, text_back, CharFlags::None);
        }
    }
}

impl MenuEvents for PainterWindow {
    fn on_update_menubar(&self, menubar: &mut MenuBar) {
        menubar.add(self.menu);
    }

    fn on_command(
        &mut self,
        _menu: Handle<Menu>,
        _item: Handle<menu::Command>,
        command: painterwindow::Commands,
    ) {
        match command {
            painterwindow::Commands::Char25 => {
                //self.set_drawing_char('░');
            }
            painterwindow::Commands::Char50 => {
                //self.set_drawing_char('▒');
            }
            painterwindow::Commands::Char75 => {
                //self.set_drawing_char('▓');
            }
            painterwindow::Commands::Char100 => {
                //self.set_drawing_char('█');
            }
            _ => {}
        }
    }
}

impl AccordionEvents for PainterWindow {
    fn on_panel_changed(
        &mut self,
        _: Handle<Accordion>,
        new_panel_index: u32,
        _: u32,
    ) -> EventProcessStatus {
        let d = match new_panel_index {
            0 => Some(DrawingObject::Selection(SelectionObject::default())),
            1 => Some(DrawingObject::Move(MoveObject::default())),
            2 => Some(DrawingObject::Rectangle(RectangleObject::default())),
            3 => Some(DrawingObject::FillRectangle(FillRectangleObject::default())),
            4 => Some(DrawingObject::Line(LineObject::default())),
            5 => Some(DrawingObject::Text(TextObject::default())),
            _ => None,
        };
        if let Some(drawing_object) = d {
            let h = self.painter;
            if let Some(p) = self.control_mut(h) {
                p.write_current_object();
                p.reset(drawing_object);
            }
            self.update_proprties();
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}

impl ColorPickerEvents for PainterWindow {
    fn on_color_changed(&mut self, _: Handle<ColorPicker>, _: Color) -> EventProcessStatus {
        self.update_proprties();
        EventProcessStatus::Processed
    }
}
impl RadioBoxEvents for PainterWindow {
    fn on_selected(&mut self, _: Handle<RadioBox>) -> EventProcessStatus {
        self.update_proprties();
        EventProcessStatus::Processed
    }
}
impl TextFieldEvents for PainterWindow {
    fn on_validate(&mut self, _: Handle<TextField>, _: &str) -> EventProcessStatus {
        self.update_proprties();
        EventProcessStatus::Processed
    }
}
impl SelectorEvents<LineType> for PainterWindow {
    fn on_selection_changed(
        &mut self,
        _: Handle<Selector<LineType>>,
        _: Option<LineType>,
    ) -> EventProcessStatus {
        self.update_proprties();
        EventProcessStatus::Processed
    }
}

impl ButtonEvents for PainterWindow {
    fn on_pressed(&mut self, _handle: Handle<Button>) -> EventProcessStatus {
        EventProcessStatus::Ignored
    }
}

