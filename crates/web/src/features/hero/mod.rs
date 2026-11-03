mod field;
mod info;

use std::{cell::RefCell, rc::Rc};

use gloo::render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, PointerEvent};
use yew::prelude::*;

use crate::env::MotionService;

use field::{FlowField, FlowMode};
use info::FlowFieldInfo;

fn schedule_frame(
    field: Rc<RefCell<FlowField>>,
    handle: Rc<RefCell<Option<AnimationFrame>>>,
) {
    let next_field = field.clone();
    let next_handle = handle.clone();

    let frame = request_animation_frame(move |timestamp| {
        next_field.borrow_mut().render(timestamp);
        schedule_frame(next_field, next_handle);
    });

    *handle.borrow_mut() = Some(frame);
}

#[function_component(HeroFlowField)]
pub fn hero_flow_field() -> Html {
    let canvas = use_node_ref();
    let pointer = use_mut_ref(|| None::<(f64, f64)>);
    let mode = use_state(FlowMode::default);
    let mode_value = use_mut_ref(FlowMode::default);
    let motion = use_context::<MotionService>().expect("MotionService provider missing");
    let reduced_motion = motion.prefers_reduced_motion();

    {
        let canvas = canvas.clone();
        let pointer = pointer.clone();
        let mode_value = mode_value.clone();

        use_effect_with(reduced_motion, move |reduced| {
            let frame = Rc::new(RefCell::new(None::<AnimationFrame>));
            let mut resize_listener = None;

            if let Some(canvas) = canvas.cast::<HtmlCanvasElement>() {
                let window = canvas.owner_document().and_then(|document| document.default_view());

                if let Some(field) = FlowField::new(canvas, pointer, mode_value, *reduced) {
                    let field = Rc::new(RefCell::new(field));
                    field.borrow_mut().render(0.0);

                    if *reduced {
                        let redraw = field.clone();
                        if let Some(window) = window {
                            resize_listener = Some(gloo::events::EventListener::new(
                                &window,
                                "resize",
                                move |_| redraw.borrow_mut().render(0.0),
                            ));
                        }
                    } else {
                        schedule_frame(field, frame.clone());
                    }
                }
            }

            move || {
                frame.borrow_mut().take();
                drop(resize_listener);
            }
        });
    }

    let onpointermove = {
        let pointer = pointer.clone();
        Callback::from(move |event: PointerEvent| {
            let target = event.current_target();
            let Some(canvas) = target.and_then(|target| target.dyn_into::<HtmlCanvasElement>().ok()) else {
                return;
            };

            let width = f64::from(canvas.client_width());
            let height = f64::from(canvas.client_height());

            if width > 0.0 && height > 0.0 {
                *pointer.borrow_mut() = Some((
                    f64::from(event.offset_x()) / width,
                    f64::from(event.offset_y()) / height,
                ));
            }
        })
    };

    let onpointerleave = {
        let pointer = pointer.clone();
        Callback::from(move |_: PointerEvent| {
            *pointer.borrow_mut() = None;
        })
    };

    let on_mode_change = {
        let mode = mode.clone();
        let mode_value = mode_value.clone();
        Callback::from(move |next: FlowMode| {
            *mode_value.borrow_mut() = next;
            mode.set(next);
        })
    };

    html! {
        <div
            data-component="hero-flow-visual"
            class="hidden min-w-0 min-[851px]:block"
        >
            <div
                data-component="hero-flow-field"
                data-motion={if reduced_motion { "static" } else { "animated" }}
                data-flow-mode={mode.label()}
                class="relative min-h-[240px] aspect-[4/3] overflow-hidden rounded-2xl border border-divider bg-[var(--hero-field-surface)]"
                aria-hidden="true"
            >
                <div class="hero-flow-ambient pointer-events-none absolute inset-0" />
                <canvas
                    ref={canvas}
                    data-component="hero-flow-canvas"
                    class="absolute inset-0 h-full w-full"
                    onpointermove={onpointermove}
                    onpointerleave={onpointerleave}
                />
                <div class="hero-flow-vignette pointer-events-none absolute inset-0 rounded-2xl" />
            </div>
            <FlowFieldInfo
                mode={*mode}
                on_mode_change={on_mode_change}
            />
        </div>
    }
}
