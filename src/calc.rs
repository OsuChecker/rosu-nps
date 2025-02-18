use rosu_map::Beatmap;
use rosu_map::section::hit_objects::HitObject;

pub fn get_play_length(map: &Beatmap) -> Option<f64>{
    Some(&map.hit_objects.last()?.start_time-&map.hit_objects.first()?.start_time)
}

pub fn calculate_avg_nps(map: &Beatmap) -> Option<f64>{
    let hit = map.hit_objects.len() as f64;
    Some(hit/get_play_length(map)?)
}

pub fn to_sec(ms: f64) -> f64{
    ms/1000.0
}
pub fn calculate_distribution(map: &Beatmap, block: i32) -> Option<Vec<f64>> {
    let play_length = get_play_length(map)?;
    let block_duration = play_length / block as f64;
    let mut blocks = vec![Vec::new(); block as usize];

    for hit_object in &map.hit_objects {
        let index = ((hit_object.start_time / block_duration) as usize)
            .min(block as usize - 1);
        blocks[index].push(hit_object);
    }
    Some(blocks
        .iter()
        .map(|block| (block.len() as f64 / to_sec(block_duration)))
        .collect())
}

fn get_frequency(map: &Beatmap, frequency: f64) -> Option<f64>{
    Some((get_play_length(map)?/frequency))
}
pub fn calculate_by_frequency(map: &Beatmap, frequency: f64) -> Option<Vec<f64>> {
    Some(calculate_distribution(map, get_frequency(map,frequency)? as i32 )?)
}
#[cfg(test)]
mod tests {
    use super::*;
    use rosu_map::Beatmap;

    #[test]
    fn test_calculate_distribution() {
        let file = "./resources/test.osu";
        let map = Beatmap::from_path(&file)
            .expect("File does not exist or is not a valid .osu file");
        let frequency = 4;
        let distribution = calculate_distribution(&map, frequency)
            .expect("Calc distribution failed");
        assert_eq!(distribution.len(), frequency as usize);
        let expected = [18.064172837569114, 12.838660604344815, 17.532071069695096, 19.142020008390833];
        assert_eq!(distribution, expected);
        let total_hits = map.hit_objects.len() as f64;
        let sum_distribution: f64 = distribution.iter().sum();

    }
}