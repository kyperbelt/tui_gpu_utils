use std::process::Command;

pub struct SnapshotData {
    pub graphics_current: i32,
    pub graphics_max: i32,
    pub memory_current: i32,
    pub memory_max: i32,
    pub temperature: i32,
    pub fan_speed: i32
}

impl Default for SnapshotData {
    fn default() -> SnapshotData {
        SnapshotData {
            graphics_current: 0,
            graphics_max: 0,
            memory_current: 0,
            memory_max: 0,
            temperature: 0,
            fan_speed: 0
        }
    }
}

pub struct GpuInfo{
    pub name : String,
    pub driver : String,
    pub bios : String
}

pub fn get_gpu_info()->Option<GpuInfo>{
    let result = Command::new("nvidia-smi")
        .arg("--query-gpu=name,driver_version,vbios_version")
        .arg("--format=csv,noheader,nounits")
        .output();

    if let Ok(output) = result{

        let raw_string = String::from_utf8_lossy(&output.stdout);

        let values : Vec<&str> = raw_string.split(",").collect();
        Some(
            GpuInfo{
                name: values[0].to_owned().trim().to_string(),
                driver: values[1].to_owned().trim().to_string(),
                bios: values[2].to_owned().trim().to_string(),
            }
        )

    }else{
        None
    }
}


pub fn get_clock_data() -> Option<SnapshotData> {
    let result = Command::new("nvidia-smi")
        .arg("--query-gpu=clocks.current.graphics,clocks.max.graphics,clocks.current.memory,clocks.max.memory,temperature.gpu,fan.speed")
        .arg("--format=csv,noheader,nounits")
        .output();

    if let Ok(output) = result{

        let mut data = SnapshotData{..Default::default()};

        let raw_string = String::from_utf8_lossy(&output.stdout);

        // let raw_values : Vec<&str> = raw_string.split(",").collect();

        let values : Vec<i32> = raw_string.split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect();

        data.graphics_current = values[0];
        data.graphics_max     = values[1];
        data.memory_current   = values[2];
        data.memory_max       = values[3];
        data.temperature      = values[4];
        data.fan_speed        = values[5];

        Some(data)
        // let captured_stdout = String::from_utf8_lossy(&output.stdout);
    }else{
        eprintln!("{:?}",result);
        None
    }
}
