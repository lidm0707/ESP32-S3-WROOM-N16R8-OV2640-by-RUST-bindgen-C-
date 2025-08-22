use esp_idf_hal::gpio::*;
use esp_idf_sys::{
    camera::{camera_fb_location_t, camera_grab_mode_t, framesize_t, pixformat_t},
    *,
};
use std::marker::PhantomData;

pub struct FrameBuffer<'a> {
    fb: *mut camera::camera_fb_t,
    _p: PhantomData<&'a camera::camera_fb_t>,
}

impl<'a> FrameBuffer<'a> {
    pub fn data(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts((*self.fb).buf, (*self.fb).len) }
    }

    pub fn width(&self) -> usize {
        unsafe { (*self.fb).width }
    }

    pub fn height(&self) -> usize {
        unsafe { (*self.fb).height }
    }

    pub fn format(&self) -> camera::pixformat_t {
        unsafe { (*self.fb).format }
    }

    pub fn timestamp(&self) -> camera::timeval {
        unsafe { (*self.fb).timestamp }
    }

    pub fn fb_return(&self) {
        unsafe { camera::esp_camera_fb_return(self.fb) }
    }
}

impl Drop for FrameBuffer<'_> {
    fn drop(&mut self) {
        self.fb_return();
    }
}

pub struct CameraSensor<'a> {
    sensor: *mut camera::sensor_t,
    _p: PhantomData<&'a camera::sensor_t>,
}

impl<'a> CameraSensor<'a> {
    pub fn init_status(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).init_status.unwrap()(self.sensor) })
    }
    pub fn reset(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).reset.unwrap()(self.sensor) })
    }
    pub fn set_pixformat(&self, format: camera::pixformat_t) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_pixformat.unwrap()(self.sensor, format) })
    }
    pub fn set_framesize(&self, framesize: camera::framesize_t) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_framesize.unwrap()(self.sensor, framesize) })
    }
    pub fn set_contrast(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_contrast.unwrap()(self.sensor, level) })
    }
    pub fn set_brightness(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_brightness.unwrap()(self.sensor, level) })
    }
    pub fn set_saturation(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_saturation.unwrap()(self.sensor, level) })
    }
    pub fn set_sharpness(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_sharpness.unwrap()(self.sensor, level) })
    }
    pub fn set_denoise(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_denoise.unwrap()(self.sensor, level) })
    }
    pub fn set_gainceiling(&self, gainceiling: camera::gainceiling_t) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_gainceiling.unwrap()(self.sensor, gainceiling) })
    }
    pub fn set_quality(&self, quality: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_quality.unwrap()(self.sensor, quality) })
    }
    pub fn set_colorbar(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_colorbar.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_whitebal(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_whitebal.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_gain_ctrl(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_gain_ctrl.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_exposure_ctrl(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_exposure_ctrl.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_hmirror(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_hmirror.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_vflip(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_vflip.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_aec2(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_aec2.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_awb_gain(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_awb_gain.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_agc_gain(&self, gain: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_agc_gain.unwrap()(self.sensor, gain) })
    }
    pub fn set_aec_value(&self, gain: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_aec_value.unwrap()(self.sensor, gain) })
    }
    pub fn set_special_effect(&self, effect: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_special_effect.unwrap()(self.sensor, effect) })
    }
    pub fn set_wb_mode(&self, mode: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_wb_mode.unwrap()(self.sensor, mode) })
    }
    pub fn set_ae_level(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_ae_level.unwrap()(self.sensor, level) })
    }
    pub fn set_dcw(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_dcw.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_bpc(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_bpc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_wpc(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_wpc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_raw_gma(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_raw_gma.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_lenc(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_lenc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn get_reg(&self, reg: i32, mask: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).get_reg.unwrap()(self.sensor, reg, mask) })
    }
    pub fn set_reg(&self, reg: i32, mask: i32, value: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_reg.unwrap()(self.sensor, reg, mask, value) })
    }
    
    #![allow(clippy::too_many_arguments)]
    pub fn set_res_raw(
        &self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        offset_x: i32,
        offset_y: i32,
        total_x: i32,
        total_y: i32,
        output_x: i32,
        output_y: i32,
        scale: bool,
        binning: bool,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_res_raw.unwrap()(
                self.sensor,
                start_x,
                start_y,
                end_x,
                end_y,
                offset_x,
                offset_y,
                total_x,
                total_y,
                output_x,
                output_y,
                scale,
                binning,
            )
        })
    }
    pub fn set_pll(
        &self,
        bypass: i32,
        mul: i32,
        sys: i32,
        root: i32,
        pre: i32,
        seld5: i32,
        pclken: i32,
        pclk: i32,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_pll.unwrap()(
                self.sensor,
                bypass,
                mul,
                sys,
                root,
                pre,
                seld5,
                pclken,
                pclk,
            )
        })
    }
    pub fn set_xclk(&self, timer: i32, xclk: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_xclk.unwrap()(self.sensor, timer, xclk) })
    }
}

