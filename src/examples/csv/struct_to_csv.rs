#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CsvData {
    pub name: String,
    pub money: u64,
    pub phone: String,
    
}
impl CsvData {

    pub fn write_struct_csv(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {

        let mut wtr = csv::Writer::from_path(file_path)?;

        wtr.serialize((&self.name, &self.money, &self.phone))?; // serialize is used when your data consists of more complex values than just Strings
        wtr.flush()?;

        Ok(())
    }
}