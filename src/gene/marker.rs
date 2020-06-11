use std::str;
use rand::prelude::*;
use arrayvec::ArrayVec;
use rand_distr::StandardNormal;

#[derive(Copy, Clone)]
pub struct Marker {
    pub value: f32,
}

impl Marker {
    pub fn new() -> Marker {
        Marker {
            value: thread_rng().sample(StandardNormal)
        }
    }
    pub fn to_string(&self) -> String {
        let byte_marker = self.value.to_be_bytes();

        byte_marker.iter()
            .map(|val| format!("{:0>2x}", val))
            .collect()
    }
}

impl std::convert::From<Marker> for String {
    fn from(marker: Marker) -> String {
        let byte_marker = marker.value.to_be_bytes();

        byte_marker.iter()
            .map(|val| format!("{:0>2x}", val))
            .collect()
    }
}

impl std::convert::From<f32> for Marker {
    fn from(marker: f32) -> Marker {
        Marker {
            value: marker
        }
    }
}

impl std::convert::From<String> for Marker {
    fn from(marker: String) -> Marker {
        let sub_chunks = marker.as_bytes()
            .chunks(2)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        let decoded: ArrayVec<_> = sub_chunks.iter().map(|c| u8::from_str_radix(c, 16).unwrap()).collect::<ArrayVec<_>>();
        let decoded_array: [u8; 4] = decoded.into_inner().unwrap();

        Marker {
            value: f32::from_be_bytes(decoded_array)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_be_converted_and_back() {
        let marker = Marker::new();
        let string_value: String = marker.into();
        let restored_marker: Marker = string_value.into();
        assert_eq!(marker.value, restored_marker.value);
    }
}