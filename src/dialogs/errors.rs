use std::path::PathBuf;
use std::sync::mpsc::Sender;
use egui::{Modal, RichText, Ui};
use egui_extras::{Column, TableBuilder};
use crate::UiCommand;

pub fn show_errors_modal(
    ui: &mut Ui,
    errors: &Vec<String>,
    sender: Sender<UiCommand>,
) {
    let modal_id = ui.id().with("errors");

    Modal::new(modal_id).show(ui.ctx(), |ui| {
        ui.set_width(ui.available_width() * 0.8);

        ui.add(egui::Label::new(RichText::from("Errors dialog title").heading()).selectable(false));

        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .column(Column::auto())
            .column(Column::remainder());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Errors table header");
                });
            })
            .body(|mut body| {
                for (index, error) in errors.iter().enumerate() {
                    body.row(18.0, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", index));
                        });
                        row.col(|ui| {
                            ui.label(error);
                        });
                    })
                }
            });

        egui::Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                if ui
                    .button("Ok")
                    .clicked()
                {
                    sender.send(UiCommand::ClearErrors).expect("sent");
                }
            },
        );
    });
}
