mod cpu;
use cpu::*;

use std::env;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Missing ROM path")
    }

    let filename = &args[1];

    let mut cpu = Chip8::new();
    let sdl_context = sdl2::init().unwrap();
    cpu.load_rom(filename);
    
    let emulated_width:u16 = 64;
    let emulated_height:u16 = 32;
    let video_scale:u16 = 15;
     
    let video = sdl_context.video().expect("Unable to initialize video");
    let window = video.window("Chip8 Emulator", (emulated_width * video_scale).into(), (emulated_height * video_scale).into())
        .position_centered()
        .opengl()
        .build()
        .expect("Unable to build window");

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} => {
                    break 'gameloop;
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                    if let Some(k) = key2btn(key) {
                        cpu.update_key(k, 1);
                    }
                },
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = key2btn(key) {
                        cpu.update_key(k, 0);
                    }
                },
                _ => ()
            }
        }
        cpu.cycle();
        
        draw_screen(&cpu, &mut canvas, video_scale as u32)
    }
}

fn draw_screen(cpu: &Chip8, canvas: &mut Canvas<Window>,scale: u32) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = cpu.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel != 0 {
            let x = (i % 64) as u32;
            let y = (i / 64) as u32;

            let rect = Rect::new((x * scale) as i32, (y * scale) as i32, scale, scale);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None
    }

}
