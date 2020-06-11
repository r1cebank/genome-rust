mod marker;
mod mutation;
use std::str;
use marker::Marker;
use rand::prelude::*;
use mutation::MutationType;

pub struct Gene {
    pub num_markers: u16,
    pub markers: Vec<Marker>,
}

/// First marker is influence
impl Gene {
    pub fn new(num_markers: u16) -> Gene {
        if num_markers < 1 {
            panic!("Markers needs to be more than 0");
        }
        Gene {
            num_markers: num_markers,
            markers: (0..num_markers + 1).map(|_| marker::Marker::new()).collect()
        }
    }
    pub fn is_equal(left_gene: Gene, right_gene: Gene) -> bool {
        if left_gene.markers.len() != right_gene.markers.len() {
            return false;
        }
        let left_markers = left_gene.get_markers();
        let right_markers = right_gene.get_markers();
        let mut result = true;

        for (i, marker) in left_markers.iter().enumerate() {
            if *marker != right_markers[i] {
                result = false;
            }
        }

        return result;
    }
    pub fn get_influence(&self) -> f32 {
        self.markers[0].value
    }
    pub fn get_marker(&self, position: usize) -> Option<f32> {
        // position + 1 is used instead of position since first element is influence
        match self.markers.get(position + 1) {
            Some(m) => Some(m.value),
            None => None
        }
    }
    pub fn get_markers(&self) -> Vec<f32> {
        self.markers
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != 0) // Filter out the first index
            .map(|(_, m)| m.value)
            .collect()
    }
    pub fn to_string(&self) -> String {
        self.markers.iter().map(|m| String::from(*m)).collect::<String>()
    }
    fn set_marker(&mut self, target: usize, value: f32) {
        self.markers.get_mut(target).unwrap().value = value;
    }
    pub fn mutate(&mut self) {
        let mutation_type = mutation::get_mutation_type();
        let target = thread_rng().gen_range(0, self.markers.len());
        match mutation_type {
            MutationType::DELETE => { self.set_marker(target, 0 as f32)}
            MutationType::DUPLICATION => {
                // exclude the influence marker, allow noop
                let dup_target = thread_rng().gen_range(1, self.markers.len());
                self.set_marker(dup_target, self.markers[target].value);
            }
            MutationType::NEW => {
                let new_marker = Marker::new();
                self.set_marker(target, new_marker.value);
            }
            MutationType::REVERSAL => {
                // exclude the influence marker, allow noop
                let swap_target = thread_rng().gen_range(1, self.markers.len());
                let swap_value = self.markers[swap_target].value;
                self.set_marker(swap_target, self.markers[target].value);
                self.set_marker(target, swap_value);
            }
            MutationType::SHIFT => {
                self.markers.shuffle(&mut thread_rng());
            }
        }
    }
}

impl std::convert::From<Gene> for String {
    fn from(gene: Gene) -> String {
        gene.markers.iter().map(|m| String::from(*m)).collect::<String>()
    }
}


impl std::convert::From<String> for Gene {
    fn from(gene: String) -> Gene {
        let markers = gene.as_bytes()
            .chunks(8)
            .map(|x| str::from_utf8(x).unwrap())
            .map(String::from)
            .collect::<Vec<String>>();

        Gene {
            num_markers: markers.len() as u16,
            markers: markers.iter().map(|m| Marker::from(m.clone())).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_be_converted_and_back() {
        let gene = Gene::new(2);
        let original_influence = gene.get_influence();
        let string_value: String = gene.into();
        let restored_gene: Gene = string_value.clone().into();
        let restored_influence = restored_gene.get_influence();
        let restored_string_value: String = restored_gene.into();
        assert_eq!(restored_string_value, string_value);
        assert_eq!(original_influence, restored_influence);
    }
    #[test]
    fn marker_size_match() {
        let gene = Gene::new(2);
        let markers = gene.get_markers();
        assert_eq!(gene.num_markers, markers.len() as u16);
    }
    #[test]
    fn markers_should_include_influence() {
        let gene = Gene::new(2);
        assert_eq!(gene.markers.len(), 3);
    }
    #[test]
    fn get_marker_correctly() {
        let gene = Gene::new(2);
        assert_eq!(gene.get_marker(0).unwrap(), gene.markers[1].value);
    }
    #[test]
    fn get_marker_return_none() {
        let gene = Gene::new(2);
        match gene.get_marker(3) {
            Some(_) => assert!(false),
            None => assert!(true)
        }
    }
    #[test]
    fn should_return_true_on_equal() {
        let gene = Gene::new(2);
        let string_value: String = gene.into();
        let original_gene: Gene = string_value.clone().into();
        let restored_gene: Gene = string_value.clone().into();

        assert!(Gene::is_equal(original_gene, restored_gene));
    }
}
