#[derive(Debug)]
struct spiderman{
    name:String,
}
#[derive(Debug)]
struct batman{
    name:String,
}
#[derive(Debug)]
struct Hulk{
    name:String,
}
#[derive(Debug)]
struct Ronald{
    name:String,
}

pub trait power{
    fn power_score(&self)->u32;   
}

impl power for spiderman{
    fn power_score(&self)->u32{
        100
    }
}

impl power for batman{
    fn power_score(&self)->u32{
        400
    }
}

impl power for Hulk{
    fn power_score(&self)->u32{
        50
    }
}

impl power for Ronald{
    fn power_score(&self)->u32{
        525
    }
}
 fn main() {
     let power_01 = spiderman{
         name:"Spiderman".to_string(),
     };

      let power_02 = batman{
         name:"Spiderman".to_string(),
     };
      let power_03 = Hulk{
         name:"Spiderman".to_string(),
     };
      let power_04 = Ronald{
         name:"Spiderman".to_string(),
     };

     println!("{:?}",power_01.power_score());
     println!("{:?}",power_02.power_score());
     println!("{:?}",power_03.power_score());
     println!("{:?}",power_04.power_score());
 }
