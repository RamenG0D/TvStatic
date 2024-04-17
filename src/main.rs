pub mod custom_random;
use raylib::{color::Color, drawing::{RaylibDraw, RaylibDrawHandle}, logging, rgui::RaylibDrawGui, rstr};
use custom_random::Random;

static mut PAUSED: bool = false;
static mut ASPECT: i32 = 30;

#[inline]
pub fn pause_menu(rd: &mut RaylibDrawHandle, rng: &mut Random) {
    if rd.is_window_resized() {
        // refresh screen
        rd.clear_background(Color::BLACK);
        
        // draw captured pause background
        draw_static(rd, rng);
        
        // redraw the pause screen gray overlay
        rd.draw_rectangle(0, 0, rd.get_screen_width(), rd.get_screen_height(), Color::new(0, 0, 0, 120));
    } else {
        rd.draw_text("PAUSED", (rd.get_screen_width() / 2)-40, 10, 20, Color::WHITE);

        // draw a button to resume the program
        if rd.gui_button(
            raylib::math::Rectangle::new(
                (rd.get_screen_width() / 2) as f32  - (130.0 / 2.0), 
                (rd.get_screen_height() / 2) as f32 - (60.0 / 2.0), 
                130.0, 
                60.0
            ), 
            Some(rstr!("Resume")
        )) {
            unsafe{PAUSED = false;}
        }

        rd.draw_text(format!("Aspect: {}", unsafe{ASPECT}).as_str(), (rd.get_screen_width() / 2) - (130 / 2), (rd.get_screen_height() / 2) - (60 / 2) + 70, 24, Color::WHITE);
        // check if the left or right arrow keys are pressed
        if rd.is_key_pressed(raylib::consts::KeyboardKey::KEY_LEFT) && unsafe{ASPECT > 1} {
            unsafe{ASPECT -= 1};
        } else
        if rd.is_key_pressed(raylib::consts::KeyboardKey::KEY_RIGHT) && unsafe{ASPECT < 50} {
            unsafe{ASPECT += 1};
        }
    }
}

fn main() {
    logging::set_trace_log(raylib::consts::TraceLogLevel::LOG_NONE); // disables logging
    let (mut rl, rt) = raylib::init().title("Tv-Static").size(600, 600).vsync().resizable().build();
    rl.set_target_fps(30);
    
    // create a random number generator (uses only pseudo-random numbers for speed!)
    let mut rng = Random::new();
    while !rl.window_should_close() {
        // check if F11 key was pressed to toggle fullscreen
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_F11) { rl.toggle_fullscreen(); }

        let mut rd: RaylibDrawHandle = rl.begin_drawing(&rt);
        // toggles the pause state
        if rd.is_key_pressed(raylib::consts::KeyboardKey::KEY_P) {
            unsafe{PAUSED = !PAUSED;}
        }

        // if the program is paused
        // draw a grey rectangle over the screen
        // then show a gui
        // and skip the rest of the loop
        if unsafe{PAUSED} {
            rd.draw_rectangle(0, 0, rd.get_screen_width(), rd.get_screen_height(), Color::new(0, 0, 0, 10));
            pause_menu(&mut rd, &mut rng);

            continue;
        }
        rd.clear_background(Color::BLACK);
        draw_static(&mut rd, &mut rng);
    }
}

#[inline]
fn draw_static(rd: &mut RaylibDrawHandle, rng: &mut Random) {
    rng.new_seed();
    // we are going to draw each pixel as a rectangle so that we can avoid doing a draw call for each pixel
    // instead we can divide the screen into a grid of rectangles (determined by ASPECT) and draw each rectangle with a random color
    let (w, h) = (rd.get_screen_width() / unsafe{ASPECT}, rd.get_screen_height() / unsafe{ASPECT});
    for x in 0..w {
        for y in 0..h {
            // rd.draw_pixel(x, y, random_color(&mut rng));
            rd.draw_rectangle(
                x * unsafe{ASPECT}, 
                y * unsafe{ASPECT}, 
                unsafe{ASPECT}, 
                unsafe{ASPECT}, 
                rng.random_color()
            );
        }
    }
}
