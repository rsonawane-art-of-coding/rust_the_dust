/* Structure Methods */

#[derive(Debug)]
struct Computer {
    brand   : String,
    cpu     : String,
    ram     : u32,
    storage_type : String,
}

impl Computer {
    /* Associated method which does not take self as argument*/
    fn new (brand:String, cpu:String, ram:u32, storage_type:String) -> Self {
        Self {
            brand,
            cpu,
            ram,
            storage_type,
        }
    }

    fn system_info(&self) {
        println!("Brand Name    : {}", self.brand);
        println!("CPU           : {}", self.cpu);
        println!("RAM           : {}", self.ram);
        println!("STORAGE       : {}\n", self.storage_type);
    }
    
}

impl Computer {
    fn update_storage(&mut self, storage_type:String) {
        self.storage_type = storage_type;
    }
}

fn main()
{
    let mut lenovo_computer = Computer::new(
        String::from("lenovo"),
        String::from("Snapdragon XElite 2"),
        16,
        String::from("SSD"),
    );

    lenovo_computer.system_info(); 

    /* Directly modifying structure members */
    lenovo_computer.ram = 64;

    /* Modifying structure using method */
    lenovo_computer.update_storage("UFS SSD".to_string());

    lenovo_computer.system_info(); 
}


