use crate::convert::ConvertableFrom;
use super::*;

pub struct LABSettings {
    refs: (f32, f32, f32)
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

    fn try_convert_from(_value: ARGB, _: Self::Options) -> Result<Self, Self::Error> {
        todo!();
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