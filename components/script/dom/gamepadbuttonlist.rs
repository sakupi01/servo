/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;

use crate::dom::bindings::codegen::Bindings::GamepadButtonListBinding::GamepadButtonListMethods;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object};
use crate::dom::bindings::root::{Dom, DomRoot, DomSlice};
use crate::dom::gamepadbutton::GamepadButton;
use crate::dom::window::Window;
use crate::script_runtime::CanGc;

// https://w3c.github.io/gamepad/#gamepadbutton-interface
#[dom_struct]
pub(crate) struct GamepadButtonList {
    reflector_: Reflector,
    list: Vec<Dom<GamepadButton>>,
}

impl GamepadButtonList {
    #[cfg_attr(crown, allow(crown::unrooted_must_root))]
    fn new_inherited(list: &[&GamepadButton]) -> GamepadButtonList {
        GamepadButtonList {
            reflector_: Reflector::new(),
            list: list.iter().map(|button| Dom::from_ref(*button)).collect(),
        }
    }

    pub(crate) fn new(
        window: &Window,
        list: &[&GamepadButton],
        can_gc: CanGc,
    ) -> DomRoot<GamepadButtonList> {
        reflect_dom_object(
            Box::new(GamepadButtonList::new_inherited(list)),
            window,
            can_gc,
        )
    }
}

impl GamepadButtonListMethods<crate::DomTypeHolder> for GamepadButtonList {
    // https://w3c.github.io/gamepad/#dom-gamepad-buttons
    fn Length(&self) -> u32 {
        self.list.len() as u32
    }

    // https://w3c.github.io/gamepad/#dom-gamepad-buttons
    fn Item(&self, index: u32) -> Option<DomRoot<GamepadButton>> {
        self.list
            .get(index as usize)
            .map(|button| DomRoot::from_ref(&**button))
    }

    // https://w3c.github.io/gamepad/#dom-gamepad-buttons
    fn IndexedGetter(&self, index: u32) -> Option<DomRoot<GamepadButton>> {
        self.Item(index)
    }
}

impl GamepadButtonList {
    /// Initialize the number of buttons in the "standard" gamepad mapping.
    /// <https://www.w3.org/TR/gamepad/#dfn-initializing-buttons>
    pub(crate) fn init_buttons(window: &Window, can_gc: CanGc) -> DomRoot<GamepadButtonList> {
        let standard_buttons = &[
            GamepadButton::new(window, false, false, can_gc), // Bottom button in right cluster
            GamepadButton::new(window, false, false, can_gc), // Right button in right cluster
            GamepadButton::new(window, false, false, can_gc), // Left button in right cluster
            GamepadButton::new(window, false, false, can_gc), // Top button in right cluster
            GamepadButton::new(window, false, false, can_gc), // Top left front button
            GamepadButton::new(window, false, false, can_gc), // Top right front button
            GamepadButton::new(window, false, false, can_gc), // Bottom left front button
            GamepadButton::new(window, false, false, can_gc), // Bottom right front button
            GamepadButton::new(window, false, false, can_gc), // Left button in center cluster
            GamepadButton::new(window, false, false, can_gc), // Right button in center cluster
            GamepadButton::new(window, false, false, can_gc), // Left stick pressed button
            GamepadButton::new(window, false, false, can_gc), // Right stick pressed button
            GamepadButton::new(window, false, false, can_gc), // Top button in left cluster
            GamepadButton::new(window, false, false, can_gc), // Bottom button in left cluster
            GamepadButton::new(window, false, false, can_gc), // Left button in left cluster
            GamepadButton::new(window, false, false, can_gc), // Right button in left cluster
            GamepadButton::new(window, false, false, can_gc), // Center button in center cluster
        ];
        rooted_vec!(let buttons <- standard_buttons.iter().map(DomRoot::as_traced));
        Self::new(window, buttons.r(), can_gc)
    }
}
