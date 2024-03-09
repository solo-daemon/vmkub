use super::key::Key;
use std::fmt;
use std::fmt::{Debug};
use serde::{Deserialize, Serialize};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Storage {
    BucketUpTo20GB,
    Bucket20to40GB,
    Bucket40to60GB,
    Bucket60to80GB,
    Bucket80to100GB,
    Bucket100to120GB,
    Bucket120to140GB,
    Bucket140to160GB,
    Bucket160to180GB,
    Bucket180to200GB,
    Bucket200to220GB,
    Bucket220to240GB,
    Bucket240to260GB,
    Bucket260to280GB,
    Bucket280to300GB,
}
    // Define enum for RAM buckets
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Ram {
        BucketUpTo1GB,
        Bucket1to2GB,
        Bucket2to3GB,
        Bucket3to4GB,
        Bucket4to5GB,
        Bucket5to6GB,
        Bucket6to7GB,
        Bucket7to8GB,
        Bucket8to9GB,
        Bucket9to10GB,
}

// Define enum for Virtual CPU buckets
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum VirtualCpu {
    BucketUpTo2,
    Bucket2to4,
    Bucket4to6,
    Bucket6to8,
    Bucket8to10,
}

// Define enum for ARM Image categories
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ArmImage {
    Arm,
    X86,
    Other,
}


pub enum Attributes {
    Storage,
    Ram,
    Cpu,
    ArmImage,
}

impl Attributes {
    pub fn get_string(&self)->String{
        match self {
            Attributes::ArmImage=>return String::from("arm_image"),
            Attributes::Cpu=> return String::from("virtual_cpu"),
            Attributes::Ram => return String::from("ram"),
            Attributes::Storage => return String::from("storage")
        };
    }
}


//struct to store the query 
// all queries will be assumed to have a greater tahn fit
// for arch_image only ine is selected
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query {
    // storage of the machine of the vm in GBs.
    pub storage: u32,

    // RAM requiremnt in GB
    pub ram: u32,

    // Number of cpu cores
    pub cpu_cores: u32,

    // THe name of the arch image. 
    pub arch_images: u32,
}

// Method to get all Storage enums higher than the given value in descending order
pub fn get_higher_storage_enums(value: u32) -> Vec<Storage> {
    let mut enums: Vec<Storage> = (0..=300)
        .step_by(20)
        .filter_map(|v| {
            if v >= value {
                Some(get_storage_enum(v))
            } else {
                None
            }
        })
        .collect();
    enums.sort_by(|a, b| b.cmp(a));
    enums
}

pub fn get_higher_ram_enums(value: u32) -> Vec<Ram> {
    let mut enums: Vec<Ram> = (0..=10)
        .filter_map(|v| {
            if v >= value {
                Some(get_ram_enum(v))
            } else {
                None
            }
        })
        .collect();
    enums.sort_by(|a, b| b.cmp(a));
    enums
}

pub fn get_higher_virtual_cpu_enums(value: u32) -> Vec<VirtualCpu> {
    let mut enums: Vec<VirtualCpu> = (0..=10)
        .filter_map(|v| {
            if v >= value {
                Some(get_virtual_cpu_enum(v))
            } else {
                None
            }
        })
        .collect();
    enums.sort_by(|a, b| b.cmp(a));
    enums
}

pub fn get_equal_arm_image_enums(value: u32) -> Vec<ArmImage> {
    (0..=2)
        .filter_map(|v| {
            if v == value {
                Some(get_arm_image_enum(v))
            } else {
                None
            }
        })
        .collect()
}

// Function to get the enum corresponding to a given value
pub fn get_storage_enum(value: u32) -> Storage {
    match value {
        0..=20 => Storage::BucketUpTo20GB,
        21..=40 => Storage::Bucket20to40GB,
        41..=60 => Storage::Bucket40to60GB,
        61..=80 => Storage::Bucket60to80GB,
        81..=100 => Storage::Bucket80to100GB,
        101..=120 => Storage::Bucket100to120GB,
        121..=140 => Storage::Bucket120to140GB,
        141..=160 => Storage::Bucket140to160GB,
        161..=180 => Storage::Bucket160to180GB,
        181..=200 => Storage::Bucket180to200GB,
        201..=220 => Storage::Bucket200to220GB,
        221..=240 => Storage::Bucket220to240GB,
        241..=260 => Storage::Bucket240to260GB,
        261..=280 => Storage::Bucket260to280GB,
        281..=300 => Storage::Bucket280to300GB,
        _ => panic!("Invalid value for storage"),
    }
}

