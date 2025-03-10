use rchorus_component::Component;

use conformal_vst_wrapper::{ClassID, ClassInfo, EffectClass, HostInfo, Info};

const CID: ClassID = [
    0x36, 0x7f, 0xdb, 0xcb, 0xeb, 0xb8, 0x6f, 0x8f, 0x06, 0x45, 0x77, 0x91, 0xe6, 0x68, 0xcc, 0x85,
];
const EDIT_CONTROLLER_CID: ClassID = [
    0x93, 0x5e, 0x78, 0xce, 0x21, 0xd1, 0x9c, 0xba, 0x98, 0x48, 0xdf, 0x40, 0xfa, 0xf7, 0xd3, 0x4d,
];

conformal_vst_wrapper::wrap_factory!(
    &const {
        [&EffectClass {
            info: ClassInfo {
                name: "Chorus-R",
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
        url: "http://github.com/russellmcc/bilinear-audio",
        email: "test@example.com",
        version: "1.0.0",
    }
);
