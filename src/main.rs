use std::iter::repeat_with;
use std::sync::mpsc::{Receiver, Sender};
use eframe::{App, CreationContext};
use fastrand::Rng;

mod dialogs;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 440.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "egui-modals",
        options,
        Box::new(|cc| Ok(Box::new(UiApp::new(cc)))),
    ) {
        eprintln!("Failed to run eframe: {:?}", e);
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct UiApp {
    #[serde(skip)]
    state: Option<UiState>,
}

struct UiState {
    errors: Vec<String>,
    sender: Sender<UiCommand>,
    receiver: Receiver<UiCommand>,
    rng: Rng,
}

impl UiApp {
    fn new(cc: &CreationContext) -> UiApp {
        let mut instance = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        };
        
        instance.state = Some({
            let (sender, receiver) = std::sync::mpsc::channel::<UiCommand>();
            let mut rng = Rng::new();

            UiState {
                errors: vec![],
                sender,
                receiver,
                rng,
            }
        });
        
        instance
    }
}


impl App for UiApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(state) = self.state.as_mut() {
                if ui.button("make 10..=50 errors").clicked() {
                    let error_count = state.rng.usize(10..=50);
                    state.errors = make_errors(error_count);
                }

                if ui.button("make 10 errors").clicked() {
                    state.errors = make_errors(10);
                }

                if ui.button("make 50 errors").clicked() {
                    state.errors = make_errors(50);
                }

                if !state.errors.is_empty() {
                    dialogs::errors::show_errors_modal(ui, &state.errors, state.sender.clone());
                }
                
                while let Ok(ui_command) = state.receiver.try_recv() {
                    match ui_command {
                        UiCommand::ClearErrors => {
                            state.errors.clear();
                        }
                    }
                }
            }
        });
    }
}

fn make_errors(error_count: usize) -> Vec<String> {
    let errors = {
        let mut name_gen = names::Generator::with_naming(names::Name::Numbered);

        repeat_with(move || {
            name_gen.next().unwrap()
        })
    }
        .take(error_count)
        .collect::<Vec<String>>();
    
    errors
}

pub enum UiCommand {
    ClearErrors
}