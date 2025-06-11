use egui::{Label, RichText, TextEdit, TextStyle, Vec2};

use crate::iop::{self};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WakfuHelperEguiApp {
    deck_string: String,
}

impl WakfuHelperEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for WakfuHelperEguiApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    let label = ui.heading("Deck: ");
                    ui.add_sized(
                        ui.available_size(),
                        TextEdit::singleline(&mut self.deck_string),
                    )
                    .labelled_by(label.id);
                });

                let spell_book = iop::get_iop_spells(&self.deck_string);
                let combo_list = iop::get_iop_combos();
                for combo in combo_list.iter() {
                    ui.separator();
                    ui.add_sized(
                        Vec2::new(
                            160.0,
                            ctx.style()
                                .text_styles
                                .get(&TextStyle::Heading)
                                .unwrap()
                                .size,
                        ),
                        Label::new(RichText::new(&combo.name).heading()),
                    );
                    ui.label(&combo.effect);
                    let mut use_sep = false;
                    ui.horizontal(|ui| {
                        for spell_cost in combo.spell_chain.iter() {
                            if use_sep {
                                ui.add_sized(
                                    Vec2 { x: 16.0, y: 16.0 },
                                    egui::Image::new(egui::include_image!(
                                        "../assets/combo-spell-separator.png"
                                    )),
                                    // .max_width(200.0),
                                );
                            }
                            // ui.add_sized(Vec2::new(64.0+8.0, 128.0), widget)
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                ui.group(|ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.set_min_width(32.0);
                                        ui.set_max_width(
                                            32.0 * 2.0 + ui.style().spacing.item_spacing.x,
                                        );

                                        for &spell in
                                            spell_book.get_matching_spell(spell_cost).iter()
                                        {
                                            ui.add_sized(Vec2 { x: 32.0, y: 32.0 }, spell);
                                        }
                                    })
                                });
                            });
                            use_sep = true;
                        }
                    });
                }
            })
        });
    }
}
