/*
 * Copyright 2017 Miguel Peláez <kernelfreeze@outlook.com>
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

use client::Client;
use scenes::scene::Scene;
use scenes::gui::Component;
use client::resourcemanager::ResourceManager;
use scenes::mainmenu::MainMenu;

use allegro::Color;

pub struct SplashScreen;

impl Component for SplashScreen {}

impl Scene for SplashScreen {
    fn draw(&self, client: &mut Client) -> Option<Box<Scene>> {
        client.get_core().clear_to_color(Color::from_rgb_f(1.0, 1.0, 1.0));

        let w = client.get_display().get_width() as f32;
        let h = client.get_display().get_height() as f32;

        let x = w / 2.0;
        let y = h / 2.0;

        let color = Color::from_rgb_f(1f32, 1f32, 1f32);

        self.draw_2d(client, 0.0, 0.0, w, h, "background");
        self.draw_2d(client, x - 100.0, y - 130.0, 200.0, 200.0, "logo");
        self.draw_litecraft_text(client, color, "Starting Litecraft!", x, y + 90.0);

        if !ResourceManager::load_assets(client) {
            return Some(Box::new(MainMenu::new()));
        }

        None
    }
}

impl SplashScreen {
    pub fn new() -> Self {
        SplashScreen {}
    }
}