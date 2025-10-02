use appcui::prelude::*;
mod painter_window;
use painter_window::PainterWindow;
mod painter_control;
mod tile_designer_window;
use tile_designer_window::TileDesignerWindow;
mod selection;
use selection::Selection;
mod drawing_object;
use drawing_object::DrawingObject;
mod tile_editor;
pub use tile_editor::TileEditor;
use appcui::dialogs::{OpenFileDialogFlags, SaveFileDialogFlags};

#[Desktop(events = [MenuEvents, DesktopEvents, AppBarEvents],  
          overwrite = OnPaint,
          commands = [New, TileEditor, Exit, Open, Save])]
struct PainterDesktop {
    index: u32,
    menu_file: Handle<appbar::MenuButton>,
    menu_design: Handle<appbar::MenuButton>,
}

impl PainterDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            index: 1,
            menu_file: Handle::None,
            menu_design: Handle::None,
        }
    }
}


fn string_validation(s: &str) -> Result<(), String> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return Err("Invalid format. Use 'width x height'.".to_string());
    }
    if let (Ok(w), Ok(h)) = (parts[0].trim().parse::<u32>(), parts[1].trim().parse::<u32>()) {
        if w > 0 && h > 0 && w <= 255 && h <= 255 {
            return Ok(());
        } else {
            return Err("Width and height must be between 1 and 255.".to_string());
        }
    }
    return Err("Invalid format. Use 'width x height'.".to_string());
}

impl OnPaint for PainterDesktop {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        surface.clear(theme.desktop.character);
    }
}

impl DesktopEvents for PainterDesktop {
    fn on_start(&mut self) { 
        self.menu_file = self.appbar().add(appbar::MenuButton::new("&File", menu!("
            class: PainterDesktop, items:[
                {'&New',cmd: New},
                {'&Open',cmd: Open},
                {'&Save',cmd: Save},
                {-},
                {'E&xit',cmd: Exit}
            ]
        "),0, appbar::Side::Left));
        self.menu_design = self.appbar().add(appbar::MenuButton::new("&Design", menu!("
            class: PainterDesktop, items:[
                {'&Tile Editor',cmd: TileEditor},
            ]
        "),0, appbar::Side::Left));        
    }
}

impl MenuEvents for PainterDesktop {
    fn on_command(&mut self, _menu: Handle<Menu>, _item: Handle<menu::Command>, command: painterdesktop::Commands) {
        match command {
            painterdesktop::Commands::New => {
                let name = format!("Painting─{}", self.index);
                self.index += 1;
                self.add_window(PainterWindow::new(&name));
            }
            painterdesktop::Commands::Exit => self.close(),
            painterdesktop::Commands::Open => {
                if let Some(file) = dialogs::open("Open surfaces", "", dialogs::Location::Last, Some("Surface image = [srf]"), OpenFileDialogFlags::Icons) {
                    if let Ok(painter) = PainterWindow::from_file(&file) {
                        self.add_window(painter);
                    } else {
                        dialogs::error("Error", "Failed to open the painting file.");
                    }
                }
            }
            painterdesktop::Commands::Save => {
                if let Some(path) = dialogs::save("Save surface", "", dialogs::Location::Last, Some("Surface image = [srf]"),SaveFileDialogFlags::Icons|SaveFileDialogFlags::ValidateOverwrite) {
                    if let Some(window) = self.active_window_handle() {
                        let h: Handle<PainterWindow> = unsafe { window.unsafe_cast() };
                        if let Some(painter) = self.window_mut(h) {
                            if let Err(err) = painter.save_to_file(&path) {
                                dialogs::error("Error", &format!("Failed to save the painting file: {}", err));
                            }
                        }   
                    }
                }
            }
            painterdesktop::Commands::TileEditor => {
                //if let Some(size) = dialogs::input::<Size>("Tile Size", "Enter the size of the tile (width x height)", None, Some(string_validation)) {
                self.add_window(TileDesignerWindow::new(Size::new(14,6)));
                //}
                
            }
        }
    }
}
impl AppBarEvents for PainterDesktop {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.menu_file);
        appbar.show(self.menu_design);
    }
}

fn main() -> Result<(), appcui::system::Error> {
    #[cfg(target_os = "windows")]
    App::with_backend(appcui::backend::Type::WindowsConsole)
        .desktop(PainterDesktop::new())
        .app_bar()
        .build()?
        .run();

    #[cfg(not(target_os = "windows"))]
    App::new()
        .desktop(PainterDesktop::new())
        .menu_bar()
        //.log_file("log.txt", false)
        .build()?
        .run();
    Ok(())
} 