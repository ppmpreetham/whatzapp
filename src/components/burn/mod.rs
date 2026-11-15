use std::cell::{Cell, RefCell};
use std::time::Instant;

use blenderui::motion::use_motion_value;
use blenderui::prelude::*;
use blenderui::vars;
use blenderui::{sys, RenderTarget, ShaderEffect, TargetFormat};

use crate::components::qr::qr_pixels;
use crate::theme;

const COMMON: &str = include_str!("common.glsl");
const EDGE: &str = include_str!("edge.glsl");
const STATE: &str = include_str!("state.glsl");
const FLUID: &str = include_str!("fluid.glsl");
const GLOW: &str = include_str!("glow.glsl");
const DOWN: &str = include_str!("down.glsl");
const UP: &str = include_str!("up.glsl");
const COMPOSITE: &str = include_str!("composite.glsl");

const STEP: f32 = 1.0 / 120.0;
const MAX_STEPS: u32 = 4;
const SIM_ROWS: u32 = 400;
const LEVELS: usize = 4;
const IGNITE: [f32; 4] = [0.54, 0.5, 0.46, 0.5];

fn effect(name: &str, pass: &str, inputs: usize, floats: usize) -> Result<ShaderEffect, String> {
    ShaderEffect::new(name, &format!("{COMMON}\n{pass}"), inputs, floats)
}

struct Texture(i32);

impl Drop for Texture {
    fn drop(&mut self) {
        // SAFETY: the texture is ours, made on this thread.
        unsafe { sys::BUI_texture_free(self.0) };
    }
}

fn page(width: u32, height: u32, background: [f32; 3], qr: Option<(Vec<u8>, usize)>) -> Vec<u8> {
    let (w, h) = (width as usize, height as usize);
    let bg = background.map(|c| (c.clamp(0.0, 1.0) * 255.0).round() as u8);
    let mut rgba = Vec::with_capacity(w * h * 4);
    for _ in 0..w * h {
        rgba.extend_from_slice(&[bg[0], bg[1], bg[2], 255]);
    }
    if let Some((code, side)) = qr {
        let (x0, y0) = (w.saturating_sub(side) / 2, h.saturating_sub(side) / 2);
        for row in 0..side.min(h) {
            let to = ((y0 + row) * w + x0) * 4;
            let n = side.min(w - x0) * 4;
            rgba[to..to + n].copy_from_slice(&code[row * side * 4..row * side * 4 + n]);
        }
    }
    rgba
}

pub struct Sim {
    size: (f32, f32),
    sim: (u32, u32),
    _page: Texture,
    page_id: i32,
    edge: RenderTarget,
    state: [RenderTarget; 2],
    fluid: [RenderTarget; 2],
    down: Vec<RenderTarget>,
    up: Vec<RenderTarget>,
    fx_edge: ShaderEffect,
    fx_state: ShaderEffect,
    fx_fluid: ShaderEffect,
    fx_glow: ShaderEffect,
    fx_down: ShaderEffect,
    fx_up: ShaderEffect,
    fx_composite: ShaderEffect,
    cur_state: usize,
    cur_fluid: usize,
    clock: f32,
    acc: f32,
    steps_run: u32,
    frames: u32,
}

impl Sim {
    pub fn new(width: f32, height: f32, background: [f32; 3], qr: Option<(Vec<u8>, usize)>) -> Option<Self> {
        let (pw, ph) = (width.round().max(1.0) as u32, height.round().max(1.0) as u32);
        let aspect = width / height;
        let sim = (((SIM_ROWS as f32) * aspect).round().max(1.0) as u32, SIM_ROWS);
        let image = blenderui::prelude::Image::from_rgba(pw, ph, &page(pw, ph, background, qr))?;
        let page_tex = Texture(image.texture());
        let target = |w: u32, h: u32, f| RenderTarget::new(w.max(1), h.max(1), f);
        let half = TargetFormat::Rgba16F;
        let level = |t: usize| ((sim.0 >> t).max(1), (sim.1 >> t).max(1));
        Some(Self {
            size: (width, height),
            sim,
            page_id: page_tex.0,
            _page: page_tex,
            edge: target(sim.0, sim.1, TargetFormat::Rgba8)?,
            state: [target(sim.0, sim.1, half)?, target(sim.0, sim.1, half)?],
            fluid: [target(sim.0, sim.1, half)?, target(sim.0, sim.1, half)?],
            down: (0..=LEVELS).map(|t| target(level(t).0, level(t).1, half)).collect::<Option<_>>()?,
            up: (0..LEVELS).map(|t| target(level(t).0, level(t).1, half)).collect::<Option<_>>()?,
            fx_edge: effect("burn-edge", EDGE, 1, 4).ok()?,
            fx_state: effect("burn-state", STATE, 2, 16).ok()?,
            fx_fluid: effect("burn-fluid", FLUID, 2, 13).ok()?,
            fx_glow: effect("burn-glow", GLOW, 3, 9).ok()?,
            fx_down: effect("burn-down", DOWN, 1, 2).ok()?,
            fx_up: effect("burn-up", UP, 2, 3).ok()?,
            fx_composite: effect("burn-composite", COMPOSITE, 5, 9).ok()?,
            cur_state: 0,
            cur_fluid: 0,
            clock: 0.1,
            acc: 0.0,
            steps_run: 0,
            frames: 0,
        })
    }