pub struct Camera<'a> {
    _pin_pwdn: Option<AnyIOPin>,
    _pin_reset: Option<AnyIOPin>,
    _pin_xclk: AnyIOPin,
    _pin_d0: AnyIOPin,
    _pin_d1: AnyIOPin,
    _pin_d2: AnyIOPin,
    _pin_d3: AnyIOPin,
    _pin_d4: AnyIOPin,
    _pin_d5: AnyIOPin,
    _pin_d6: AnyIOPin,
    _pin_d7: AnyIOPin,
    _pin_vsync: AnyIOPin,
    _pin_href: AnyIOPin,
    _pin_pclk: AnyIOPin,
    _pin_sda: AnyIOPin,
    _pin_scl: AnyIOPin,
    _pins: PhantomData<&'a ()>,
}

impl<'a> Camera<'a> {
    /// Default configuration for your ESP32-S3 board
    #![allow(clippy::too_many_arguments)]
    pub fn default_ov2640() -> Result<Self, EspError> {
        // Working pin configuration for your board

        let pin_pwdn: Option<AnyIOPin> = None; // No PWDN pin
        let pin_reset: Option<AnyIOPin> = None; // No RESET pin
        let pin_sda = unsafe { AnyIOPin::new(4) }; // SIOD
        let pin_scl = unsafe { AnyIOPin::new(5) }; // SIOC
        let pin_d0 = unsafe { AnyIOPin::new(11) };
        let pin_d1 = unsafe { AnyIOPin::new(9) };
        let pin_d2 = unsafe { AnyIOPin::new(8) };
        let pin_d3 = unsafe { AnyIOPin::new(10) };
        let pin_d4 = unsafe { AnyIOPin::new(12) };
        let pin_d5 = unsafe { AnyIOPin::new(18) };
        let pin_d6 = unsafe { AnyIOPin::new(17) };
        let pin_d7 = unsafe { AnyIOPin::new(16) };
        let pin_xclk = unsafe { AnyIOPin::new(15) }; // XCLK
        let pin_vsync = unsafe { AnyIOPin::new(6) };
        let pin_href = unsafe { AnyIOPin::new(7) };
        let pin_pclk = unsafe { AnyIOPin::new(13) };

        // Working camera configuration
        let config = camera::camera_config_t {
            pin_pwdn: -1,  // No PWDN pin
            pin_reset: -1, // No RESET pin
            pin_xclk: pin_xclk.pin(),
            __bindgen_anon_1: camera::camera_config_t__bindgen_ty_1 {
                pin_sccb_sda: pin_sda.pin(), // SIOD
            },
            __bindgen_anon_2: camera::camera_config_t__bindgen_ty_2 {
                pin_sccb_scl: pin_scl.pin(), // SIOC
            },
            pin_d0: pin_d0.pin(),
            pin_d1: pin_d1.pin(),
            pin_d2: pin_d2.pin(),
            pin_d3: pin_d3.pin(),
            pin_d4: pin_d4.pin(),
            pin_d5: pin_d5.pin(),
            pin_d6: pin_d6.pin(),
            pin_d7: pin_d7.pin(),
            pin_vsync: pin_vsync.pin(),
            pin_href: pin_href.pin(),
            pin_pclk: pin_pclk.pin(),
            // Clock settings
            xclk_freq_hz: 20_000_000, // ต้องใส่ ไม่งั้นกล้องไม่ detect
            ledc_channel: 0,          // LEDC_CHANNEL_0
            ledc_timer: 0,            // LEDC_TIMER_0
            // Camera settings
            pixel_format: camera::pixformat_t_PIXFORMAT_JPEG,
            frame_size: camera::framesize_t_FRAMESIZE_QQVGA,
            jpeg_quality: 12,
            fb_count: 1,
            grab_mode: camera::camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY,
            fb_location: camera::camera_fb_location_t_CAMERA_FB_IN_DRAM,
            ..Default::default()
        };

        esp!(unsafe { camera::esp_camera_init(&config) })?;

        Ok(Camera {
            _pin_pwdn: pin_pwdn,
            _pin_reset: pin_reset,
            _pin_xclk: pin_xclk,
            _pin_d0: pin_d0,
            _pin_d1: pin_d1,
            _pin_d2: pin_d2,
            _pin_d3: pin_d3,
            _pin_d4: pin_d4,
            _pin_d5: pin_d5,
            _pin_d6: pin_d6,
            _pin_d7: pin_d7,
            _pin_vsync: pin_vsync,
            _pin_href: pin_href,
            _pin_pclk: pin_pclk,
            _pin_sda: pin_sda,
            _pin_scl: pin_scl,
            _pins: PhantomData,
        })
    }
    #![allow(clippy::too_many_arguments)
    pub fn new(
        pin_pwdn: Option<AnyIOPin>,
        pin_reset: Option<AnyIOPin>,
        pin_xclk: AnyIOPin,
        pin_d0: AnyIOPin,
        pin_d1: AnyIOPin,
        pin_d2: AnyIOPin,
        pin_d3: AnyIOPin,
        pin_d4: AnyIOPin,
        pin_d5: AnyIOPin,
        pin_d6: AnyIOPin,
        pin_d7: AnyIOPin,
        pin_vsync: AnyIOPin,
        pin_href: AnyIOPin,
        pin_pclk: AnyIOPin,
        pin_sda: AnyIOPin,
        pin_scl: AnyIOPin,
        pixel_format: pixformat_t,
        frame_size: framesize_t,
        grab_mode: camera_grab_mode_t,
        fb_location: camera_fb_location_t,
    ) -> Result<Camera<'a>, EspError> {
        let config = camera::camera_config_t {
            pin_pwdn: pin_pwdn.as_ref().map_or(-1, |p: &AnyIOPin| p.pin()),
            pin_reset: pin_reset.as_ref().map_or(-1, |p: &AnyIOPin| p.pin()),
            pin_xclk: pin_xclk.pin(),
            __bindgen_anon_1: camera::camera_config_t__bindgen_ty_1 {
                pin_sccb_sda: pin_sda.pin(),
            },
            __bindgen_anon_2: camera::camera_config_t__bindgen_ty_2 {
                pin_sccb_scl: pin_scl.pin(),
            },
            pin_d0: pin_d0.pin(),
            pin_d1: pin_d1.pin(),
            pin_d2: pin_d2.pin(),
            pin_d3: pin_d3.pin(),
            pin_d4: pin_d4.pin(),
            pin_d5: pin_d5.pin(),
            pin_d6: pin_d6.pin(),
            pin_d7: pin_d7.pin(),
            pin_vsync: pin_vsync.pin(),
            pin_href: pin_href.pin(),
            pin_pclk: pin_pclk.pin(),
            xclk_freq_hz: 20_000_000,
            ledc_channel: 0,
            ledc_timer: 0,
            pixel_format,
            frame_size,
            jpeg_quality: 12,
            fb_count: 1,
            grab_mode,
            fb_location,
            ..Default::default()
        };

