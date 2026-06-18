use std::net::IpAddr;

use async_trait::async_trait;

use crate::{
    errors::ModelSelectionError,
    traits::{discovery::DiscoveryCommands, model::MinerModel},
};

#[async_trait]
pub trait MinerFirmware: ToString + DiscoveryCommands {
    async fn get_model(ip: IpAddr) -> Result<impl MinerModel, ModelSelectionError>;
    async fn get_version(ip: IpAddr) -> Option<semver::Version>;
    /// Whether this firmware reports per-board chip temperatures.
    ///
    /// Defaults to false; firmwares that surface chip temps (e.g. VNish) override this.
    fn reports_chip_temperature(&self) -> bool {
        false
    }
}
