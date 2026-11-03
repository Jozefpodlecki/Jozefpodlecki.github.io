use crate::env::MotionService;
use animato::{Easing, Tween, Update};
use gloo::timers::callback::Interval;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RevealProps {
    pub children: Children,
    #[prop_or_default]
    pub delay_ms: u32,
}

#[function_component(MotionReveal)]
pub fn motion_reveal(props: &RevealProps) -> Html {
    let motion = use_context::<MotionService>().expect("MotionService provider");
    let progress = use_state(|| if motion.prefers_reduced_motion() { 1.0_f32 } else { 0.0_f32 });

    {
        let progress = progress.clone();
        use_effect_with((props.delay_ms, motion), move |(delay, motion)| {
            let timer = Rc::new(RefCell::new(None::<Interval>));

            if motion.prefers_reduced_motion() {
                progress.set(1.0);
            } else if let Some(started) = motion.now_ms() {
                progress.set(0.0);
                let delay = f64::from(*delay);
                let mut last = started + delay;
                let mut tween = Tween::new(0.0_f32, 1.0)
                    .duration(0.48)
                    .easing(Easing::EaseOutCubic)
                    .build();
                let cancel = timer.clone();
                let motion = motion.clone();
                let interval = Interval::new(16, move || {
                    let Some(now) = motion.now_ms() else {
                        progress.set(1.0);
                        cancel.borrow_mut().take();
                        return;
                    };

                    if now - started < delay {
                        return;
                    }

                    let dt = ((now - last).clamp(0.0, 50.0) / 1000.0) as f32;
                    last = now;
                    tween.update(dt);
                    progress.set(tween.value());

                    if tween.is_complete() {
                        cancel.borrow_mut().take();
                    }
                });
                *timer.borrow_mut() = Some(interval);
            } else {
                progress.set(1.0);
            }

            move || {
                timer.borrow_mut().take();
            }
        });
    }

    let progress = *progress;
    let style = format!(
        "opacity: {progress:.3}; transform: translate3d(0, {:.1}px, 0);",
        (1.0 - progress) * 18.0
    );

    html! {
        <div data-component="motion-reveal" style={style}>
            {for props.children.iter()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use animato::{Easing, Tween, Update};

    #[test]
    fn reveal_tween_reaches_visible_state() {
        let mut tween = Tween::new(0.0_f32, 1.0)
            .duration(0.48)
            .easing(Easing::EaseOutCubic)
            .build();
        tween.update(0.48);
        assert!(tween.is_complete());
        assert!((tween.value() - 1.0).abs() < 0.001);
    }
}
