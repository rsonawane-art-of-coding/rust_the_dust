/* Structure Implementation */

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
    
    fn upgrade_brand(&mut self, brand:String) -> &mut Self {
        self.brand = brand;
        self
    }

    fn upgrade_ram(&mut self, ram:u32) -> &mut Self {
        self.ram = ram;
        self
    }
    
    fn upgrade_cpu(&mut self, cpu:String) -> &mut Self {
        self.cpu = cpu;
        self
    }

    fn upgrade_storage(&mut self, storage:String) -> &mut Self {
        self.storage_type = storage;
        self
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
    lenovo_computer.upgrade_ram(32);
    lenovo_computer.system_info();
    
    let mut apple_computer = Computer::new(
        String::from("Apple"),
        String::from("M4"),
        16,
        String::from("USF"),
    );

    apple_computer.system_info();

    /* Builders Pattern Example */
    let custom_build_computer: &Computer = apple_computer
        .upgrade_brand("Custom Build".to_string())
        .upgrade_cpu("AMD64".to_string())
        .upgrade_ram(64)
        .upgrade_storage("SSD + HDD".to_string());

    custom_build_computer.system_info();

    apple_computer.system_info();
}


