use conformal_vst_wrapper::{ClassID, ClassInfo, HostInfo, Info, SynthClass};
use p61_component::Component;

const CID: ClassID = [
    0xae, 0x09, 0xa8, 0xe7, 0x8b, 0xd0, 0x91, 0x46, 0x93, 0xcd, 0x78, 0x62, 0x51, 0xfa, 0xef, 0xa4,
];
const EDIT_CONTROLLER_CID: ClassID = [
    0xAE, 0x37, 0x9A, 0xCD, 0x2C, 0xC1, 0x00, 0x63, 0x16, 0xE0, 0x86, 0x60, 0x88, 0x36, 0x66, 0xF7,
];

conformal_vst_wrapper::wrap_factory!(
    &const {
        [&SynthClass {
            info: ClassInfo {
                name: "Poly 81",
                cid: CID,
                edit_controller_cid: EDIT_CONTROLLER_CID,
                ui_initial_size: conformal_vst_wrapper::UiSize {
                    width: 800,
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
