use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::cell::RefCell;

mod kdtree;

use kdtree::KDTree;

struct Scene {
    tree: Option<KDTree>,
    click: Option<[f64; 2]>,
}

thread_local! {
    static SCENE: RefCell<Scene> = RefCell::new(Scene {
        tree: None,
        click: None,
    });
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    generate()
}

#[wasm_bindgen]
pub fn generate() -> Result<(), JsValue> {
    let canvas = canvas()?;
    let tree = generate_kdtree(canvas.width() as f64, canvas.height() as f64);
    
    SCENE.with(|scene| {
        let mut scene = scene.borrow_mut();
        scene.tree = Some(tree);
        paint(&canvas, &scene)
    })
}

#[wasm_bindgen]
pub fn query(_: f64, _: f64) -> Result<(), JsValue> {
    Ok(())
}

fn rand() -> f64 {
    let mut bytes = [0u8; 8];
    let _ = getrandom::getrandom(&mut bytes);
    let num_u64 = u64::from_ne_bytes(bytes);
    let num_u53 = num_u64 & 0x001F_FFFF_FFFF_FFFF;
    num_u53 as f64 / (1u64<<53) as f64
}

fn generate_kdtree(width: f64, height: f64) -> KDTree {
    let items: Vec<_> =
        ('a'..='z').map(|c| (c, [width * rand(), height * rand()])).collect();

    KDTree::new(items)
}

fn canvas() -> Result<HtmlCanvasElement, JsValue> {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    Ok(canvas)
}

fn paint(canvas: &HtmlCanvasElement, scene: &Scene) -> Result<(), JsValue> {
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    context.set_fill_style_str("#000000");
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    if scene.tree.is_none() {
        return Ok(())
    }

    let tree = scene.tree.as_ref().unwrap();
    let items = tree.export();

    paint_edges(&context, &items);
    paint_nodes(&context, &items)?;
    paint_titles(&context, &items)
}

fn paint_edges(context: &CanvasRenderingContext2d, items: &[(&char, [f64; 2], Option<usize>, Option<usize>)]) {
    context.set_line_width(1.0);
    
    context.set_stroke_style_str("#ff0000");
    context.begin_path();
    for item in items {
        if let Some(left) = item.2 {
            context.move_to(item.1[0], item.1[1]);
            context.line_to(items[left].1[0], items[left].1[1]);
        }
    }
    context.stroke();

    context.set_stroke_style_str("#0000ff");
    context.begin_path();
    for item in items {
        if let Some(right) = item.3 {
            context.move_to(item.1[0], item.1[1]);
            context.line_to(items[right].1[0], items[right].1[1]);
        }
    }
    context.stroke();
}

fn paint_node(context: &CanvasRenderingContext2d, point: [f64; 2], radius: f64) -> Result<(), JsValue> {
    context.move_to(point[0] + radius, point[1]);
    context.arc(point[0], point[1], radius, 0.0, std::f64::consts::TAU)
}

fn paint_nodes(context: &CanvasRenderingContext2d, items: &[(&char, [f64; 2], Option<usize>, Option<usize>)]) -> Result<(), JsValue> {
    context.set_fill_style_str("#ffffff");
    context.begin_path();
    paint_node(context, items[0].1, 8.0)?;
    for i in 1..items.len() {
        paint_node(context, items[i].1, 4.0)?;
    }
    context.fill();

    Ok(())
}

fn paint_titles(context: &CanvasRenderingContext2d, items: &[(&char, [f64; 2], Option<usize>, Option<usize>)]) -> Result<(), JsValue> {
    context.set_fill_style_str("#00ff00");
    context.set_font("14px sans-serif");
    for item in items {
        context.fill_text(&item.0.to_string(), item.1[0] + 8.0, item.1[1])?;
    }
    Ok(())
}