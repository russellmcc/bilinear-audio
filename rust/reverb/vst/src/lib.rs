#![warn(
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    clippy::pedantic,
    clippy::todo
)]
#![allow(
    clippy::type_complexity,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::default_trait_access
)]

use reverb_component::Component;

use conformal_vst_wrapper::{ClassID, ClassInfo, EffectClass, HostInfo, Info};

const CID: ClassID = [
    0x02, 0xec, 0x71, 0x35, 0x3a, 0x08, 0x47, 0x26, 0x97, 0xb9, 0xad, 0xad, 0xaf, 0x40, 0x1c, 0xb8,
];
const EDIT_CONTROLLER_CID: ClassID = [
    0xbe, 0xc3, 0xdc, 0xbf, 0xe0, 0x33, 0x46, 0xa4, 0xaa, 0x57, 0xb2, 0xdc, 0xa5, 0xe2, 0xac, 0xe0,
];

conformal_vst_wrapper::wrap_factory!(
    &const {
        [&EffectClass {
            info: ClassInfo {
                name: "Reverb",
                cid: CID,
                edit_controller_cid: EDIT_CONTROLLER_CID,
                ui_initial_size: conformal_vst_wrapper::UiSize {
                    width: 400,
                    height: 400,
                },
            },
            factory: |_: &HostInfo| -> Component { Default::default() },
            category: "Fx",
            bypass_id: "bypass",
        }]
    },
    Info {
        vendor: "Bilinear Audio",
        url: "TODO add URL",
        email: "test@example.com",
        version: "1.0.0",
    }
);
