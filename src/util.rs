use crate::error::Error;
use anyhow::Result;
use log::{debug, error};
use std::path::Path;

pub fn get_full_image(image_name: &str, image_tag: &str) -> String {
    join_by_colon(image_name, image_tag)
}

pub fn canonicalize_str(target: &str) -> Result<String> {
    match Path::new(target).canonicalize()?.to_str() {
        Some(s) => Ok(s.to_string()),
        None => {
            debug!(
                "returning back given value for canonicalization: {}",
                target
            );
            Ok(target.to_string())
        }
    }
}

pub fn join_by_colon(before: &str, after: &str) -> String {
    format!("{}:{}", before, after)
}

pub fn get_tag_from_image(image: &str) -> Result<(String, String), Error> {
    match image.rsplit_once(":") {
        Some((name, tag)) => Ok((name.to_string(), tag.to_string())),
        None => Err(Error::OCIImageSplitFailure(image.to_string())),
    }
}

pub fn get_first_n_chars(s: String, n: usize) -> String {
    match s.len() {
        length if length <= n => {
            debug!("returning back given value since length <= {}: {}", n, s);
            s
        }
        _ => s[..n].to_string(),
    }
}

pub fn log_bollard_error(e: &bollard::errors::Error) {
    error!("error from Docker daemon: {}", e);
}
