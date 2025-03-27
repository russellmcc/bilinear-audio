use jx_alpha_component::Component;

use conformal_vst_wrapper::{ClassID, ClassInfo, HostInfo, Info, SynthClass};

const CID: ClassID = [
    0x93, 0x25, 0x34, 0xb4, 0x08, 0x5f, 0x49, 0xa9, 0xa9, 0x1d, 0x95, 0x0e, 0xcc, 0x90, 0x29, 0x1c,
];
const EDIT_CONTROLLER_CID: ClassID = [
    0x22, 0x64, 0xf3, 0x8e, 0x92, 0x11, 0x4b, 0xe9, 0x8f, 0xd1, 0xcd, 0x13, 0x86, 0xe2, 0xca, 0x2a,
];

conformal_vst_wrapper::wrap_factory!(
    &const {
        [&SynthClass {
            info: ClassInfo {
                name: "Alpha JX",
                cid: CID,
                edit_controller_cid: EDIT_CONTROLLER_CID,
                ui_initial_size: conformal_vst_wrapper::UiSize {
                    width: 400,
                    height: 400,
                },
            },
            factory: |_: &HostInfo| -> Component { Default::default() },
        }]
    },
    Info {
        vendor: "Bilinear Audio",
        url: "http://github.com/russellmcc/bilinear-audio",
        email: "test@example.com",
        version: "1.0.0",
    }
);
