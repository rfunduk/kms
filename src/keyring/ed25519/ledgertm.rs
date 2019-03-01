//! Ledger Tendermint signer

use signatory::PublicKeyed;
use signatory_ledger_tm::{self, Ed25519LedgerTmAppSigner};

use crate::{
    config::provider::ledgertm::LedgerTendermintConfig,
    error::{KmsError, KmsErrorKind::*},
    keyring::{ed25519::Signer, KeyRing},
};

pub const LEDGER_TM_PROVIDER_LABEL: &str = "ledgertm";
pub const LEDGER_TM_ID: &str = "ledgertm";

/// Create Ledger Tendermint signer object from the given configuration
pub fn init(
    keyring: &mut KeyRing,
    ledgertm_configs: &[LedgerTendermintConfig],
) -> Result<(), KmsError> {
    println!("ledgertm init");
    if ledgertm_configs.is_empty() {
        println!("config empty");
        return Ok(());
    }

    if ledgertm_configs.len() != 1 {
        fail!(
            ConfigError,
            "expected one [providers.ledgertm] in config, found: {}",
            ledgertm_configs.len()
        );
    }
    let provider = Box::new(Ed25519LedgerTmAppSigner::connect()?);
    println!("provider");
    let pk = provider.public_key()?;
    println!("pukbey");
    // TODO: key_id shouldn't be a constant here (see LEDGER_TM_ID):
    let signer = Signer::new(LEDGER_TM_PROVIDER_LABEL, LEDGER_TM_ID.to_string(), provider);
    println!("signer");
    keyring.add(pk, signer)?;
    println!("added to keyring");
    Ok(())
}
