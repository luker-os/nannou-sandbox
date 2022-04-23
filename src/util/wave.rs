use nannou::prelude::FloatConst;

#[derive(Debug)]
pub struct Sin {
    pub amplitude: f32,
    pub shift: f32,
    pub period: f32,
    pub offset: f32,
}

impl Sin {
    pub fn get(&self, x: f32) -> f32 {
        ((x - self.shift) * f32::PI() / (self.period / 2f32)).sin() * self.amplitude / 2f32
            + self.offset
    }

    pub fn set_amp_range(self, min: f32, max: f32) -> Self {
        let amplitude = max - min;
        let offset = (max + min) / 2f32;

        Self {
            amplitude,
            offset,
            ..self
        }
    }

    pub fn with_range(shift: f32, period: f32, min: f32, max: f32) -> Self {
        Self {
            shift,
            period,
            ..Self::default().set_amp_range(min, max)
        }
    }
}

impl Default for Sin {
    fn default() -> Self {
        Self {
            amplitude: 2f32,
            shift: 0f32,
            period: f32::PI(),
            offset: 0f32,
        }
    }
}

// public Sin range(double min, double max) {
//     amplitude = max - min;
//     offset = (max + min) / 2;
//     return this;
//   }
