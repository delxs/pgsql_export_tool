use eframe::egui;

struct DbInfo {
    database_host: String,
    database_port: String,
    database_name: String,
    database_user: String,
    database_password: String,
}

impl eframe::App for DbInfo {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Postgres SQL Export Tools");
            });

            egui::Frame::group(ui.style())
                .outer_margin(egui::Margin::same(15))
                .inner_margin(egui::Margin::same(10))
                .corner_radius(5)
                .stroke(egui::Stroke::new(0.5, egui::Color32::from_rgb(112, 58, 33)))
                .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Database Host: ");
                    });
                    ui.add_space(2.7);
                    ui.add(egui::TextEdit::singleline(&mut self.database_host));
                    ui.end_row();

                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.label("Database Port: ");
                    });
                    ui.add_space(2.7);
                    ui.add(egui::TextEdit::singleline(&mut self.database_port));
                    ui.end_row();

                    ui.horizontal(|ui| {
                        ui.label("Database Name: ");
                    });
                    ui.add_space(2.7);
                    ui.add(egui::TextEdit::singleline(&mut self.database_name));
                    ui.end_row();

                    ui.horizontal(|ui| {
                        ui.label("Database User: ");
                    });
                    ui.add_space(2.7);
                    ui.add(egui::TextEdit::singleline(&mut self.database_user));
                    ui.end_row();

                    ui.horizontal(|ui| {
                        ui.label("Database Password: ");
                    });
                    ui.add_space(2.7);
                    ui.add(egui::TextEdit::singleline(&mut self.database_password));
                    ui.end_row();

                    ui.add_space(10.0);

                    if ui.button("Connect").clicked() {
                        // tokio-postgres 连接数据库
                        todo!("Connect to database");
                    }
                });
            });
        });
    }
}

fn main() {
    // 初始化并运行应用
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 360.0]).with_resizable(false).with_maximize_button(false),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Postgres SQL Export Tools",
        options,
        Box::new(|_cc| Ok(Box::new(DbInfo {
            database_host: String::new(),
            database_port: String::new(),
            database_name: String::new(),
            database_user: String::new(),
            database_password: String::new(),
        }))),
    );
}