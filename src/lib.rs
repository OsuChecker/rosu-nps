use pyo3::prelude::*;
use reqwest::blocking;
use serde::Serialize;
use std::error::Error;
use pyo3::exceptions::PyValueError;

/// Struct representing a key-value pair where the `key` is a timestamp (in milliseconds),
/// and the `value` is the average notes per second (NPS).
///
/// # Example
///
/// ```rust
/// let key_value = KeyValue { key: 1000, value: 3.5 };
/// println!("Key: {}, Value: {}", key_value.key, key_value.value);
/// ```
#[derive(Serialize)]
pub struct KeyValue {
    pub key: i32,
    pub value: f64,
}

/// Downloads the content of a file from the provided URL.
///
/// # Arguments
///
/// - `url`: The URL to download the content from.
///
/// # Returns
///
/// - `Ok(String)`: The content of the file as a string if the request is successful.
/// - `Err(Box<dyn Error>)`: An error if the download fails or the status code is not successful.
///
/// # Example
///
/// ```rust
/// let url = "https://example.com/test.osu";
/// match download_file(url) {
///     Ok(content) => println!("File content: {}", content),
///     Err(e) => println!("Error downloading file: {}", e),
/// }
/// ```
pub fn download_file(url: &str) -> Result<String, Box<dyn Error>> {
    let response = blocking::get(url)?;
    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err(format!("HTTP Error: {}", response.status()).into())
    }
}

/// Parses a single line of an `.osu` file to extract a "hit object" timestamp.
///
/// # Arguments
///
/// - `line`: A single line of text from the `.osu` file.
///
/// # Returns
///
/// - `Ok(i32)`: A hit object timestamp in milliseconds if successful.
/// - `Err(PyValueError)`: A Python exception if parsing fails.
///
/// # Example
///
/// ```rust
/// let line = "100,200,1500";
/// match read_note(line) {
///     Ok(hit) => println!("Hit object timestamp: {}", hit),
///     Err(e) => println!("Error parsing hit object: {}", e),
/// }
/// ```
pub fn read_note(line: &str) -> PyResult<i32> {
    let mut parts = line.split(',');
    if let (Some(_), Some(_), Some(hit_object)) = (parts.next(), parts.next(), parts.next()) {
        hit_object
            .parse::<i32>()
            .map_err(|e| PyValueError::new_err(format!("Error parsing hit object ({}): {}", hit_object, e)))
    } else {
        Err(PyValueError::new_err(
            "Error 17001: Invalid .osu file (Cannot read hitObject)",
        ))
    }
}

/// Parses the "hit objects" section from an `.osu` file into a vector of timestamps (in milliseconds).
///
/// # Arguments
///
/// - `curl_content`: The full content of an `.osu` file as a string.
///
/// # Returns
///
/// - `Ok(Vec<i32>)`: A vector of timings of hit objects if successful.
/// - `Err(String)`: An error message if parsing failed.
///
/// # Example
///
/// ```rust
/// let content = "\
/// [HitObjects]
/// 100,200,1500
/// 300,400,1800
/// ";
///
/// match parse_hit_objects(content) {
///     Ok(timings) => println!("Timings: {:?}", timings),
///     Err(e) => println!("Error parsing content: {}", e),
/// }
/// ```
pub fn parse_hit_objects(curl_content: &str) -> Result<Vec<i32>, String> {
    let mut hit_objects = false;
    curl_content
        .lines()
        .filter_map(|line| {
            if !hit_objects {
                if line == "[HitObjects]" {
                    hit_objects = true;
                }
                None
            } else {
                match read_note(line) {
                    Ok(note) => Some(Ok(note)),
                    Err(_) => None
                }
            }
        })
        .collect::<Result<Vec<i32>, String>>()
}

/// Calculates the average notes per second from a vector of hit timings.
///
/// # Arguments
///
/// - `timings`: A vector of hit object timestamps (in milliseconds).
/// - `frequency`: The frequency (number of intervals per second) for calculating the NPS.
///
/// # Returns
///
/// - `Ok(Vec<KeyValue>)`: A vector of results where each `KeyValue` represents an NPS calculation at a time interval.
/// - `Err(String)`: An error message if input is invalid.
///
/// # Example
///
/// ```rust
/// let timings = vec![1000, 1500, 2000, 2500];
/// let frequency = 1.0; // 1 interval per second.
///
/// match calculate_average_notes_per_second(&timings, frequency) {
///     Ok(results) => {
///         for result in results {
///             println!("Key: {}, Value: {}", result.key, result.value);
///         }
///     }
///     Err(e) => println!("Error calculating NPS: {}", e),
/// }
/// ```
pub fn calculate_average_notes_per_second(timings: &[i32], frequency: f64) -> Result<Vec<KeyValue>, String> {
    if timings.is_empty() {
        return Err("The timings vector is empty.".to_string());
    }

    let start_time = *timings.first().unwrap();
    let end_time = *timings.last().unwrap();
    let t_duration: i32 = end_time - start_time;
    let interval_ms: f64 = (t_duration as f64 * frequency) / 100.0_f64;

    if interval_ms <= 0.0 {
        return Err("Invalid duration or frequency is too high.".to_string());
    }

    let mut result = Vec::new();
    let mut interval_start = start_time;
    let mut current_note_index: usize = 0;

    while interval_start < end_time {
        let interval_end = interval_start + interval_ms as i32;
        let interval_end_index = timings.partition_point(|&x| x < interval_end);
        let note_count = interval_end_index - current_note_index;

        let interval_seconds = interval_ms / 1000.0_f64;
        let nps = note_count as f64 / interval_seconds;

        result.push(KeyValue {
            key: interval_start,
            value: nps,
        });

        interval_start = interval_end;
        current_note_index = interval_end_index;
    }

    Ok(result)
}

/// Python-facing function to calculate the average notes per second from a given `.osu` file URL.
///
/// # Arguments
///
/// - `url`: The URL of the `.osu` file.
/// - `frequency`: The frequency (in intervals per second) for calculating average notes per second.
///
/// # Returns
///
/// - `Ok(String)`: A JSON string with the results in the format `[{"key": ..., "value": ...}]`.
/// - `Err(PyValueError)`: A Python exception if downloading, parsing, or calculations fail.
///
/// # Python Example:
///
/// ```python
/// from untitled2 import get_nps
///
/// url = "https://example.com/sample.osu"
/// try:
///     result = get_nps(url, 2.0)
///     print(result)
/// except ValueError as e:
///     print(f"Error: {e}")
/// ```
#[pyfunction]
pub fn get_nps(url: &str, frequency: f64) -> PyResult<String> {
    let curl_content = download_file(url).map_err(|e| PyValueError::new_err(format!("Failed to download file: {}", e)))?;
    let parsed_hit_objects = parse_hit_objects(&curl_content)
        .map_err(|e| PyValueError::new_err(format!("Error parsing .osu file: {}", e)))?;
    let nps_result = calculate_average_notes_per_second(&parsed_hit_objects, frequency)
        .map_err(|e| PyValueError::new_err(format!("Error calculating NPS: {}", e)))?;
    let json_output = serde_json::to_string(&nps_result)
        .map_err(|e| PyValueError::new_err(format!("JSON Serialization Error: {}", e)))?;
    Ok(json_output)
}

/// Python module definition for the `untitled2` crate.
///
/// This module exposes the `get_nps` function to Python.
#[pymodule]
pub fn nps(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_nps, m)?)?;
    Ok(())
}