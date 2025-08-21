# Adapted from https://github.com/Kezii/esp32cam_rs


argo run --example test_camera --release
cargo build --example test_camera

# how to use

esp-cam = { git = "https://github.com/lidm0707/ESP32-S3-WROOM-N16R8-OV2640-by-RUST-bindgen-C-", branch = "main" }
