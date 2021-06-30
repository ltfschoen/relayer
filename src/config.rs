use std::path::PathBuf;

use serde::Deserialize;

const fn default_port() -> u16 { 9955 }

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WebbRelayerConfig {
    /// WebSocket Server Port number
    ///
    /// default to 9955
    #[serde(default = "default_port")]
    pub port: u16,
    /// Interprets the string in order to generate a key Pair.
    ///
    /// - If `s` is a possibly `0x` prefixed 64-digit hex string, then it will
    ///   be interpreted
    /// directly as a `MiniSecretKey` (aka "seed" in `subkey`).
    /// - If `s` is a valid BIP-39 key phrase of 12, 15, 18, 21 or 24 words,
    ///   then the key will
    /// be derived from it. In this case:
    ///   - the phrase may be followed by one or more items delimited by `/`
    ///     characters.
    ///   - the path may be followed by `///`, in which case everything after
    ///     the `///` is treated
    /// as a password.
    /// - If `s` begins with a `/` character it is prefixed with the Substrate
    ///   public `DEV_PHRASE` and
    /// interpreted as above.
    ///
    /// In this case they are interpreted as HDKD junctions; purely numeric
    /// items are interpreted as integers, non-numeric items as strings.
    /// Junctions prefixed with `/` are interpreted as soft junctions, and
    /// with `//` as hard junctions.
    ///
    /// There is no correspondence mapping between SURI strings and the keys
    /// they represent. Two different non-identical strings can actually
    /// lead to the same secret being derived. Notably, integer junction
    /// indices may be legally prefixed with arbitrary number of zeros.
    /// Similarly an empty password (ending the SURI with `///`) is perfectly
    /// valid and will generally be equivalent to no password at all.
    pub suri: String,
}

pub fn load<P: Into<PathBuf>>(path: P) -> anyhow::Result<WebbRelayerConfig> {
    let base: PathBuf = path.into();
    let mut cfg = config::Config::new();
    cfg.merge(config::File::with_name(&base.display().to_string()))?
        .merge(config::Environment::with_prefix("WEBB"))?;
    cfg.try_into().map_err(Into::into)
}