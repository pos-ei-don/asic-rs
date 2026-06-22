#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(
    feature = "python",
    pyclass(skip_from_py_object, get_all, module = "asic_rs")
)]
#[cfg_attr(feature = "python", asic_rs_pydantic::py_pydantic_model)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Configured thermal limits of the miner (°C).
///
/// Mirrors the Braiins OS layout. Fields are optional because not every
/// firmware reports every threshold (e.g. VNish exposes `minimum` and `danger`
/// but not `target`/`hot`).
pub struct TemperatureConfig {
    /// The target chip temperature the firmware tunes toward.
    pub target: Option<f64>,
    /// The temperature at which the fans run at 100%.
    pub hot: Option<f64>,
    /// The temperature at/above which the miner reboots or triggers thermal scaling.
    pub danger: Option<f64>,
    /// The minimum temperature the miner must preheat to before it will start.
    pub minimum: Option<f64>,
}

impl TemperatureConfig {
    /// Create an empty temperature configuration (all thresholds unset).
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl TemperatureConfig {
    #[new]
    #[pyo3(signature = (target = None, hot = None, danger = None, minimum = None))]
    fn py_new(
        target: Option<f64>,
        hot: Option<f64>,
        danger: Option<f64>,
        minimum: Option<f64>,
    ) -> Self {
        Self {
            target,
            hot,
            danger,
            minimum,
        }
    }
}

#[cfg(feature = "python")]
mod python_impls {
    use asic_rs_pydantic::get_optional_field;
    use pyo3::{Borrowed, PyAny, PyErr, PyResult, conversion::FromPyObject, types::PyAnyMethods};

    use super::TemperatureConfig;

    impl FromPyObject<'_, '_> for TemperatureConfig {
        type Error = PyErr;

        fn extract(obj: Borrowed<'_, '_, PyAny>) -> PyResult<Self> {
            let opt = |name: &str| -> PyResult<Option<f64>> {
                Ok(get_optional_field(&obj, name)?
                    .map(|value| value.extract())
                    .transpose()?
                    .flatten())
            };
            Ok(TemperatureConfig {
                target: opt("target")?,
                hot: opt("hot")?,
                danger: opt("danger")?,
                minimum: opt("minimum")?,
            })
        }
    }
}
