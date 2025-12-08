use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,)]
pub struct SaveData {
    pub highscore: f32,
}
impl std::fmt::Display for SaveData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Highscore: {}", self.highscore)
    }
}
pub fn load_save(path: &str) -> SaveData {
    let data = std::fs::read_to_string(path)
        .unwrap_or("{\"highscore\":0}".to_string());

    serde_json::from_str(&data).unwrap()
}

pub fn save(path: &str, save_data: &SaveData) {
    let json = serde_json::to_string(save_data).unwrap();
    std::fs::write(path, json).unwrap();
}