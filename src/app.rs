use egui_plot::{Bar, BarChart, Plot};
use rand::Rng;
use serde::{Deserialize, Serialize};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct PityExperimentApp {
    // Parameters
    proba: f32,
    rounds: usize,
    pity_limit: usize,
    num_simu: usize,

    // Results
    #[serde(skip)]
    histogram: Option<Vec<usize>>,
    #[serde(skip)]
    last_run_simulation: usize,
}

impl Default for PityExperimentApp {
    fn default() -> Self {
        Self {
            proba: 1.0 / 20.0,
            rounds: 2000,
            pity_limit: 20,
            num_simu: 1000,
            histogram: None,
            last_run_simulation: 0,
        }
    }
}

impl PityExperimentApp {
    /// Called once before the first frame.
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
    fn pity_experiment(&self, proba: f32, num_rounds: usize, pity_limit: usize) -> usize {
        let mut rng = rand::rng();
        let mut num_wins = 0;
        let mut pity_counter = 0;
        for _ in 0..num_rounds {
            if pity_counter >= pity_limit {
                num_wins += 1;
                pity_counter = 0;
            } else {
                let win = rng.random::<f32>() < proba;
                if win {
                    pity_counter = 0;
                    num_wins += 1;
                } else {
                    pity_counter += 1;
                }
            }
        }
        num_wins
    }

    fn run_simulation(&mut self) {
        let mut hist = Vec::with_capacity(self.num_simu);
        for _ in 0..self.num_simu {
            let wins = self.pity_experiment(self.proba, self.rounds, self.pity_limit);
            hist.push(wins);
        }
        self.histogram = Some(hist);
        self.last_run_simulation += 1;
    }
}

impl eframe::App for PityExperimentApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pity Experiment Simulator");

            // Parameter controls
            ui.horizontal(|ui| {
                ui.label("Probability of winning per round:");
                ui.add(egui::Slider::new(&mut self.proba, 0.0..=1.0).fixed_decimals(1));
            });
            ui.horizontal(|ui| {
                ui.label("Rounds per simulation:");
                ui.add(egui::DragValue::new(&mut self.rounds).range(1..=100_000));
            });
            ui.horizontal(|ui| {
                ui.label("Pity limit (auto win after):");
                ui.add(egui::DragValue::new(&mut self.pity_limit).range(1..=1000));
            });
            ui.horizontal(|ui| {
                ui.label("Number of simulations:");
                ui.add(egui::DragValue::new(&mut self.num_simu).range(1..=10_000));
            });

            if ui.button("Run Simulation").clicked() {
                self.run_simulation();
            }

            // Show histogram
            if let Some(hist) = &self.histogram {
                ui.separator();
                ui.label(format!(
                    "Histogram of total wins per run (last run of simulation #{})",
                    self.last_run_simulation
                ));
               // Example: binning into 10 bars
                let num_bars = 10;
                let min = *hist.iter().min().unwrap_or(&0);
                let max = *hist.iter().max().unwrap_or(&0);
                let bin_width = ((max - min) as f64 / num_bars as f64).ceil().max(1.0) as usize;

                let mut bins: Vec<usize> = vec![0usize; num_bars];
                for &value in hist {
                    let idx = ((value - min) / bin_width).min(num_bars - 1);
                    bins[idx] += 1;
                }

                // Now create Vec<Bar>
                let bars: Vec<Bar> = bins.iter().enumerate().map(|(i, &count)| {
                    // Use the bin midpoint for x
                    let x = min as f64 + (i as f64 + 0.5) * bin_width as f64;
                    Bar::new(x, count as f64).width(bin_width as f64)
                }).collect();

                // Create and show the bar chart
                Plot::new("histogram_plot").show(ui, |plot_ui| {
                    plot_ui.bar_chart(BarChart::new(bars));
                });
            }
        });
    }
}