    pub fn errors(&self) -> String {
        [&self.fx_edge, &self.fx_state, &self.fx_fluid, &self.fx_glow, &self.fx_down, &self.fx_up, &self.fx_composite]
            .iter()
            .map(|e| e.error())
            .find(|e| !e.is_empty())
            .unwrap_or_default()
    }

    pub fn run(&mut self, dt: f32, time: f32, mut draw: impl FnMut(Option<&RenderTarget>, &ShaderEffect, &[i32], &[f32])) {
        let dt = dt.clamp(0.0, 0.05);
        let (w, h) = self.size;
        let aspect = w / h;
        let sim = self.sim;

        if self.frames < 3 {
            draw(Some(&self.edge), &self.fx_edge, &[self.page_id], &[1.3 / w, 1.3 / h, 1.0 / sim.0 as f32, 1.0 / sim.1 as f32]);
        }
        self.frames += 1;

        self.acc += dt;
        let mut steps = 0;
        while self.acc >= STEP && steps < MAX_STEPS {
            self.acc -= STEP;
            steps += 1;
            let ignite = if self.steps_run < 2 { 1.0 } else { 0.0 };
            let speed = (1.4 + (self.clock - 0.1) * 0.24).min(3.4);
            let next = 1 - self.cur_state;
            draw(
                Some(&self.state[next]),
                &self.fx_state,
                &[self.state[self.cur_state].texture(), self.edge.texture()],
                &[
                    w, h, 0.5, 0.5, IGNITE[0], IGNITE[1], IGNITE[2], IGNITE[3], STEP, self.clock, aspect, ignite, 0.0, speed, 0.0, 0.0,
                ],
            );
            self.cur_state = next;
            self.steps_run += 1;
            self.clock += STEP;
        }
        if steps == MAX_STEPS {
            self.acc = 0.0;
        }

        let next = 1 - self.cur_fluid;
        draw(
            Some(&self.fluid[next]),
            &self.fx_fluid,
            &[self.state[self.cur_state].texture(), self.fluid[self.cur_fluid].texture()],
            &[dt, time, aspect, 1.0, 0.0, 0.5, 0.5, 0.0, 0.0, IGNITE[0], IGNITE[1], IGNITE[2], IGNITE[3]],
        );
        self.cur_fluid = next;
        let state = self.state[self.cur_state].texture();
        let fluid = self.fluid[self.cur_fluid].texture();

        draw(
            Some(&self.down[0]),
            &self.fx_glow,
            &[state, fluid, self.edge.texture()],
            &[w, h, w * 0.5, h * 0.5, 1.0, 1.0, 0.0, time, 0.0],
        );
        for t in 1..=LEVELS {
            let (pw, ph) = (self.down[t - 1].size().0 as f32, self.down[t - 1].size().1 as f32);
            draw(Some(&self.down[t]), &self.fx_down, &[self.down[t - 1].texture()], &[1.0 / pw, 1.0 / ph]);
        }
        let mut smaller = self.down[LEVELS].texture();
        for t in (0..LEVELS).rev() {
            let (sw, sh) = (self.down[t + 1].size().0 as f32, self.down[t + 1].size().1 as f32);
            draw(Some(&self.up[t]), &self.fx_up, &[smaller, self.down[t].texture()], &[0.5 / sw, 0.5 / sh, 0.3]);
            smaller = self.up[t].texture();
        }

        draw(
            None,
            &self.fx_composite,
            &[state, fluid, self.up[0].texture(), self.edge.texture(), self.page_id],
            &[w, h, w * 0.5, h * 0.5, 1.0, 1.0, time, 0.0, 0.0],
        );
    }
}

#[component]
pub fn Burn(data: SharedString) {
    let tick = use_motion_value(0.0);
    use_effect!(move || { tick.animate(1.0, vars! { duration: 14.0, ease: "none" }); }, []);

    let background = theme::rgb("background");
    let sim: RefCell<Option<Sim>> = RefCell::new(None);
    let start = Instant::now();
    let last = Cell::new(start);

    rsx! {
        {canvas(move |bounds, painter: &mut Painter| {
            let _ = tick.get();
            let mut sim = sim.borrow_mut();
            if sim.is_none() {
                *sim = Sim::new(bounds.size.width, bounds.size.height, background, qr_pixels(&data));
            }
            let Some(sim) = sim.as_mut() else { return };
            let now = Instant::now();
            let dt = now.duration_since(last.replace(now)).as_secs_f32();
            let time = start.elapsed().as_secs_f32() % 1000.0;
            sim.run(dt, time, |target, effect, inputs, floats| match target {
                Some(t) => painter.effect_to(bounds, t, effect, inputs, floats),
                None => painter.effect(bounds, effect, inputs, floats, None),
            });
        })
        .size_full()}
    }
}
