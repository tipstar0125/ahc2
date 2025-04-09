use crate::coord::Coord;
use crate::{Input, Output};

use eframe::egui::{
    show_tooltip_at_pointer, Align2, CentralPanel, Color32, Context, FontFamily, FontId, Id, Key,
    Pos2, Rect, RichText, Slider, Stroke, Ui, Vec2,
};
use eframe::{run_native, App, Frame, NativeOptions, Storage, Theme};
use std::time::{Duration, Instant};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;
const VIS_WIDTH: f32 = 600.0;
const VIS_HEIGHT: f32 = 600.0;
const OFFSET_WIDTH: f32 = (WIDTH - VIS_WIDTH) / 2.0;
const OFFSET_HEIGHT: f32 = (HEIGHT - VIS_HEIGHT) / 2.0;
const SPEED_MIN: usize = 1;
const SPEED_MAX: usize = 10;

pub struct Egui {
    input: Input,
    output: Output,
    turn: usize,
    max_turn: usize,
    checked: bool,
    play: bool,
    speed: usize,
    instant: Instant,
    cnt: usize,
}

impl Egui {
    fn new(input: Input, output: Output, max_turn: usize) -> Self {
        Egui {
            input,
            output,
            turn: 0,
            max_turn,
            checked: true,
            play: false,
            speed: 5,
            instant: Instant::now(),
            cnt: 0,
        }
    }
}

impl App for Egui {
    fn save(&mut self, _storage: &mut dyn Storage) {}
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint_after(Duration::from_millis(5));
        if self.instant.elapsed() >= Duration::from_millis(10) {
            self.cnt += 1;
            if self.cnt % (SPEED_MIN + SPEED_MAX - self.speed) == 0
                && self.play
                && self.turn < self.max_turn - 1
            {
                self.turn += 1;
            }
            self.instant = Instant::now();
        }

        if self.turn > self.max_turn - 1 {
            self.turn = self.max_turn - 1;
        }

        CentralPanel::default().show(ctx, |ui| {
            let width = self.input.width;
            let height = self.input.height;
            let d = VIS_WIDTH / ((height.max(width)) as f32);

            view_world(ui, &self.input, d);
            view_ok_points(ui, &self.output, d);
            view_true_edges(ui, &self.input, &self.output, d);
            view_ellipses(ui, &self.output, d);
            view_true_points(ui, &self.input, d);
            view_range(ui, &self.input, d);

            ui.horizontal(|ui| {
                ui.label(RichText::new("Turn: ").size(20.0));
                ui.add(Slider::new(&mut self.turn, 0..=self.max_turn));
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new("Speed: ").size(20.0));
                ui.add(Slider::new(&mut self.speed, SPEED_MIN..=SPEED_MAX));
            });

            if ctx.input().key_released(Key::Space) {
                self.play = !self.play;
            };
            if self.turn == self.max_turn {
                self.play = false;
            }
            if ctx.input().key_pressed(Key::ArrowRight) && self.turn < self.max_turn {
                self.turn += 1;
            };
            if ctx.input().key_pressed(Key::ArrowLeft) && self.turn > 0 {
                self.turn -= 1;
            };
        });
    }
}

pub fn view_world(ui: &mut Ui, input: &Input, d: f32) {
    let view_top_left_pos = Pos2 { x: 0.0, y: 0.0 };
    let view_bottom_right_pos = Pos2 {
        x: d * input.width as f32,
        y: d * input.height as f32,
    };

    rect(
        ui,
        view_top_left_pos,
        view_bottom_right_pos,
        Color32::TRANSPARENT,
        Color32::BLACK,
        2.0,
    );
}

pub fn view_true_points(ui: &mut Ui, input: &Input, d: f32) {
    for coord in &input.xy {
        let pos = Pos2 {
            x: d * coord.x as f32,
            y: d * coord.y as f32,
        };
        circle(ui, pos, 3.0, Color32::GRAY, Color32::TRANSPARENT);
    }
}

pub fn view_range(ui: &mut Ui, input: &Input, d: f32) {
    for range in &input.range {
        let pos0 = Pos2 {
            x: d * range.0 as f32,
            y: d * range.2 as f32,
        };
        let pos1 = Pos2 {
            x: d * range.1 as f32,
            y: d * range.3 as f32,
        };
        rect(ui, pos0, pos1, Color32::TRANSPARENT, Color32::GRAY, 1.0);
    }
}

pub fn view_true_edges(ui: &mut Ui, input: &Input, output: &Output, d: f32) {
    for (i, j) in &output.true_edges {
        let pos0 = Pos2 {
            x: d * input.xy[*i].x as f32,
            y: d * input.xy[*i].y as f32,
        };
        let pos1 = Pos2 {
            x: d * input.xy[*j].x as f32,
            y: d * input.xy[*j].y as f32,
        };
        line(ui, pos0, pos1, Color32::GRAY, 1.0);
    }
}

pub fn view_ok_points(ui: &mut Ui, output: &Output, d: f32) {
    for coord in &output.ok_points {
        let pos = Pos2 {
            x: d * coord.x as f32,
            y: d * coord.y as f32,
        };
        circle(ui, pos, 3.0, Color32::GREEN, Color32::TRANSPARENT);
    }
}

