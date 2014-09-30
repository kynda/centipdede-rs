#![feature(globs)]

// Load external crates provided via cargo.
extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

// Texture is used for images; Gl for accessing OpenGL
use opengl_graphics::{
    Gl,
    Texture,
};

// For creating a window
use sdl2_game_window::WindowSDL2;


use piston::{
    AssetStore,                     // Managing assets
    EventIterator,                  // Used for the game loop
    EventSettings,                  // Struct used for setting and updates
    WindowSettings,                 // Struct defines window config
    Render,                         // Called each iteration of the game loop
};

use piston::graphics::*;

// Game Constants
pub static WIN_HEIGHT: int = 480;
pub static WIN_WIDTH: int = 800;

fn run( config: GameConfig ) {
  
    // Values for Window Creation
    let opengl = piston::shader_version::opengl::OpenGL_3_2;
    let window_settings = WindowSettings {
            title: "Centipede-RS".to_string(),
            size: [config.window_width, config.window_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        };
    
    // Create SDL Window
    let mut window = WindowSDL2::new(
        opengl,
        window_settings
    );

    // Create Asset Store
    let asset_store = AssetStore::from_folder("../bin/assets");

    // Load Logo from Asset Store
    let image = asset_store.path("rust-logo.png").unwrap();
    let image = Texture::from_path(&image).unwrap();

    // Create Settings for Game loop
    let event_settings = EventSettings {
            updates_per_second: config.updates_per_second,
            max_frames_per_second: config.max_frames_per_second,
        };
        
    // Create new OpenGL Handle
    let ref mut gl = Gl::new(opengl);
    
    let mut yellow = 0.0;
    
    // For each e in Event Iterator (whose range is 0 to infinity) 
    // e becomes a new Event by passing our window and event settings
    for e in EventIterator::new(&mut window, &event_settings) {
        
        // If e is Render(args) do something, else return ()?
        match e {
            Render(args) => {
                
                yellow = yellow + 0.001;
                
                // Creates viewport at 0,0 with width and height of window.
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                // graphics::context a new drawing context (think html5)
                let c = Context::abs(args.width as f64, args.height as f64);
                
                // I'm guessing this draws white across the entire window
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                
                let num_cols = config.window_width as int / config.tile_size as int;
                
                let num_rows = config.window_height as int / config.tile_size as int;                                
                
                let mut green = 0.0;
                
                // Draw a square   
                for row in range(0i, num_rows ) {
                    
                    let mut red = 0.0;
                    
                    green = green + 0.02;
                
                    let row_shift: f64 = row as f64 * config.tile_size as f64;
                
                    for col in range(0i, num_cols ) {

                        let col_shift: f64 = col as f64 * config.tile_size as f64;
                        
                        red = red + 0.02;

                        c.square(
                            col_shift,
                            row_shift,
                            config.tile_size as f64
                        ).rgb(red, green, yellow).draw(gl);

                    }
                    
                }
                
                // Draws the texture we loaded previously in the screen.
                // Next question: how do I draw this somewhere else?
                //c.image(&image).draw(gl);
            },
            _ => {},
        }
    }    
    
}

struct GameConfig {
    window_height: u32,
    window_width: u32,
    updates_per_second: u64,
    max_frames_per_second: u64,
    tile_size: uint
}

fn main() {
    
    let config = GameConfig {
            window_height: 480,
            window_width: 800,
            updates_per_second: 120,
            max_frames_per_second: 60,
            tile_size: 32,
        };

    run( config );

}

