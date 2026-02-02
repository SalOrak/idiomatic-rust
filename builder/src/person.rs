
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Person {
    name: String,
    family_name: String,
    age: u8,
    phone: Option<u64>,
    home_address: Option<String>,
    job_title: Option<String>,
    education: Option<String>,
    residency: Option<String>,
    nationality: Option<String>,
}

#[allow(dead_code)]
impl Person {
    pub fn new(
        name: String, 
        family_name: String, 
        age: u8,
    ) -> Self {
        Self {
            name,
            family_name,
            age,
            .. Default::default()
        }
    }

    pub fn with_phone(self, phone:u64) -> Self { Self { phone: Some(phone), ..self } }
    pub fn with_home_address(self, home_address : String) -> Self { Self {home_address: Some(home_address), ..self}}
    pub fn with_job_title(self, job_title: String) -> Self { Self {job_title : Some(job_title), ..self} }
    pub fn with_education(self, education: String) -> Self { Self {education : Some(education), ..self} }
    pub fn with_residency(self, residency: String) -> Self { Self { residency : Some(residency), ..self } }
    pub fn with_nationality(self, nationality: String) -> Self { Self {nationality : Some(nationality), ..self}  }
}
