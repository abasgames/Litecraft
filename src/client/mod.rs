/*
   Copyright 2017 Miguel Peláez <kernelfreeze@greenlab.games>
   Copyright 2017 Raúl Salas <raulsalas.martin@greenlab.games>
   
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
       http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

pub extern crate allegro;
pub extern crate allegro_font;
pub extern crate allegro_image;
pub extern crate allegro_ttf;
pub extern crate allegro_sys;

pub mod resourcemanager;

use self::allegro::*;
use self::allegro_image::*;
use self::allegro_sys::base::ALLEGRO_VERSION_STR;

use scenes::scene::Scene;
use scenes::splash::SplashScreen;

use self::resourcemanager::ResourceManager;

// Versions and stuff...
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const MINECRAFT: &'static str = "1.13";

// Our data struct
pub struct Client {
    core: Core,
    queue: EventQueue,
    display: Box<Display>,
    resource_manager: ResourceManager,
    gui_scale: u8
}

impl Client {
    pub fn get_display(&self) -> &Box<Display> {
        &self.display
    }

    pub fn get_core(&self) -> &Core {
        &self.core
    }

    pub fn get_resource_manager(&self) -> &ResourceManager {
        &self.resource_manager
    }

    pub fn get_resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn scale(&self) -> u8 {
        self.gui_scale
    }
}

pub fn run(session: &str) {
    let core = Core::init().unwrap();
    ImageAddon::init(&core).unwrap();

    info!("Using Allegro v{}", ALLEGRO_VERSION_STR);

    let display = Box::new(Display::new(&core, 800, 600).unwrap());
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

    display.set_window_title("Litecraft");

    let queue = EventQueue::new(&core).unwrap();
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());

    let mut client = Client {
        core,
        queue,
        display,
        resource_manager: ResourceManager::new(),
        gui_scale: 1u8
    };

    ResourceManager::load(&mut client);

    let mut redraw = true;
    timer.start();

    info!("Starting main loop!");

    let mut scene: Box<Scene> = Box::new(SplashScreen::new());

    'exit: loop {
        if redraw && client.queue.is_empty() {
            match scene.draw(&mut client) {
                Some(s) => scene = s,
                None => (),
            };
            client.core.flip_display();

            redraw = false;
        }

        match client.queue.wait_for_event() {
            DisplayClose { .. } => break 'exit,
            TimerTick { .. } => redraw = true,
            _ => (),
        }
    }
}