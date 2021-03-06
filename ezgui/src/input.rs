// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use piston::input::{Button, Event, IdleArgs, Key, PressEvent};
use std::collections::HashMap;

// As we check for user input, record the input and the thing that would happen. This will let us
// build up some kind of OSD of possible actions.
pub struct UserInput {
    event: Event,
    event_consumed: bool,
    unimportant_actions: Vec<String>,
    important_actions: Vec<String>,

    // If two different callers both expect the same key, there's likely an unintentional conflict.
    reserved_keys: HashMap<Key, String>,

    // TODO hack :(
    empty_event: Event,
}

// TODO it'd be nice to automatically detect cases where two callers are trying to check for the
// same key in the same round. probably indicates a lack of exclusive editor-or-simulation checks

impl UserInput {
    pub fn new(event: Event) -> UserInput {
        UserInput {
            event,
            event_consumed: false,
            unimportant_actions: Vec::new(),
            important_actions: Vec::new(),
            reserved_keys: HashMap::new(),
            empty_event: Event::from(IdleArgs { dt: 0.0 }),
        }
    }

    pub fn number_chosen(&mut self, num_options: usize, action: &str) -> Option<usize> {
        assert!(num_options >= 1 && num_options <= 9);

        // TODO less repetition, an array of keys probably
        if num_options >= 1 {
            self.reserve_key(Key::D1, action);
        }
        if num_options >= 2 {
            self.reserve_key(Key::D2, action);
        }
        if num_options >= 3 {
            self.reserve_key(Key::D3, action);
        }
        if num_options >= 4 {
            self.reserve_key(Key::D4, action);
        }
        if num_options >= 5 {
            self.reserve_key(Key::D5, action);
        }
        if num_options >= 6 {
            self.reserve_key(Key::D6, action);
        }
        if num_options >= 7 {
            self.reserve_key(Key::D7, action);
        }
        if num_options >= 8 {
            self.reserve_key(Key::D8, action);
        }
        if num_options >= 9 {
            self.reserve_key(Key::D9, action);
        }

        if self.event_consumed {
            return None;
        }

        let num = if let Some(Button::Keyboard(key)) = self.event.press_args() {
            match key {
                Key::D1 => Some(1),
                Key::D2 => Some(2),
                Key::D3 => Some(3),
                Key::D4 => Some(4),
                Key::D5 => Some(5),
                Key::D6 => Some(6),
                Key::D7 => Some(7),
                Key::D8 => Some(8),
                Key::D9 => Some(9),
                _ => None,
            }
        } else {
            None
        };
        match num {
            Some(n) if n <= num_options => {
                self.consume_event();
                Some(n)
            }
            _ => {
                self.important_actions.push(String::from(action));
                None
            }
        }
    }

    pub fn key_pressed(&mut self, key: Key, action: &str) -> bool {
        self.reserve_key(key, action);

        if self.event_consumed {
            return false;
        }

        if let Some(Button::Keyboard(pressed)) = self.event.press_args() {
            if key == pressed {
                self.consume_event();
                return true;
            }
        }
        self.important_actions.push(String::from(action));
        false
    }

    pub fn unimportant_key_pressed(&mut self, key: Key, action: &str) -> bool {
        self.reserve_key(key, action);

        if self.event_consumed {
            return false;
        }

        if let Some(Button::Keyboard(pressed)) = self.event.press_args() {
            if key == pressed {
                self.consume_event();
                return true;
            }
        }
        self.unimportant_actions.push(String::from(action));
        false
    }

    pub fn use_event_directly(&self) -> &Event {
        if self.event_consumed {
            return &self.empty_event;
        }
        &self.event
    }

    pub fn use_event_directly_for_important_action(&mut self, action: &str) -> &Event {
        if self.event_consumed {
            return &self.empty_event;
        }
        self.important_actions.push(String::from(action));
        &self.event
    }

    // Should only be called publicly after using event directly
    pub fn consume_event(&mut self) {
        assert!(!self.event_consumed);
        self.event_consumed = true;
    }

    pub fn get_possible_actions(self) -> Vec<String> {
        // TODO have a way to toggle showing all actions!
        self.important_actions
    }

    fn reserve_key(&mut self, key: Key, action: &str) {
        if let Some(prev_action) = self.reserved_keys.get(&key) {
            println!("both {} and {} read key {:?}", prev_action, action, key);
        }
        self.reserved_keys.insert(key, action.to_string());
    }
}
