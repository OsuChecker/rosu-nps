use rosu_map::Beatmap;

// Constantes pour améliorer la lisibilité
const MS_TO_SEC: f64 = 1000.0;

pub fn calc_nps(map: &Beatmap) -> Option<f64> {
    let drain_time_ms = map.hit_objects.last()?.start_time - map.hit_objects.first()?.start_time;
    if drain_time_ms <= 0.0 {
        return Some(map.hit_objects.len() as f64);
    }
    Some(map.hit_objects.len() as f64 / (drain_time_ms / MS_TO_SEC))
}

pub fn calc_nps_range_by_time(map: &Beatmap, start_time: f64, end_time: f64) -> Option<f64> {
    let drain_time_ms = end_time - start_time;
    if drain_time_ms <= 0.0 {
        return Some(0.0);
    }
    
    let start_idx = map.hit_objects.partition_point(|h| h.start_time < start_time);
    let end_idx = map.hit_objects.partition_point(|h| h.start_time <= end_time);
    
    let count = end_idx.saturating_sub(start_idx);
    Some(count as f64 / (drain_time_ms / MS_TO_SEC))
}

pub fn calc_nps_range_by_hitobjects(
    map: &Beatmap, 
    start_obj: &rosu_map::section::hit_objects::HitObject, 
    end_obj: &rosu_map::section::hit_objects::HitObject
) -> Option<f64> {
    let drain_time_ms = end_obj.start_time - start_obj.start_time;
    if drain_time_ms <= 0.0 {
        return Some(0.0);
    }
    
    // Calculate indices using pointer arithmetic - O(1) - PS : VIVA C
    let slice_ptr = map.hit_objects.as_ptr();
    let start_ptr = start_obj as *const _ as *const rosu_map::section::hit_objects::HitObject;
    let end_ptr = end_obj as *const _ as *const rosu_map::section::hit_objects::HitObject;
    
    if start_ptr < slice_ptr || end_ptr < slice_ptr {
        return None;
    }
    
    let start_idx = unsafe { start_ptr.offset_from(slice_ptr) as usize };
    let end_idx = unsafe { end_ptr.offset_from(slice_ptr) as usize };
    
    if start_idx >= map.hit_objects.len() || end_idx >= map.hit_objects.len() {
        return None;
    }
    
    let (start_idx, end_idx) = if start_idx <= end_idx {
        (start_idx, end_idx + 1)
    } else {
        (end_idx, start_idx + 1)
    };
    
    let count = end_idx - start_idx;
    Some(count as f64 / (drain_time_ms / MS_TO_SEC))
}

pub fn calc_distribution(map: &Beatmap, t_parts: i32) -> Option<Vec<f64>> {
    if t_parts <= 0 || map.hit_objects.is_empty() {
        return None;
    }

    let first_time = map.hit_objects.first()?.start_time;
    let last_time = map.hit_objects.last()?.start_time;
    let total_duration_ms = last_time - first_time;
    
    if total_duration_ms <= 0.0 {
        return Some(vec![0.0; t_parts as usize]);
    }

    let part_duration_ms = total_duration_ms / t_parts as f64;
    let part_duration_sec = part_duration_ms / MS_TO_SEC;
    let mut distribution = vec![0.0; t_parts as usize];
    
    for part in 0..t_parts {
        let part_start_time = first_time + part as f64 * part_duration_ms;
        let part_end_time = first_time + (part + 1) as f64 * part_duration_ms;
        
        // Utiliser < pour start_time et <= pour end_time pour correspondre à calculate_distribution_old
        let start_idx = map.hit_objects.partition_point(|h| h.start_time < part_start_time);
        
        // Pour la dernière partie, inclure la dernière note avec <=
        let end_idx = if part == t_parts - 1 {
            map.hit_objects.partition_point(|h| h.start_time <= part_end_time)
        } else {
            map.hit_objects.partition_point(|h| h.start_time < part_end_time)
        };
        
        let count = end_idx.saturating_sub(start_idx);
        distribution[part as usize] = count as f64 / part_duration_sec;
    }
    
    Some(distribution)
}

pub fn to_sec(ms: f64) -> f64 {
    ms / MS_TO_SEC
}

pub fn calc_distribution_2(map: &Beatmap, t_parts: i32) -> Option<Vec<f64>> {
    if t_parts <= 0 || map.hit_objects.is_empty() {
        return None;
    }

    let first_time = map.hit_objects.first()?.start_time;
    let last_time = map.hit_objects.last()?.start_time;
    let total_duration_ms = last_time - first_time;
    let part_duration_ms = total_duration_ms / t_parts as f64;
    let part_size = t_parts as usize;
    let mut counts = vec![0usize; part_size];
    let inv_part_duration = 1.0 / part_duration_ms;
    
    for hit_object in &map.hit_objects {
        let relative_time = hit_object.start_time - first_time;
        let index = ((relative_time * inv_part_duration) as usize).min(part_size - 1);
        counts[index] += 1;
    }

    let part_duration_sec = to_sec(part_duration_ms);

    Some(
        counts
            .into_iter()
            .map(|count| count as f64 / part_duration_sec)
            .collect(),
    )
}

pub fn calc_distribution_smart(map: &Beatmap, t_parts: i32) -> Option<Vec<f64>> {
    if t_parts <= 0 || map.hit_objects.is_empty() {
        return None;
    }
    
    let n_objects = map.hit_objects.len();
    
    // Heuristique : si t_parts * log2(n_objects) > n_objects, utiliser l'algorithme old
    // En pratique, on peut simplifier avec un seuil empirique
    let threshold = (n_objects as f64).sqrt() as i32;
    
    if t_parts > threshold {
        // Pour beaucoup de parties, l'algorithme old est plus efficace
        calc_distribution_2(map, t_parts)
    } else {
        // Pour peu de parties, l'algorithme new avec binary search est plus efficace
        calc_distribution(map, t_parts)
    }
}