use jpeg_decoder as jpeg;

pub struct MjpegDecoder {}
pub struct YuyvDecoder {}

/// Decodes a camera frame into RGBA bytes in place
pub trait Decoder {
    fn decode(&self, out: &mut [u8], buf: &[u8]);
}

impl Decoder for MjpegDecoder {
    fn decode(&self, out: &mut [u8], buf: &[u8]) {
        let decoded = jpeg::Decoder::new(buf)
            .decode()
            .expect("failed to decode JPEG");
        for (px, rgb) in out.chunks_exact_mut(4).zip(decoded.chunks_exact(3)) {
            px.copy_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
        }
    }
}

// BT.601, limited range, integer math
fn yuv2rgba(px: &mut [u8], y: u8, u: i32, v: i32) {
    let c = 298 * (y as i32 - 16) + 128;
    let clamp = |x: i32| (x >> 8).clamp(0, 255) as u8;
    px.copy_from_slice(&[
        clamp(c + 409 * v),
        clamp(c - 100 * u - 208 * v),
        clamp(c + 516 * u),
        255,
    ]);
}

impl Decoder for YuyvDecoder {
    fn decode(&self, out: &mut [u8], buf: &[u8]) {
        for (px, yuyv) in out.chunks_exact_mut(8).zip(buf.chunks_exact(4)) {
            let (u, v) = (yuyv[1] as i32 - 128, yuyv[3] as i32 - 128);
            yuv2rgba(&mut px[..4], yuyv[0], u, v);
            yuv2rgba(&mut px[4..], yuyv[2], u, v);
        }
    }
}
