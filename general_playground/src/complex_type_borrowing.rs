pub struct Manager
{
    factories: Vec<Factory>,
    powerplants: Vec<Powerplant>
}

impl Manager
{
    pub fn new() -> Manager
    {
        Manager
        {
            factories: Vec::new(),
            powerplants: Vec::new(),
        }
    }

    pub fn add_factory(&mut self, fact: Factory)
    {
        self.factories.push(fact);
    }

    pub fn add_plant(&mut self, plant: Powerplant)
    {
        self.powerplants.push(plant);
    }

    pub fn readonly_factory_list(&self) -> &Vec<Factory>
    {
        &self.factories
    }

    pub fn readonly_plant_list(&self) -> &Vec<Powerplant>
    {
        &self.powerplants
    }

}

#[derive(Debug)]
pub struct Factory
{
    is_powered: bool
}

impl Factory
{
    pub fn new() -> Factory
    {
        Factory
        {
            is_powered: false
        }
    }
}

#[derive(Debug)]
pub struct Powerplant
{
    producing_power: bool
}

impl Powerplant
{
    pub fn new() -> Powerplant
    {
        Powerplant
        {
            producing_power: false
        }
    }
}
