use eframe::App;
use eframe::egui;
use egui::Ui;
use egui::Window;
use rfd;
use egui::menu::menu_button;
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "horry shit", 
        options, 
        Box::new(|_cc|  Box::new(CodeEdit::default())),
    );
}

#[derive(Default)]
pub struct CodeEdit {
    lang: String,
    code: String,
    number: i32,
    picked_path: Option<String>,
}

struct Files {
    picked_path: Option<String>,
}

impl CodeEdit {
    fn default() -> Self {
        Self {
            lang: "rs".into(),
            code: "//An example\n\
            fn main() {\n\
                \tprintln!(\"Hello world!\");\n\
                }\n\
                ".into(),
            number: 0,
            picked_path: std::option::Option::None
        }
    } 

    fn menu(&mut self, ui: &mut Ui) {
        //let file = Files;
        egui::menu::bar(ui, |ui| {
            
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_path = Some(path.display().to_string());
                    }
                }
            });
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {

                }
                if ui.button("Redo").clicked() {

                }
            })
        });    
    }

    
    fn explorer(&mut self, ui: &mut Ui) {
        
    }
}




impl eframe::App for CodeEdit {
    fn update( &mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let  Self {lang, code, number, picked_path} = self;
        egui::TopBottomPanel::top("To be Decided")
            .resizable(false)
            .min_height(20.0)
            .show(ctx, |ui| {
                self.menu(ui);
        });   
        

        egui::SidePanel::left("EXPLORER")
            .resizable(true)
            .default_width(250.0)
            .width_range(180.0..=300.0)
            .show(ctx, |ui| {
                self.explorer(ui);
        });
        
        
        
        
        
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline( &mut self.code)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_rows(20)
                            .lock_focus(true)
                            .desired_width(std::f32::INFINITY)
                           
                        );
                    //self.lang = "rs".into();
                    //self.code = "gahhh".into();
                    //self.number = 0;
                    
                })
            });
        }
}
