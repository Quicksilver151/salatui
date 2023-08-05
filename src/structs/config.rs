
#[derive(Debug, Default)]
pub struct CalculationMethod {
    pub name: String,
    pub location: String,
    pub coordinates: String  //TODO: change to actual coord struct
    
}

#[derive(Debug, Default)]
pub struct TimeSet{
    pub name: String,
    pub location: String,
    pub coordinates: String,
}

#[derive(Debug, Default)]
pub enum Provider {
    #[default]
    Manual,
    Data(TimeSet),
    Calculation(CalculationMethod),
}


#[derive(Debug, Default)]
pub struct Config{
    pub provider: Provider
}















