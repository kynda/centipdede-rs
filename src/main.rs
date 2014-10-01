#![feature(globs)]

// Load external crates provided via cargo.
extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

// Texture is used for images; Gl for accessing OpenGL
use opengl_graphics::{
    Gl,  
};

// For creating a window
use sdl2_game_window::WindowSDL2;

// Stuff from piston.
use piston::{
    EventIterator,                  // Used for the game loop
    EventSettings,                  // Struct used for setting and updates
    WindowSettings,                 // Struct defines window config
    Render,                         // Render Evemt 
};

use piston::graphics::*;
use piston::shader_version::opengl;

// Config options for the game.
struct GameConfig {
    title: String,
    window_height: u32,
    window_width: u32,
    updates_per_second: u64,
    max_frames_per_second: u64,
    tile_size: uint
}

// Represents the Game Loop
struct Game {
    config: GameConfig
}

// Represents the Game Loop
impl Game {
        
    // Returns a new game struct
    pub fn new( config: GameConfig ) -> Game {                           
                
        // Return a new Game
        Game {
            config: config,
        }
        
    }  
        
    // Run the game loop
    pub fn run( &mut self ) {
        
        let mut window = self.window();  
        
        // Get Gl
        let ref mut gl = Gl::new( opengl::OpenGL_3_2 );

        // Create Settings for Game loop
        let event_settings = EventSettings {
                updates_per_second: self.config.updates_per_second,
                max_frames_per_second: self.config.max_frames_per_second,
            };   
    
        // For each e in Event Iterator (whose range is 0 to infinity) 
        // e becomes a new Event by passing our window and event settings
        for e in EventIterator::new(&mut window, &event_settings) {

            // If e is Render(args) do something, else return ()?
            match e {
                Render(args) => self.render( gl ),
                _ => {},
            }
        
        }
        
    }    
    
    // Returns a window.
    fn window( &self ) -> WindowSDL2 {
        
        // Values for Window Creation
        let window_settings = WindowSettings {
                title: self.config.title.to_string(),
                size: [self.config.window_width, self.config.window_height],
                fullscreen: false,
                exit_on_esc: true,
                samples: 0,
            };

        // Create SDL Window
        WindowSDL2::new(
            opengl::OpenGL_3_2,
            window_settings
        )        
    }
    
    // Render the game state to the screen.
    fn render( &mut self, gl: &mut Gl ) {

        let width = self.config.window_width;
        let height = self.config.window_height;

        // Creates viewport at 0,0 with width and height of window.
        gl.viewport(0, 0, width as i32, height as i32);

        // graphics::context a new drawing context (think html5)
        let c = Context::abs(width as f64, height as f64);

        // I'm guessing this draws white across the entire window
        c.rgb(1.0, 1.0, 1.0).draw(gl);

        // Get number of columns and rows.
        let tile_size = self.config.tile_size;        
        let num_cols = width as int / tile_size as int;
        let num_rows = height as int / tile_size as int;                                

        let mut red = 0.01;

        // Fill screen with red one 32x32 tile at a time.
        for row in range(0i, num_rows ) {

            red = red + 0.02;

            let row_shift: f64 = row as f64 * tile_size as f64;

            for col in range(0i, num_cols ) {

                let col_shift: f64 = col as f64 * tile_size as f64;

                c.square(
                    col_shift,
                    row_shift,
                    tile_size as f64
                ).rgb( red, 0.0, 0.0).draw(gl);                                

            }

        }
    }
    
}

// Entry point for our game.
fn main() {
    
    let config = GameConfig {
            title: "Centipede-RS".to_string(),
            window_height: 480,
            window_width: 800,
            updates_per_second: 120,
            max_frames_per_second: 60,
            tile_size: 32,
        };

    // Create and run new game.
    let mut game = Game::new( config );
    game.run();

}