pub fn get_ram_enum(value: u32) -> Ram {
    match value {
        0..=1 => Ram::BucketUpTo1GB,
        2..=2 => Ram::Bucket1to2GB,
        3..=3 => Ram::Bucket2to3GB,
        4..=4 => Ram::Bucket3to4GB,
        5..=5 => Ram::Bucket4to5GB,
        6..=6 => Ram::Bucket5to6GB,
        7..=7 => Ram::Bucket6to7GB,
        8..=8 => Ram::Bucket7to8GB,
        9..=9 => Ram::Bucket8to9GB,
        10..=10 => Ram::Bucket9to10GB,
        _ => panic!("Invalid value for RAM"),
    }
}

pub fn get_virtual_cpu_enum(value: u32) -> VirtualCpu {
    match value {
        0..=2 => VirtualCpu::BucketUpTo2,
        3..=4 => VirtualCpu::Bucket2to4,
        5..=6 => VirtualCpu::Bucket4to6,
        7..=8 => VirtualCpu::Bucket6to8,
        9..=10 => VirtualCpu::Bucket8to10,
        _ => panic!("Invalid value for virtual CPU"),
    }
}

pub fn get_arm_image_enum(value : u32) -> ArmImage {
    match value {
        0 => ArmImage::Arm,
        1 => ArmImage::X86,
        2 => ArmImage::Other,
        _ => panic!("Invalid category for ARM Image"),
    }
}


pub fn get_attribute_key(attribute: String, value : u32) -> Key {
    
    let  concatenated =  match attribute.as_str() {
        "virtual_cpu" => {
            let virtual_cpu_value = get_virtual_cpu_enum(value);
            format!("{}:{:?}", attribute, virtual_cpu_value)
        }
        "arm_image" => {
            let arm_image_value = get_arm_image_enum(value);
            format!("{}:{:?}", attribute, arm_image_value)
        }
        "ram" => {
            let ram_value = get_ram_enum(value);
            format!("{}:{:?}", attribute, ram_value)
        }
        "storage" => {
            let storage_value = get_storage_enum(value);
            format!("{}:{:?}", attribute, storage_value)
        }
        _ => format!("{}:{}", attribute, value),
    };

    // println!("{}", concatenated);
    let key = Key::new(concatenated);
    key
}



pub fn get_attribute_key_from_enum(attribute: String,  value: impl Debug) -> Key {

    let conc = format!("{}:{:?}", attribute, value);

    // let  concatenated =  match attribute.as_str() {
    //     "virtual_cpu" => {
    //         let virtual_cpu_value = get_virtual_cpu_enum(value);
    //         format!("{}:{:?}", attribute, EnumToString)
    //     }
    //     "arm_image" => {
    //         let arm_image_value = get_arm_image_enum(value);
    //         format!("{}:{:?}", attribute, arm_image_value)
    //     }
    //     "ram" => {
    //         let ram_value = get_ram_enum(value);
    //         format!("{}:{:?}", attribute, ram_value)
    //     }
    //     "storage" => {
    //         let storage_value = get_storage_enum(value);
    //         format!("{}:{:?}", attribute, storage_value)
    //     }
    //     _ => format!("{}:{}", attribute, value),
    // };

    // println!("{}", concatenated);
    let key = Key::new(conc);
    key
}

// Define the struct with Key and Value fields
#[derive(Clone, Debug, Serialize,  Deserialize)]
pub struct DHTValueStruct {
    pub key: Key,
    pub value: u32,
}
impl fmt::Display for DHTValueStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key: {:?}, Value: {}", self.key, self.value)
    }
}