        esp!(unsafe { camera::esp_camera_init(&config) })?;

        Ok(Camera {
            _pin_pwdn: pin_pwdn,
            _pin_reset: pin_reset,
            _pin_xclk: pin_xclk,
            _pin_d0: pin_d0,
            _pin_d1: pin_d1,
            _pin_d2: pin_d2,
            _pin_d3: pin_d3,
            _pin_d4: pin_d4,
            _pin_d5: pin_d5,
            _pin_d6: pin_d6,
            _pin_d7: pin_d7,
            _pin_vsync: pin_vsync,
            _pin_href: pin_href,
            _pin_pclk: pin_pclk,
            _pin_sda: pin_sda,
            _pin_scl: pin_scl,
            _pins: PhantomData,
        })
    }

    pub fn get_framebuffer(&self) -> Option<FrameBuffer> {
        let fb = unsafe { camera::esp_camera_fb_get() };
        if fb.is_null() {
            None
        } else {
            Some(FrameBuffer {
                fb,
                _p: PhantomData,
            })
        }
    }

    pub fn sensor(&self) -> Option<CameraSensor<'a>> {
        let sensor = unsafe { camera::esp_camera_sensor_get() };
        if sensor.is_null() {
            None
        } else {
            Some(CameraSensor {
                sensor,
                _p: PhantomData,
            })
        }
    }
}

impl<'a> Drop for Camera<'a> {
    fn drop(&mut self) {
        esp!(unsafe { camera::esp_camera_deinit() }).expect("error during esp_camera_deinit")
    }
}
