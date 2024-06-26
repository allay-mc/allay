//                                    :
//                                 ::::::::
//                             :::::::::::::::
//                         :::::::::::::::::::::::
//                      ::::::::::::::::::::::::::::::
//                  :-:-::-::::::::::::::::::::::::::::::
//              =-----::-::::::::::::::::::::::::::::::::::::
//             ---------:::::::::::::::::::::::::::::::::::::::*
//             ------:-:-::::::::::::::::::::::::::::::::::+****
//             -------:-:-::::::::::::::::::::::::::::::********
//             ----------:::::::::-::::::::-::::::::************
//             =--------------::-:::-:--:-:-:-:-=***************
//             ======----------------------:-*******************
//             =========-----------------***********************
//             ==============----------*************************
//             ================--------*************************
//             ====:::-============----*************************
//             ====:::::==============-*************************
//             ====:::::=====:=========*************************
//             ====:::::=====::::======*************************
//             ====:::::=====:::::=====*************************
//             +===:::::=====:::::=====*************************
//             =++==::::=====:::::=====*************************
//             ++++====:=====:::::=====*************************
//             +=++==========:::::=====***********************#
//                +============:::=====***********************+*=
//                ++++=================*********************+**+====
//                ==++++**========+=++=**********************+**====
//               ====+++**+++=====+++++******************  ***+*====
//              =======***+++++**=+=+=+*****+=************ ***********
//             =======*****++++*****+++**#=====*********** ******+*************
//             ======******+**+*+*+******#*====*******************+*******+*+*+********+
//             =====+********************#*+++++***************************+***+*+*+*+*=====
//               ===***   **+*+**********##*++++********************     ***+***+*+*+**====+
//                           ************###+++++*******************     *********  *+*=====
//                           ************####====************  *****     *********
//                           ************#%##=====***********   ****     *********
//                           ********    #   #====************  ****          ****
//                           ********        #=====***********  ****          *********
//                           ****            # ====************               *********
//                           ****              =====********                  *********
//                                              ====*****                         *****
//                                              ==                                *****
//                                                                                *****

mod cli;
mod config;
mod diagnostic;
mod error;
mod filter;
mod health;
mod init;
mod localization;
mod manifest;
mod pack;
mod paths;
mod plugin;
mod project;
mod scaffolding;
mod uuid;

use crate::init::init;
use config::Config;
use error::Error;
use health::Health;
use manifest::Manifest;
use pack::Pack;
use project::Project;
use std::process::ExitCode;

fn main() -> ExitCode {
    init();
    // TODO: delete old logs
    let matches = cli::cmd().get_matches();
    cli::run(&matches)
}
