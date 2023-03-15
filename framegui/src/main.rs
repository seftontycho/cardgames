use eframe::{
    egui::{self, RichText},
    run_native, App, Frame,
};

use cards::game::{
    highlow::{Action, HighLow},
    Game,
};

struct CardGamesApp {
    game: HighLow,
    pub highscore: u8,
    pub correct: Option<bool>,
}

impl CardGamesApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            game: HighLow::new(),
            highscore: 0,
            correct: None,
        }
    }
}

impl App for CardGamesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        let text_colour = ctx.style().visuals.text_color();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let window_width = ui.available_width();
                let window_height = ui.available_height();

                let scale_unit = window_width.min(window_height) / 600.0;

                // make font sizes proportional to the window size
                let font_size = 1.0_f32.max(scale_unit);

                let title_font = egui::FontId::proportional(100.0 * font_size);
                let title = RichText::new("Higher or Lower").font(title_font);
                ui.heading(title);

                // need a gap here ()
                ui.add_space(50.0 * font_size);

                let card_font = egui::FontId::proportional(40.0 * font_size);
                let card = self.game.observation();
                let card = RichText::new(format!("{} of {}", card.rank, card.suit)).font(card_font);
                ui.label(card);

                // need a gap here
                ui.add_space(50.0 * font_size);

                let button_font = egui::FontId::proportional(50.0 * font_size);

                // evenly space the buttons in the horizontal direction
                // higher centered in the first third of the space
                // lower centered in the last third of the space

                let button_width = 100.0 * scale_unit;
                let button_height = 50.0 * scale_unit;

                ui.horizontal(|ui| {
                    ui.add_space(window_width / 3.0 - button_width);

                    let higher = RichText::new("Higher").font(button_font.clone());
                    let higher =
                        egui::Button::new(higher).min_size(egui::vec2(button_width, button_height));

                    if ui.add(higher).clicked() {
                        let (_, reward, _) = self.game.step(Action::Higher);
                        self.correct = Some(reward > 0);
                    }

                    ui.add_space(window_width / 3.0 - button_width);

                    let lower = RichText::new("Lower").font(button_font.clone());
                    let lower =
                        egui::Button::new(lower).min_size(egui::vec2(button_width, button_height));

                    if ui.add(lower).clicked() {
                        let (_, reward, _) = self.game.step(Action::Lower);
                        self.correct = Some(reward > 0);
                    }
                });

                // need a gap here
                ui.add_space(50.0 * font_size);

                // score text ahould flash green if the score increased
                let current_score = self.game.score();

                let score_font = egui::FontId::proportional(40.0 * font_size);
                let score = RichText::new(format!("Score: {}", current_score)).font(score_font);

                let colour = match self.correct {
                    Some(true) => egui::Color32::GREEN,
                    Some(false) => egui::Color32::RED,
                    None => text_colour,
                };

                ui.colored_label(colour, score);

                // need a gap here
                ui.add_space(50.0 * font_size);

                if current_score > self.highscore {
                    self.highscore = current_score;
                }

                let highscore_font = egui::FontId::proportional(40.0 * font_size);
                let highscore =
                    RichText::new(format!("Highscore: {}", self.highscore)).font(highscore_font);

                ui.label(highscore);
            });
        });
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(800.0, 600.0));

    run_native(
        "CardGames",
        native_options,
        Box::new(|cc| Box::new(CardGamesApp::new(cc))),
    )
    .expect("Failed to run native");
}
