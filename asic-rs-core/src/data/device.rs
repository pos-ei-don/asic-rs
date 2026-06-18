#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display as StrumDisplay, EnumString};

use crate::traits::{firmware::MinerFirmware, model::MinerModel};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyclass(from_py_object, module = "asic_rs"))]
#[cfg_attr(feature = "python", asic_rs_pydantic::py_pydantic_model(getters))]
/// Static identity and hardware information for a miner model.
pub struct DeviceInfo {
    /// Miner manufacturer or make.
    pub make: String,
    /// Miner model name.
    pub model: String,
    /// Expected hardware shape.
    pub hardware: MinerHardware,
    /// Firmware name or family.
    pub firmware: String,
    /// Mining hash algorithm.
    pub algo: HashAlgorithm,
    /// The cooling method used by this miner (air, hydro, immersion).
    ///
    /// Derived from the model and constant for a given device, so it can be used
    /// to decide whether environment/fluid-temperature telemetry applies.
    #[serde(default)]
    #[cfg_attr(feature = "python", pydantic(default = CoolingType::Air))]
    pub cooling: CoolingType,
    /// Whether this miner's firmware reports per-board chip temperatures.
    ///
    /// Firmware-dependent (e.g. VNish reports chip temps, stock/Braiins do not),
    /// constant for a given device. Lets consumers decide whether per-board
    /// chip-temperature telemetry applies without inspecting a (possibly transient)
    /// live value.
    #[serde(default)]
    #[cfg_attr(feature = "python", pydantic(default = false))]
    pub reports_chip_temperature: bool,
}

impl DeviceInfo {
    /// Build device information from a model and firmware implementation.
    pub fn new(model: impl MinerModel, firmware: impl MinerFirmware, algo: HashAlgorithm) -> Self {
        Self {
            hardware: model.clone().into(),
            cooling: model.cooling(),
            reports_chip_temperature: firmware.reports_chip_temperature(),
            make: model.make_name(),
            model: model.to_string(),
            firmware: firmware.to_string(),
            algo,
        }
    }
}

#[cfg_attr(feature = "python", pyclass(from_py_object, module = "asic_rs"))]
#[cfg_attr(feature = "python", asic_rs_pydantic::py_pydantic_model(getters))]
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Default)]
/// Expected hardware counts for a miner model.
pub struct MinerHardware {
    /// Expected number of fans.
    pub fans: Option<u8>,
    /// Expected hashboards, represented as the expected number of chips per board.
    pub boards: Option<Vec<Option<u16>>>,
}

impl MinerHardware {
    /// Expected number of hashboards.
    pub fn board_count(&self) -> Option<u8> {
        self.boards
            .as_ref()
            .and_then(|boards| u8::try_from(boards.len()).ok())
    }

    /// Expected total chip count across all hashboards.
    pub fn total_chips(&self) -> Option<u16> {
        self.boards
            .as_ref()
            .map(|boards| boards.iter().copied().flatten().sum())
    }

    /// Expected chip count for a specific hashboard position.
    pub fn chips_for_board(&self, position: usize) -> Option<u16> {
        self.boards
            .as_ref()
            .and_then(|boards| boards.get(position).copied().flatten())
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl MinerHardware {
    #[getter]
    pub fn chips(&self) -> Option<u16> {
        self.total_chips()
    }

    #[getter]
    #[pyo3(name = "board_count")]
    pub fn py_board_count(&self) -> Option<u8> {
        self.board_count()
    }
}

#[cfg_attr(feature = "python", pyclass(from_py_object, module = "asic_rs"))]
#[cfg_attr(feature = "python", derive(asic_rs_pydantic::PyPydanticEnum))]
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize, StrumDisplay, EnumString,
)]
/// Mining hash algorithm.
pub enum HashAlgorithm {
    /// SHA-256 mining.
    #[cfg_attr(feature = "python", pydantic(value = "SHA256"))]
    #[serde(rename = "SHA256")]
    SHA256,
    /// Scrypt mining.
    #[cfg_attr(feature = "python", pydantic(value = "Scrypt"))]
    #[serde(rename = "Scrypt")]
    Scrypt,
    /// X11 mining.
    #[cfg_attr(feature = "python", pydantic(value = "X11"))]
    #[serde(rename = "X11")]
    X11,
    /// Blake2S256 mining.
    #[cfg_attr(feature = "python", pydantic(value = "Blake2S256"))]
    #[serde(rename = "Blake2S256")]
    Blake2S256,
    /// Kadena mining.
    #[cfg_attr(feature = "python", pydantic(value = "Kadena"))]
    #[serde(rename = "Kadena")]
    Kadena,
}

#[cfg_attr(feature = "python", pymethods)]
impl HashAlgorithm {
    pub fn __repr__(&self) -> String {
        self.to_string()
    }

    pub fn __str__(&self) -> String {
        self.to_string()
    }
}

#[cfg_attr(feature = "python", pyclass(from_py_object, str, module = "asic_rs"))]
#[cfg_attr(feature = "python", derive(asic_rs_pydantic::PyPydanticEnum))]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    StrumDisplay,
    EnumString,
    Default,
)]
/// The cooling method used by a miner model.
pub enum CoolingType {
    /// Air cooling (default).
    #[cfg_attr(feature = "python", pydantic(value = "Air"))]
    #[serde(rename = "Air")]
    #[default]
    Air,
    /// Hydro / water cooling.
    #[cfg_attr(feature = "python", pydantic(value = "Hydro"))]
    #[serde(rename = "Hydro")]
    Hydro,
    /// Immersion cooling.
    #[cfg_attr(feature = "python", pydantic(value = "Immersion"))]
    #[serde(rename = "Immersion")]
    Immersion,
}

#[cfg_attr(feature = "python", pymethods)]
impl CoolingType {
    pub fn __repr__(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooling_type_defaults_to_air() {
        assert_eq!(CoolingType::default(), CoolingType::Air);
    }

    #[test]
    fn device_info_defaults_caps_when_absent_in_json() {
        // Older payloads (pre-capability) omit `cooling` / `reports_chip_temperature`;
        // they must deserialize to the conservative defaults rather than failing.
        let json = r#"{
            "make": "Antminer",
            "model": "S19",
            "hardware": {"fans": 4, "boards": [76, 76, 76]},
            "firmware": "Stock",
            "algo": "SHA256"
        }"#;

        let info: DeviceInfo = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(info.cooling, CoolingType::Air);
        assert!(!info.reports_chip_temperature);
    }

    #[test]
    fn device_info_roundtrips_caps() {
        let json = r#"{
            "make": "Antminer",
            "model": "S19 Pro Hydro",
            "hardware": {"fans": 0, "boards": [110, 110, 110]},
            "firmware": "VNish",
            "algo": "SHA256",
            "cooling": "Hydro",
            "reports_chip_temperature": true
        }"#;

        let info: DeviceInfo = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(info.cooling, CoolingType::Hydro);
        assert!(info.reports_chip_temperature);
    }
}
