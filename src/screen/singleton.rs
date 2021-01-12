use lazy_static::lazy_static;
use spin::Mutex;

impl Screen {
    lazy_static! {
        pub static ref WRITER: Mutex<super::Screen> = Mutex::new(super::Screen {
            column_position: 0,
            color_code: super::ColorCode::new(super::Color::Yellow, super::Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut super::Buffer) },
        });
    }
}
