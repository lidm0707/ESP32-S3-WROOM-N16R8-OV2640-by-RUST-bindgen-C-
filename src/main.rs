use esp_cam::espcam::Camera;
use esp_idf_hal::prelude::Peripherals;

fn main() {
    // ต้องเรียกก่อนเพื่อ patch runtime
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    let _peripherals = Peripherals::take().unwrap();

    // ✅ init กล้องแบบ fix mapping
    let camera = Camera::new_camera(
    )
    .expect("camera init failed");

    loop {
        if let Some(framebuffer) = camera.get_framebuffer() {
            log::info!("Got framebuffer!");
            log::info!("width: {}", framebuffer.width());
            log::info!("height: {}", framebuffer.height());
            log::info!("len: {}", framebuffer.data().len());
            log::info!("format: {}", framebuffer.format());

            std::thread::sleep(std::time::Duration::from_millis(4000));
        } else {
            log::info!("no framebuffer");
        }
    }
}