fn eigen_decomposition_2x2(cov: &[[f64; 2]; 2]) -> ([f64; 2], f64) {
    let a = cov[0][0];
    let b = cov[0][1]; // = cov[1][0]
    let d = cov[1][1];

    let trace = a + d;
    let det = a * d - b * b;
    let delta = (trace * trace - 4.0 * det).sqrt();

    let lambda1 = (trace + delta) / 2.0;
    let lambda2 = (trace - delta) / 2.0;

    // 主成分の回転角（rad）
    let theta = if b.abs() > 1e-8 {
        (lambda1 - a) / b
    } else {
        0.0
    };
    let angle = theta.atan();

    ([lambda1, lambda2], angle)
}

const PI: f64 = std::f64::consts::PI;

pub fn view_ellipses(ui: &mut Ui, output: &Output, d: f32) {
    let sigma = 2.0;
    let radius = 0.2;
    for (mu, cov) in &output.ellipses {
        ellipse(
            ui,
            Color32::RED,
            Color32::TRANSPARENT,
            radius,
            d,
            mu,
            cov,
            sigma,
        );
    }
}

pub fn visualizer(input: Input, output: Output, max_turn: usize) {
    let options = NativeOptions {
        initial_window_size: Some((WIDTH, HEIGHT).into()),
        initial_window_pos: Some(Pos2 { x: 100.0, y: 100.0 }),
        resizable: false,
        default_theme: Theme::Light,
        ..NativeOptions::default()
    };
    let gui = Egui::new(input, output, max_turn);
    run_native("visualizer", options, Box::new(|_cc| Box::new(gui)));
}
// 0 <= val <= 1
pub fn color32(mut val: f32) -> Color32 {
    val = val.min(1.0);
    val = val.max(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    Color32::from_rgb(r.round() as u8, g.round() as u8, b.round() as u8)
}
pub fn txt(ui: &mut Ui, txt: &str, mut pos: Pos2, size: f32, color: Color32) {
    pos.x += OFFSET_WIDTH;
    pos.y += OFFSET_HEIGHT;
    let anchor = Align2::CENTER_CENTER;
    let font_id = FontId::new(size, FontFamily::Monospace);
    ui.painter().text(pos, anchor, txt, font_id, color);
}
pub fn line(ui: &mut Ui, mut pos1: Pos2, mut pos2: Pos2, color: Color32, stroke_width: f32) {
    pos1.x += OFFSET_WIDTH;
    pos2.x += OFFSET_WIDTH;
    pos1.y += OFFSET_HEIGHT;
    pos2.y += OFFSET_HEIGHT;
    let points = [pos1, pos2];
    let stroke = Stroke {
        width: stroke_width,
        color,
    };
    ui.painter().line_segment(points, stroke);
}
pub fn rect(
    ui: &mut Ui,
    mut pos1: Pos2,
    mut pos2: Pos2,
    fill_color: Color32,
    stroke_color: Color32,
    stroke_width: f32,
) -> Rect {
    pos1.x += OFFSET_WIDTH;
    pos2.x += OFFSET_WIDTH;
    pos1.y += OFFSET_HEIGHT;
    pos2.y += OFFSET_HEIGHT;

    let rect = Rect {
        min: pos1,
        max: pos2,
    };
    let rounding = 0.0;
    let stroke = Stroke {
        width: stroke_width,
        color: stroke_color,
    };
    ui.painter().rect(rect, rounding, fill_color, stroke);
    rect
}
pub fn circle(
    ui: &mut Ui,
    mut center: Pos2,
    radius: f32,
    fill_color: Color32,
    stroke_color: Color32,
) -> Rect {
    center.x += OFFSET_WIDTH;
    center.y += OFFSET_HEIGHT;
    let stroke = Stroke {
        width: 3.0,
        color: stroke_color,
    };
    ui.painter().circle(center, radius, fill_color, stroke);

    Rect {
        min: Pos2 {
            x: center.x - radius,
            y: center.y - radius,
        },
        max: Pos2 {
            x: center.x + radius,
            y: center.y + radius,
        },
    }
}
pub fn arrow(ui: &mut Ui, mut origin: Pos2, vec: Vec2, stroke_color: Color32, stroke_width: f32) {
    origin.x += OFFSET_WIDTH;
    origin.y += OFFSET_HEIGHT;
    let stroke = Stroke {
        width: stroke_width,
        color: stroke_color,
    };
    ui.painter().arrow(origin, vec, stroke);
}
pub fn ellipse(
    ui: &mut Ui,
    stroke_color: Color32,
    fill_color: Color32,
    stroke_width: f32,
    d: f32,
    mu: &[f64; 2],
    cov: &[[f64; 2]; 2],
    sigma: f64,
) {
    let ([lambda1, lambda2], angle) = eigen_decomposition_2x2(cov);

    let r1 = sigma * lambda1.sqrt();
    let r2 = sigma * lambda2.sqrt();

    // 楕円点列を作る
    let n = 100;
    for i in 0..=n {
        let t = 2.0 * PI * (i as f64) / (n as f64);
        // 単位円 → 楕円
        let x = r1 * t.cos();
        let y = r2 * t.sin();
        // 回転
        let x_rot = x * angle.cos() - y * angle.sin();
        let y_rot = x * angle.sin() + y * angle.cos();
        // 平行移動 & スケーリング
        let center = Pos2 {
            x: ((mu[0] as f32) + (x_rot as f32)) * d,
            y: ((mu[1] as f32) - (y_rot as f32)) * d,
        };
        circle(ui, center, stroke_width, fill_color, stroke_color);
    }
}
