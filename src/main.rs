pub mod custom_random;
pub mod pause_menu;

use pause_menu::{gui_pause_menu, init_gui_pause_menu, set_button, GuiPauseMenuState, BARS_BUTTON, FADE_BUTTON, LERP_BUTTON, RESUME_BUTTON, SPIRAL_BUTTON, STATIC_BUTTON, WS_BUTTON};
use raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, logging, math::lerp};
use custom_random::Random;


static mut PMENU: GuiPauseMenuState = init_gui_pause_menu();

static mut CLEAR_SCREEN: bool = true;
fn set_clear_screen(val: bool) {
    unsafe{CLEAR_SCREEN = val;}
}
static mut PAUSED: bool = false;
fn toggle_paused() {
    unsafe{PAUSED = !PAUSED;}
}
static mut ASPECT: i32  = 30;

type ScreenFn = fn(&mut RaylibDrawHandle, &mut Random);
static mut DRAW_SCREEN: ScreenFn = draw_static;

fn main() {
    logging::set_trace_log(raylib::consts::TraceLogLevel::LOG_NONE); // disables logging
    let (mut rl, rt) = raylib::init().title("Tv-Static").size(600, 600).vsync().resizable().build();
    rl.set_target_fps(30);
    
    // create a random number generator (uses only pseudo-random numbers for speed!)
    let mut rng = Random::new();
    
    // set button actions
    setup_buttons();

    while !rl.window_should_close() {
        // check if F11 key was pressed to toggle fullscreen
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_F11) { rl.toggle_fullscreen(); }

        // toggles the pause state
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_P) {
            toggle_paused();
        }
        
        let mut rd: RaylibDrawHandle = rl.begin_drawing(&rt);
        if unsafe{CLEAR_SCREEN || PAUSED} {
            rd.clear_background(Color::BLACK);
        }

        // if the program is paused
        // draw a grey rectangle over the screen
        // then show a gui
        // and skip the rest of the loop
        if unsafe{PAUSED} {
            // draw the pause menu
            gui_pause_menu(&mut rd, unsafe{&mut PMENU});

            rd.draw_text("PAUSED", (rd.get_screen_width() / 2)-40, 10, 20, Color::WHITE);
            rd.draw_text(format!("Aspect: {}", unsafe{ASPECT}).as_str(), (rd.get_screen_width() / 2) - (130 / 2), (rd.get_screen_height() / 2) - (60 / 2) + 70, 24, Color::WHITE);
            // check if the left or right arrow keys are pressed
            if rd.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT) && unsafe{ASPECT > 1} {
                unsafe{ASPECT -= 1};
            } else
            if rd.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT) && unsafe{ASPECT < 50} {
                unsafe{ASPECT += 1};
            }

            continue;
        }
        
        unsafe{DRAW_SCREEN(&mut rd, &mut rng)};
    }
}

fn setup_buttons() {unsafe{
    set_button(RESUME_BUTTON, || {
        PAUSED = false;
    });
    set_button(STATIC_BUTTON, || {
        set_clear_screen(true);
        DRAW_SCREEN = draw_static;
    });
    set_button(BARS_BUTTON, || {
        set_clear_screen(true);
        DRAW_SCREEN = draw_crt;
    });
    set_button(LERP_BUTTON, || {
        set_clear_screen(true);
        DRAW_SCREEN = draw_lerp;
    });
    set_button(FADE_BUTTON, || {
        set_clear_screen(false);
        DRAW_SCREEN = draw_fade;
    });
    set_button(SPIRAL_BUTTON, || {
        set_clear_screen(true);
        DRAW_SCREEN = draw_spiral;
    });
    set_button(WS_BUTTON, || {
        set_clear_screen(true);
        DRAW_SCREEN = draw_ws;
    });
}}

fn draw_lerp(
    rd: &mut RaylibDrawHandle, 
    rng: &mut Random
) {
    rng.new_seed();

    // we are going to draw each pixel as a rectangle so that we can avoid doing a draw call for each pixel
    // instead we can divide the screen into a grid of rectangles (determined by ASPECT) and draw each rectangle with a random color
    let aspect = unsafe{ASPECT};
    let (w, h) = (
        rd.get_screen_width()  / aspect, 
        rd.get_screen_height() / aspect
    );

    let mut last_color = rng.random_color();
    for x in 0..w {
        for y in 0..h {
            // we will lerp between the new color and the last color
            let lerp_val = rng.random_range_float(0.0..1.0) as f32;
            let color = rng.random_color();

            // lerp between the last color and the new color
            let color = Color::new(
                lerp(last_color.r as f32, color.r as f32, lerp_val) as u8,
                lerp(last_color.g as f32, color.g as f32, lerp_val) as u8,
                lerp(last_color.b as f32, color.b as f32, lerp_val) as u8,
                lerp(last_color.a as f32, color.a as f32, lerp_val) as u8
            );
            
            rd.draw_rectangle(
                x * aspect, 
                y * aspect, 
                aspect, 
                aspect, 
                color
            );

            last_color = color;
        }
    }
}

