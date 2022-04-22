use serde::{Deserialize, Serialize};
use std::fmt;

use std::error::Error;
use std::str::FromStr;

use clap::{Parser, Subcommand, ArgGroup};

#[derive(Parser, Debug, Clone)]
#[clap(author, version = "0.1.0-alpha", about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub task: Tasks,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Tasks {
    /// edit pdf file, pdf file must be patched first, using a non-patched pdf will result in
    /// undefined behavior
    #[clap(group(ArgGroup::new("oprs").required(true).args(&[ "operations", "opr-file"])))]
    Edit {
        /// page size, required for correct font size rendering
        #[clap(short, default_value = "A4")]
        page_size: PageSize,
        /// run `leafedit list operations` for a list of supported operations
        #[clap(short)]
        operations: Vec<Opr>,

        /// use file of operations
        #[clap(short = 'f')]
        opr_file: Option<String>,

        /// /path/to/file
        #[clap(name = "INPUT")]
        file: String,

        /// /out/file/path
        #[clap(name = "OUTPUT")]
        save_as: String,
    },

    /// apply a set of operations that are necessary before any edits,
    /// like embeding fonts and reseting content tranformation matrix
    Patch {
        /// page to patch
        #[clap(short, default_value = "1")]
        page: u32,
        /// /path/to/file
        #[clap(name = "INPUT")]
        file: String,
        /// /out/file/path
        #[clap(name = "OUTPUT")]
        save_as: String,
    },

    /// list options for different commands
    List {
        #[clap(subcommand)]
        list: ListOptions,
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListOptions {
    /// list supported page sizes
    PageSize,
    /// list supported options
    Operations
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PageSize {
    Word,
    A4,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Opr {
    #[serde(rename = "Wr")]
    WriteLine (u32, u32, u32, String),

    #[serde(rename = "Cc")]
    ChangeColor (Color),

    #[serde(rename = "Crgb")]
    ChangeRgb(u8, u8, u8),

    #[serde(rename = "Lw")]
    SetWidth (u32),

    #[serde(rename = "Dl")]
    DrawLine (u32, u32, u32, u32),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Color {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "white")]
    White,
}

impl Error for ParseOperationError {}
impl Error for PageSizeNotUndersoodError {}

impl fmt::Display for ParseOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid operation, run `leafedit list operations` \
               to get list of supported operations")
    }
}

impl fmt::Display for PageSizeNotUndersoodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Page Size not supported, `leafedit list pagesizes` \
               to get list of supported sizes")
    }
}

#[derive(Debug)]
pub struct ParseOperationError;

impl FromStr for Opr {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ron::from_str(s) {
            Ok(opr) => Ok(opr),
            Err(_) => Err(ParseOperationError),
        }
    }

}

#[derive(Debug)]
pub struct PageSizeNotUndersoodError;
impl FromStr for PageSize {
    type Err = PageSizeNotUndersoodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ron::from_str(s) {
            Ok(opr) => Ok(opr),
            Err(_) => Err(PageSizeNotUndersoodError),
        }
    }

}
