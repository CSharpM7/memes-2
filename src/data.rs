#![feature(proc_macro_hygiene)]
use std::{
    io,
    env,
    path::Path,
    convert::TryInto,
    str::FromStr,
    fs,
    thread::{self}
};
use arcropolis_api;
use skyline_web::{Webpage, WebSession};

pub mod gamemode {
    use super::*;
    use lazy_static::lazy_static;
    
    lazy_static! {
        static ref GAMEMODE: RwLock<i32> = RwLock::new(0);
    }
    //pub static mut GAMEMODE: i32 = 0;
    pub const GAMEMODE_HDR: i32 = 1;
    pub const GAMEMODE_ULTS: i32 = 2;
    pub fn is_HDR() -> bool
    {
        return *GAMEMODE.read().unwrap() == GAMEMODE_HDR;
    }
    pub fn is_ULTS() -> bool
    {
        return *GAMEMODE.read().unwrap() == GAMEMODE_ULTS;
    }

    pub fn set_gamemode(){
        let mut hdr_enabled = false;
        let hdr_folder = "sd:/ultimate/mods/hdr";
        let hdr_folderAssets = "sd:/ultimate/mods/hdr-assets";
        let hdr_folderStage = "sd:/ultimate/mods/hdr-stages";
        if Path::new(hdr_folder).exists(){
            hdr_enabled = hdr_enabled || arcropolis_api::is_mod_enabled(arcropolis_api::hash40(hdr_folder).as_u64())
        }
        if Path::new(hdr_folderAssets).exists(){
            hdr_enabled = hdr_enabled || arcropolis_api::is_mod_enabled(arcropolis_api::hash40(hdr_folderAssets).as_u64())
        }
        if Path::new(hdr_folderStage).exists(){
            hdr_enabled = hdr_enabled || arcropolis_api::is_mod_enabled(arcropolis_api::hash40(hdr_folderStage).as_u64())
        }
        println!("[smashline2_flower::data] HDR: {}",hdr_enabled);

        let mut ultS_enabled = false;
        let ultS_folder = "sd:/ultimate/mods/Ultimate S Arcropolis";
        let ultS_folderStage = "sd:/ultimate/mods/Ultimate S Stages";
        if Path::new(ultS_folder).exists(){
            ultS_enabled = ultS_enabled || arcropolis_api::is_mod_enabled(arcropolis_api::hash40(ultS_folder).as_u64())
        }
        if Path::new(ultS_folderStage).exists(){
            ultS_enabled = ultS_enabled || arcropolis_api::is_mod_enabled(arcropolis_api::hash40(ultS_folderStage).as_u64())
        }
        println!("[smashline2_flower::data] Ult-S: {}",ultS_enabled);
        {
            if hdr_enabled{
                *GAMEMODE.write().unwrap() = GAMEMODE_HDR;
            }
            else if ultS_enabled{
                *GAMEMODE.write().unwrap() = GAMEMODE_ULTS;
            }
        }

    }
}

use std::sync::RwLock;
lazy_static! {
    static ref MOD_DIR: RwLock<String> = RwLock::new("".to_string());
}

pub fn patch_files()
{
    unsafe {
        let motionFolder = format!("{}/fighter/edge/motion/body",&*MOD_DIR.read().unwrap());
        if !Path::new(motionFolder.as_str()).exists()
        {
            return;
        }
        let modFolder =  format!("{}",&*MOD_DIR.read().unwrap());

        super::singleslot::install(modFolder.as_str());
    }
    
}

const IDENTIFIER: &str = "smashline_flower";
pub fn inital_setup()->bool {
    let mut found_folder = false;

    unsafe {
        for entry in std::fs::read_dir("sd:/ultimate/mods").unwrap() {
            let entry = entry.unwrap();
            let mut path = entry.path();
            if path.is_dir() {
                path.push(IDENTIFIER);
                if Path::new(&path).exists() {
                    found_folder = true;
                    path.pop();
                    *MOD_DIR.write().unwrap() = format!("{}", path.display());
                    break;
                }
            }
        }
    }
    return found_folder;
}


pub fn install() {
    if inital_setup() {
        let install_thread = std::thread::spawn(move || {
            patch_files();
        });
        install_thread.join();
    }
    else {
        println!("[smashline2_flower::data] Aborting");
        skyline_web::DialogOk::ok("A critical file was deleted from this moveset! Exiting");
        unsafe {skyline::nn::oe::ExitApplication();}
    }
}