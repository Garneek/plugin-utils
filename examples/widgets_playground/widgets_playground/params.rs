use nih_plug::{
    formatters,
    params::{BoolParam, FloatParam, Params},
    prelude::{FloatRange, SmoothingStyle},
    util,
};

#[derive(Params)]
pub struct EguiUtilsExampleParams {
    #[id = "percent"]
    pub percent: FloatParam,

    #[id = "db"]
    pub db: FloatParam,

    #[id = "nr"]
    pub nr: FloatParam,

    #[id = "bool"]
    pub bool: BoolParam,
}

impl Default for EguiUtilsExampleParams {
    fn default() -> Self {
        Self {
            percent: FloatParam::new(
                "Percentage",
                1_f32,
                FloatRange::Linear {
                    min: 0_f32,
                    max: 1_f32,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50_f32))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(1))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            db: FloatParam::new(
                "Db",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            nr: FloatParam::new(
                "NumberFloat",
                1_f32,
                FloatRange::Linear {
                    min: 0_f32,
                    max: 32_f32,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50_f32)),

            bool: BoolParam::new("Boolean", false),
        }
    }
}
