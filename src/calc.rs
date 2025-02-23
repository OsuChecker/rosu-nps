use rosu_map::Beatmap;


/// Returns the total play length of the beatmap in milliseconds
///
/// Calculates the duration between the first and last hit objects in the beatmap.
///
/// # Arguments
///
/// * `map` - A reference to the Beatmap struct containing hit objects
///
/// # Returns
///
/// * `Some(f64)` - The duration in milliseconds if the beatmap contains hit objects
/// * `None` - If the beatmap has no hit objects
pub fn get_play_length(map: &Beatmap) -> Option<f64> {
    let first = map.hit_objects.first()?;
    let last = map.hit_objects.last()?;
    Some(last.start_time - first.start_time)
}

/// Calculates the average number of notes per second (NPS) for the beatmap
///
/// # Arguments
///
/// * `map` - A reference to the Beatmap struct containing hit objects
///
/// # Returns
///
/// * `Some(f64)` - The average NPS if calculation succeeds
/// * `None` - If the beatmap is empty or duration cannot be calculated
pub fn calculate_avg_nps(map: &Beatmap) -> Option<f64>{
    if map.hit_objects.is_empty() { return None }
    let hit = map.hit_objects.len() as f64;
    Some(hit/(get_play_length(map)?/1000f64))
}
/// Converts milliseconds to seconds
///
/// # Arguments
///
/// * `ms` - Time value in milliseconds
///
/// # Returns
///
/// * `f64` - Time value in seconds
pub fn to_sec(ms: f64) -> f64{
    ms/1000.0
}

/// Calculates the note density distribution by dividing the map into equal time blocks
///
/// # Arguments
///
/// * `map` - A reference to the Beatmap struct containing hit objects
/// * `block` - Number of blocks to divide the map into
///
/// # Returns
///
/// * `Some(Vec<f64>)` - Vector containing notes per second for each block
/// * `None` - If block count is invalid (<=0) or map is empty
pub fn calculate_distribution(map: &Beatmap, block: i32) -> Option<Vec<f64>> {
    if block <= 0 { return None }
    if map.hit_objects.is_empty() { return None }

    let play_length = get_play_length(map)?;
    let first_note_time = map.hit_objects.first()?.start_time;
    let actual_duration = play_length ;
    let block_duration = actual_duration / block as f64;
    let block_size = block as usize;
    let mut counts = vec![0usize; block_size];
    let inv_block_duration = 1.0 / block_duration;
    let hit_objects = &map.hit_objects;

    for hit_object in hit_objects {
        let relative_time = hit_object.start_time - first_note_time;
        let index = ((relative_time * inv_block_duration) as usize)
            .min(block_size - 1);
        counts[index] += 1;
    }

    let sec_duration = to_sec(block_duration);

    Some(counts
        .into_iter()
        .map(|count| count as f64 / sec_duration)
        .collect())
}


/// Calculates the note density distribution based on a specified sampling frequency
///
/// # Arguments
///
/// * `map` - A reference to the Beatmap struct containing hit objects
/// * `frequency` - The sampling frequency in Hz
///
/// # Returns
///
/// * `Some(Vec<f64>)` - Vector containing notes per second for each sample period
/// * `None` - If frequency is invalid (<=0) or map is empty
pub fn calculate_by_frequency(map: &Beatmap, frequency: f64) -> Option<Vec<f64>> {
    if frequency <= 0.0 { return None; }
    let samples = (1f64/frequency).round() as i32;
    calculate_distribution(map, samples)
}
#[cfg(test)]
mod tests {
    use super::*;
    use rosu_map::Beatmap;

    fn setup() -> Beatmap {
        let file = "./resources/test.osu";
        Beatmap::from_path(&file)
            .expect("File does not exist or is not a valid .osu file")
    }

    #[test]
    fn test_calculate_distribution() {
        let map = setup();
        let frequency = 4;
        let distribution = calculate_distribution(&map, frequency)
            .expect("Calc distribution failed");
        assert_eq!(distribution.len(), frequency as usize);
        let expected = [18.064172837569114, 12.838660604344815, 17.532071069695096, 19.142020008390833];
        assert_eq!(distribution, expected);

    }

    #[test]
    fn test_calculate_frequency(){
        let map = setup();
        let frequency = 0.25;
        let distribution = calculate_by_frequency(&map, frequency)
            .expect("Calc distribution failed");
        assert_eq!(distribution.len(), 4);
        let expected = [18.10510374279019, 12.825016969271122, 17.532071069695096, 19.11473273824345];
        assert_eq!(distribution, expected);
    }
    #[test]
    fn test_invalid_frequency() {
        let map = setup();
        let result = calculate_by_frequency(&map, 0.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_calculate_avg_nps(){
        let map = setup();
        let avg_nps = calculate_avg_nps(&map)
            .expect("Calc avg nps failed");
        assert_eq!(avg_nps, 16.894231129999966);
    }

    #[test]
    fn test_distribution_sum() {
        let map = setup();
        let frequency = 4;
        let distribution = calculate_distribution(&map, frequency).unwrap();
        let sum: f64 = distribution.iter().sum();
        let avg_nps = calculate_avg_nps(&map).unwrap();
        assert!((sum/frequency as f64 - avg_nps).abs() < 0.1);
    }

    #[test]
    fn test_get_play_length(){
        let map = setup();
        let play_length = get_play_length(&map)
            .expect("Calc play length failed");
        assert_eq!(play_length, 293177.0);
    }

    #[test]
    fn test_frequency_consistency() {
        let map = setup();
        let dist1 = calculate_by_frequency(&map, 0.25).unwrap();
        let dist2 = calculate_distribution(&map, 4).unwrap();
        assert_eq!(dist1, dist2);
    }

    #[test]
    fn test_to_sec_conversion() {
        assert_eq!(to_sec(1000.0), 1.0);
        assert_eq!(to_sec(500.0), 0.5);
        assert_eq!(to_sec(0.0), 0.0);
    }

    #[test]
    fn test_different_block_sizes() {
        let map = setup();
        let distributions = vec![
            calculate_distribution(&map, 2).unwrap(),
            calculate_distribution(&map, 4).unwrap(),
            calculate_distribution(&map, 8).unwrap()
        ];
        assert_eq!(distributions[0].len(), 2);
        assert_eq!(distributions[1].len(), 4);
        assert_eq!(distributions[2].len(), 8);
    }
    #[test]
    fn test_frequency_rounding() {
        let map = setup();
        let freq1 = calculate_by_frequency(&map, 0.33333).unwrap();
        let freq2 = calculate_by_frequency(&map, 0.33334).unwrap();
        assert_eq!(freq1.len(), freq2.len());
    }

}