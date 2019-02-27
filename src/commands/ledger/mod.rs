//! Subcommands of the `tmkms` command-line application

use abscissa::{Callable, GlobalConfig};
use std::process;
use crate::config::KmsConfig;
use crate::keyring::KeyRing;
use tendermint::amino_types::vote::{Vote, SignVoteRequest};
use tendermint::amino_types::{SignedMsgType, SignableMsg};

#[derive(Debug, Options)]
pub enum LedgerCommand {
    #[options(help = "initialise the height/round/step")]
    Initialise(InitCommand),
}

impl_command!(LedgerCommand);

impl Callable for LedgerCommand {
    fn call(&self) {
        match self {
            LedgerCommand::Initialise(init) => init.call(),
        }
    }
}

impl LedgerCommand {
    pub(super) fn config_path(&self) -> Option<&str> {
        match self {
            LedgerCommand::Initialise(init) => init.config.as_ref().map(|s| s.as_ref()),
        }
    }
}

#[derive(Debug, Options)]
pub struct InitCommand {
    #[options(short = "c", long = "config")]
    pub config: Option<String>,

    #[options(short = "h", long = "height")]
    pub height: Option<i64>,

    #[options(short = "r", long = "round")]
    pub round: Option<i64>,

    #[options(short = "s", long = "step")]
    pub step: Option<i64>,
}

impl Callable for InitCommand {
    fn call(&self) {
        let config = KmsConfig::get_global();

        KeyRing::load_from_config(&config.providers).unwrap_or_else(|e| {
            status_err!("couldn't load keyring: {}", e);
            process::exit(1);
        });

        let mut vote = Vote::default();
        vote.height = self.height.unwrap();
        vote.round = self.round.unwrap();
        vote.vote_type = SignedMsgType::PreCommit.to_u32();
        println!("{:?}", vote);
        let sign_vote_req = SignVoteRequest { vote: Some(vote) };
        let mut to_sign = vec![];
        sign_vote_req.sign_bytes(config.validator[0].chain_id, &mut to_sign).unwrap();
        

        let _sig = KeyRing::sign(None, &to_sign).unwrap();
        
        println!("Successfully called the init command with height {}, round {}, and step {}", self.height.unwrap(), self.round.unwrap(), self.step.unwrap());
    }
}