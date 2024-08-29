use serde::{Serialize, Deserialize};

use super::app_error;

#[derive(Deserialize, Serialize)]
struct AnalyteModel {
    AnalyteId: i32,
    AnalyteName: String,
    LabUnitDictionaryId: i32,
    LabUnitDictionaryName: String,
    AllowSigns: bool
}

impl AnalyteModel {
    pub fn Validate(&self) -> Result<(), app_error::Error> {
        if self.AnalyteId < 100 {
            return Err(app_error::Error::Lulzy());
        }
        Ok(())
    }
}
