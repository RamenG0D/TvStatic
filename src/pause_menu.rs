use raylib::prelude::*;

use crate::ASPECT;

pub const RESUME_BUTTON: usize  = 0;
pub const BARS_BUTTON: usize    = 1;
pub const FADE_BUTTON: usize    = 2;
pub const LERP_BUTTON: usize    = 3;
pub const SPIRAL_BUTTON: usize  = 4;
pub const STATIC_BUTTON: usize  = 5;
pub const WS_BUTTON: usize      = 6;

type ButtonFn = Box<dyn FnMut()>;

static mut BUTTONS: [Option<ButtonFn>; 7] = 
[
    None, // resume_button
    None, // bars_button
    None, // fade_button
    None, // lerp_button
    None, // time-to-shine_button
    None, // static_button
    None, // whole-screen_button
];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GuiPauseMenuState {
    pub spinner_edit_mode: bool,
    pub layout_recs: [Rectangle; 9],
}
pub const fn init_gui_pause_menu() -> GuiPauseMenuState {
    let mut state: GuiPauseMenuState = GuiPauseMenuState {
        spinner_edit_mode: false,
        layout_recs: [
            Rectangle {
                x: 0.0,
                y: 0.0,
                width:  0.0,
                height: 0.0,
            }; 9
        ],
    };
    state.spinner_edit_mode = false;
    state.layout_recs[0] = Rectangle {
            x: 1.0,
            y: 1.0,
            width: 120.0,
            height: 24.0,
        };
    state.layout_recs[1] = Rectangle {
            x: 1.0,
            y: 165.0,
            width: 120.0,
            height: 24.0,
        };
    state.layout_recs[2] = Rectangle {
            x: 1.0,
            y: 265.0,
            width: 120.0,
            height: 24.0,
        };
    state.layout_recs[3] = Rectangle {
            x: 256.0,
            y: 265.0,
            width: 120.0,
            height: 24.0,
        };
    state.layout_recs[4] = Rectangle {
            x: 256.0,
            y: 165.0,
            width: 120.0,
            height: 24.0,
        };
    state.layout_recs[5] = Rectangle {
            x: -256.0,
            y: 165.0,
            width: 120.0,
            height: 24.0,
        };
    state.layout_recs[6] = Rectangle {
            x: -256.0,
            y: 265.0,
            width: 120.0,
            height: 24.0,
        };
    state.layout_recs[7] = Rectangle {
            x: 1.0,
            y: -200.0,
            width: 240.0,
            height: 48.0,
        };
    state.layout_recs[8] = Rectangle {
            x: 528.0,
            y: 312.0,
            width: 120.0,
            height: 24.0,
        };
    return state;
}

pub unsafe fn get_button<'a>(btn: usize) -> Result<&'a mut ButtonFn, String> {
    match BUTTONS[btn] {
        Some(ref mut button) => Ok(button),
        None => Err(format!("Button {btn} is not set"))
    }
}

pub unsafe fn set_button<T>(btn: usize, func: T) 
    where T: FnMut() + 'static
{
    BUTTONS[btn] = Some(Box::new(func))
}

fn move_middle(
    rec: Rectangle,
    rl: &RaylibHandle
) -> Rectangle {
    let (w, h) = (
        (rl.get_screen_width()  / 2) as f32, 
        (rl.get_screen_height() / 2) as f32
    );
    // keeps the current rectangle (x, y) position centered
    Rectangle::new(
        w - ((rec.x+rec.width ) / 2.0),
        h - ((rec.y+rec.height) / 2.0),
        rec.width,
        rec.height
    )
}

pub fn gui_pause_menu(
    rl: &mut RaylibDrawHandle,
    state: &mut GuiPauseMenuState
) {
    let moved = [
        move_middle((*state).layout_recs[0],  rl),
        move_middle((*state).layout_recs[1],  rl),
        move_middle((*state).layout_recs[2],  rl),
        move_middle((*state).layout_recs[3],  rl),
        move_middle((*state).layout_recs[4],  rl),
        move_middle((*state).layout_recs[5],  rl),
        move_middle ((*state).layout_recs[6], rl),
        move_middle((*state).layout_recs[7],  rl),
        move_middle((*state).layout_recs[8],  rl)
    ];
    if rl.gui_button (moved[0], Some(rstr!("Resume"       )))  {unsafe{ (get_button(RESUME_BUTTON).unwrap())();}} else
    if rl.gui_button (moved[1], Some(rstr!("CRT-BARS"     )))  {unsafe{ (get_button(BARS_BUTTON).unwrap())();  }} else
    if rl.gui_button (moved[2], Some(rstr!("FADE"         )))  {unsafe{ (get_button(FADE_BUTTON).unwrap())();  }} else
    if rl.gui_button (moved[3], Some(rstr!("LERP"         )))  {unsafe{ (get_button(LERP_BUTTON).unwrap())();  }} else
    if rl.gui_button (moved[4], Some(rstr!("SPIRAL"       )))  {unsafe{ (get_button(SPIRAL_BUTTON).unwrap())();   }} else
    if rl.gui_button (moved[5], Some(rstr!("STATIC"       )))  {unsafe{ (get_button(STATIC_BUTTON).unwrap())();}} else
    if rl.gui_button (moved[6], Some(rstr!("WHOLE-SCREEN" ))) {unsafe{ (get_button(WS_BUTTON).unwrap())();     }} else
    if rl.gui_spinner(moved[7], None, unsafe{&mut ASPECT}, 1, 50, (*state).spinner_edit_mode) {
        (*state).spinner_edit_mode = !( (*state).spinner_edit_mode );
    }
}
