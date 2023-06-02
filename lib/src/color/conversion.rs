use crate::convert::ConvertableFrom;
use super::*;

pub struct LABSettings {
    pub refs: (f32, f32, f32)
}

impl ConvertableFrom<ARGB> for AXYZ {
    type Error = ();
    type Options = ();

    fn try_convert_from(value: ARGB, _: Self::Options) -> Result<Self, Self::Error> {
        fn adj(channel: f32) -> f32 {
            let scaled = channel / 255_f32;
            if scaled > 0.04045 {
                ((scaled + 0.055) / 1.055).powf(2.4)
            }
            else {
                scaled / 12.92
            }
        }

        let var_r: f32 = adj(value.red as f32);
        let var_g: f32 = adj(value.green as f32);
        let var_b: f32 = adj(value.blue as f32);

        Ok(Self {
            x: var_r * 0.4124 + var_g * 0.3576 + var_b * 0.1805,
            y: var_r * 0.2126 + var_g * 0.7152 + var_b * 0.0722,
            z: var_r * 0.0193 + var_g * 0.1192 + var_b * 0.9505,
            alpha: value.alpha
        })
    }
}

impl ConvertableFrom<ARGB> for ALAB {
    type Error = ();
    type Options = LABSettings;

    fn try_convert_from(value: ARGB, options: Self::Options) -> Result<Self, Self::Error> {
        let xyz = AXYZ::try_convert_from(value, ())?;
        Self::try_convert_from(xyz, options)
    }
}

impl ConvertableFrom<ARGB> for AHSV {
    type Error = ();
    type Options = ();

    fn try_convert_from(value: ARGB, _: Self::Options) -> Result<Self, Self::Error> {
        let r = (value.red as f32) / 255_f32;
        let g = (value.green as f32) / 255_f32;
        let b = (value.blue as f32) / 255_f32;

        let min = f32::min(r, f32::min(g, b));
        let max = f32::max(r, f32::max(g, b));
        let delta = max - min;
   
        let alpha = value.alpha;
        let v = max;

        if delta == 0_f32 {
            Ok(AHSV { 
                h: 0_f32, 
                s: 0_f32, 
                v, 
                alpha
            })
        }
        else {
            let s = delta / max;

            let delta_r = (((max - r) / 6_f32) + (delta / 2_f32)) / delta;
            let delta_g = (((max - g) / 6_f32) + (delta / 2_f32)) / delta;
            let delta_b = (((max - b) / 6_f32) + (delta / 2_f32)) / delta;

            let h = if r == max {
                delta_b - delta_g
            }
            else if g == max {
                (1_f32 / 3_f32) + delta_r - delta_b
            }
            else if b == max {
                (2_f32 / 3_f32) + delta_g - delta_r
            }
            else {
                0_f32
            };

            let h_adj = if h < 0_f32 {
                h + 1_f32
            }
            else if h > 0_f32 {
                h - 1_f32
            }
            else {
                h
            };

            Ok(AHSV { 
                h: h_adj, 
                s, 
                v, 
                alpha
            })
        }
    }
}

impl ConvertableFrom<AXYZ> for ARGB {
    type Error = ();
    type Options = ();

    fn try_convert_from(_value: AXYZ, _: Self::Options) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl ConvertableFrom<AXYZ> for ALAB {
    type Error = ();
    type Options = LABSettings;

    fn try_convert_from(value: AXYZ, options: Self::Options) -> Result<Self, Self::Error> {
        fn adj(channel: f32) -> f32 {
            if channel > 0.008856 {
                channel.powf(1_f32 / 3_f32)
            }
            else {
                (7.787 * channel) + (16_f32 / 116_f32)
            }
        }

        let var_x = adj(value.x / options.refs.0);
        let var_y = adj(value.y / options.refs.1);
        let var_z = adj(value.z / options.refs.2);

        Ok(Self {
            l: (116_f32 * var_y) - 16_f32,
            a: 500_f32 * (var_x - var_y),
            b: 200_f32 * (var_y - var_z),
            alpha: value.alpha
        })
    }
}

impl ConvertableFrom<AXYZ> for AHSV {
    type Error = ();
    type Options = ();

    fn try_convert_from(_value: AXYZ, _: Self::Options) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl ConvertableFrom<ALAB> for ARGB {
    type Error = ();
    type Options = LABSettings;

    fn try_convert_from(_value: ALAB, _options: Self::Options) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl ConvertableFrom<ALAB> for AXYZ {
    type Error = ();
    type Options = LABSettings;

    fn try_convert_from(_value: ALAB, _options: Self::Options) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl ConvertableFrom<ALAB> for AHSV {
    type Error = ();
    type Options = LABSettings;

    fn try_convert_from(_value: ALAB, _options: Self::Options) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl ConvertableFrom<AHSV> for ARGB {
    type Error = ();
    type Options = ();

    fn try_convert_from(_value: AHSV, _: Self::Options) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl ConvertableFrom<AHSV> for AXYZ {
    type Error = ();
    type Options = ();

    fn try_convert_from(_value: AHSV, _: Self::Options) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl ConvertableFrom<AHSV> for ALAB {
    type Error = ();
    type Options = LABSettings;

    fn try_convert_from(_value: AHSV, _options: Self::Options) -> Result<Self, Self::Error> {
        todo!();
    }
}