fn draw_spiral(
    rd: &mut RaylibDrawHandle, 
    rng: &mut Random
) {
    rng.new_seed();

    let aspect = unsafe { ASPECT };
    let screen_width = rd.get_screen_width();
    let screen_height = rd.get_screen_height();
    let mut pixels_drawn = 0;
    let mut x = screen_width / 2;
    let mut y = screen_height / 2;
    let mut direction = 0;
    let mut length = 1;
    let mut color = rng.random_color();

    while pixels_drawn < screen_width * screen_height {
        for _ in 0..length {
            rd.draw_rectangle(x, y, aspect, aspect, color);
            pixels_drawn += 1;
            match direction {
                0 => x += aspect,
                1 => y += aspect,
                2 => x -= aspect,
                3 => y -= aspect,
                _ => {}
            }
        }
        direction = (direction + 1) % 4;
        if direction % 2 == 0 {
            length += 1;
        }
        color = rng.random_color();
    }
}

fn draw_ws(
    rd: &mut RaylibDrawHandle, 
    rng: &mut Random
) {
    #[allow(non_upper_case_globals)]
    static mut last_color: Color = Color::BLACK;
    rng.new_seed();

    // here we just get a random color and ensure its not to different from the last color
    let color = rng.random_color();
    let color = Color::new(
        lerp(unsafe{last_color.r as f32}, color.r as f32, 0.5) as u8,
        lerp(unsafe{last_color.g as f32}, color.g as f32, 0.5) as u8,
        lerp(unsafe{last_color.b as f32}, color.b as f32, 0.5) as u8,
        lerp(unsafe{last_color.a as f32}, color.a as f32, 0.5) as u8
    );

    // draw the color to the screen
    rd.draw_rectangle(0, 0, rd.get_screen_width(), rd.get_screen_height(), color);
}

fn draw_fade(
    rd: &mut RaylibDrawHandle, 
    rng: &mut Random
) {
    rng.new_seed();

    // we are going to draw each pixel as a rectangle so that we can avoid doing a draw call for each pixel
    // instead we can divide the screen into a grid of rectangles (determined by ASPECT) and draw each rectangle with a random color
    let aspect = unsafe{ASPECT};
    let (w, h) = (
        rd.get_screen_width()  / aspect, 
        rd.get_screen_height() / aspect
    );

    for x in 0..w {
        for y in 0..h {
            let rgb = rng.random_range(0..255) as u8;
            let color = Color::new(rgb,rgb,rgb, rgb);
            rd.draw_rectangle(
                x * aspect, 
                y * aspect, 
                aspect, 
                aspect, 
                color
            );
        }
    }
}

fn draw_crt(
    rd: &mut RaylibDrawHandle, 
    rng: &mut Random
) {
    rng.new_seed();

    // we are going to draw each pixel as a rectangle so that we can avoid doing a draw call for each pixel
    // instead we can divide the screen into a grid of rectangles (determined by ASPECT) and draw each rectangle with a random color
    let aspect = unsafe{ASPECT};

    // draw the CRT scanlines each with a random color, and using the aspect to determine the line thickness
    for y in 0..rd.get_screen_width() {
        if y % aspect == 0 {
            let color = rng.random_color();
            rd.draw_rectangle(
                y, 
                0, 
                aspect, 
                rd.get_screen_height(), 
                color
            );
        }
    }
}

fn draw_static(
    rd: &mut RaylibDrawHandle, 
    rng: &mut Random
) {
    rng.new_seed();

    // we are going to draw each pixel as a rectangle so that we can avoid doing a draw call for each pixel
    // instead we can divide the screen into a grid of rectangles (determined by ASPECT) and draw each rectangle with a random color
    let aspect = unsafe{ASPECT};
    let (w, h) = (
        rd.get_screen_width()  / aspect, 
        rd.get_screen_height() / aspect
    );

    for x in 0..w {
        for y in 0..h {
            let color = rng.random_color();
            rd.draw_rectangle(
                x * aspect, 
                y * aspect, 
                aspect, 
                aspect, 
                color
            );
        }
    }
}
