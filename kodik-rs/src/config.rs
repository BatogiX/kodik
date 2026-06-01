use anyhow::Result;
use clap::{
    ArgAction, Parser, ValueEnum,
    builder::styling::{self, Styles},
};
use kodik_parser::TranslationType;
use kodik_utils::load_netscape_cookies;
use log::LevelFilter;
use reqwest::Url;

const STYLES: Styles = Styles::styled()
    .header(styling::AnsiColor::BrightGreen.on_default().bold())
    .usage(styling::AnsiColor::BrightGreen.on_default().bold())
    .literal(styling::AnsiColor::BrightCyan.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(name = "kodik", styles = STYLES, arg_required_else_help = true, about = "CLI tool for getting direct links to files from Kodik")]
pub struct Config {
    /// Url(s) to parse
    #[arg(value_name = "URL", required = true)]
    pub urls: Vec<Url>,

    /// Use verbose output (-vv very verbose)
    #[arg(short = 'v', long, action = ArgAction::Count, default_value = "0")]
    pub verbose: u8,

    /// Do not print log messages
    #[arg(short = 's', long, conflicts_with = "verbose")]
    pub silent: bool,

    /// Specify video quality
    #[arg(short = 'q', long, value_name = "QUALITY", default_value = "720")]
    pub quality: Quality,

    /// Netscape formatted file to read cookies from
    #[arg(long, value_name = "FILE")]
    pub cookies: Option<String>,

    /// Specify translation title
    #[arg(long, value_name = "TITLE")]
    pub translation_title: Option<String>,

    /// Specify translation type
    #[arg(long, value_name = "TYPE")]
    pub translation_type: Option<TranslationTypeArg>,

    /// Expand a media database URL into all related URLs
    #[arg(long, value_name = "MODE")]
    pub related_mode: Option<RelatedMode>,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Self, clap::Error> {
        Self::try_parse_from(args)
    }

    pub const fn level_filter(&self) -> LevelFilter {
        if self.silent {
            LevelFilter::Off
        } else {
            match self.verbose {
                0 => LevelFilter::Info,
                1 => LevelFilter::Debug,
                _ => LevelFilter::Trace,
            }
        }
    }

    pub fn load_cookies(&self) -> Result<reqwest::cookie::Jar> {
        Ok(match &self.cookies {
            Some(path) => load_netscape_cookies(path)?,
            None => reqwest::cookie::Jar::default(),
        })
    }
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
#[non_exhaustive]
pub enum Quality {
    #[value(name = "360")]
    P360 = 360,
    #[value(name = "480")]
    P480 = 480,
    #[default]
    #[value(name = "720")]
    P720 = 720,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[non_exhaustive]
pub enum TranslationTypeArg {
    Voice,
    Subtitles,
}

impl From<TranslationTypeArg> for TranslationType {
    fn from(arg: TranslationTypeArg) -> Self {
        match arg {
            TranslationTypeArg::Voice => Self::Voice,
            TranslationTypeArg::Subtitles => Self::Subtitles,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[non_exhaustive]
pub enum RelatedMode {
    #[value(name = "all")]
    All,
    #[value(name = "essential")]
    Essential,
}
