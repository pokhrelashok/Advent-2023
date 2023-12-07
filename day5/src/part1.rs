#[derive(Debug)]
struct SoilVariables {
    pub seed: u64,
    pub soil: Option<u64>,
    pub fertilizer: Option<u64>,
    pub water: Option<u64>,
    pub light: Option<u64>,
    pub temperature: Option<u64>,
    pub humidity: Option<u64>,
    pub location: Option<u64>,
}
impl SoilVariables {
    pub fn get(&self, str: &str) -> Option<u64> {
        match str {
            "seed" => Some(self.seed),
            "soil" => Some(self.soil).unwrap(),
            "fertilizer" => Some(self.fertilizer).unwrap(),
            "water" => Some(self.water).unwrap(),
            "light" => Some(self.light).unwrap(),
            "temperature" => Some(self.temperature).unwrap(),
            "humidity" => Some(self.humidity).unwrap(),
            "location" => Some(self.location).unwrap(),
            _ => None,
        }
    }
    pub fn set(&mut self, str: &str, val: u64) {
        match str {
            "soil" => self.soil = Some(val),
            "fertilizer" => self.fertilizer = Some(val),
            "water" => self.water = Some(val),
            "light" => self.light = Some(val),
            "temperature" => self.temperature = Some(val),
            "humidity" => self.humidity = Some(val),
            "location" => self.location = Some(val),
            _ => (),
        };
    }
}
struct Data {
    pub des_start: u64,
    pub source_start: u64,
    pub count: u64,
}

pub fn process(input: &str) -> u64 {
    let mut result: Vec<SoilVariables> = vec![];
    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut line_iter = lines.iter_mut();

    // Using a let binding to create a longer-lived value
    let seed_str = line_iter
        .next()
        .expect("Failed to get next line")
        .split(": ")
        .last()
        .expect("Failed to get last element after splitting")
        .split_whitespace()
        .collect::<Vec<&str>>();

    // Now you can use seed_str in multiple places without temporary value issues
    let seeds: Vec<u64> = seed_str
        .iter()
        .map(|d| d.parse::<u64>().expect("Failed to parse u64"))
        .collect();

    seeds.iter().for_each(|seed| {
        result.push(SoilVariables {
            seed: seed.to_owned(),
            soil: None,
            fertilizer: None,
            water: None,
            light: None,
            temperature: None,
            humidity: None,
            location: None,
        })
    });

    let mut destionation = "".to_string();
    let mut source = "".to_string();
    line_iter.for_each(|line| match line.replace(" ", "").parse::<u128>() {
        Ok(_) => {
            let values_str = line.trim().split(" ").collect::<Vec<&str>>();
            let values = values_str
                .iter()
                .map(|f| f.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let data = Data {
                des_start: values.get(0).unwrap().to_owned(),
                source_start: values.get(1).unwrap().to_owned(),
                count: values.get(2).unwrap().to_owned(),
            };
            result.iter_mut().for_each(|res| {
                let s_val = res.get(&source).unwrap();
                let d_val = res.get(&destionation);
                if data.source_start <= s_val && s_val < (data.source_start + data.count) {
                    res.set(&destionation, s_val - data.source_start + data.des_start);
                } else if d_val.is_none() {
                    res.set(&destionation, s_val)
                }
            })
        }
        Err(_) => {
            if line.contains("-to-") {
                let mut source_destination_name = line.trim().split("-to-");
                source = source_destination_name.next().unwrap().trim().to_string();
                destionation = source_destination_name.next().unwrap().trim().to_string();
            }
        }
    });
    return result.iter().map(|f| f.location.unwrap()).min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process(
            "seeds: 79 14 55 13

        seed-to-soil
        50 98 2
        52 50 48
        
        soil-to-fertilizer
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light
        88 18 7
        18 25 70
        
        light-to-temperature
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity
        0 69 1
        1 0 69
        
        humidity-to-location
        60 56 37
        56 93 4",
        );
        assert_eq!(result, 35);
    }
}
