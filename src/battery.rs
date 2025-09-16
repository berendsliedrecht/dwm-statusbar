use core::fmt::Display;
use std::fs::read_to_string;

pub struct Battery {
    bat0_percentage: String,
    bat0_is_charging: bool,
    bat1_percentage: String,
    bat1_is_charging: bool,
}

impl Battery {
    pub fn new() -> Result<Self, ()> {
        let (bat0_percentage, bat0_is_charging) = Self::read_battery("BAT0")?;
        let (bat1_percentage, bat1_is_charging) = Self::read_battery("BAT1")?;

        Ok(Self {
            bat0_percentage: format!("{:.0}", bat0_percentage),
            bat0_is_charging,
            bat1_percentage: format!("{:.0}", bat1_percentage),
            bat1_is_charging,
        })
    }

    fn read_battery(bat: &str) -> Result<(f32, bool), ()> {
        let base = format!("/sys/class/power_supply/{}", bat);

        let energy_now = read_to_string(format!("{}/energy_now", base))
            .map_err(|_| ())?
            .trim()
            .parse::<u64>()
            .map_err(|_| ())?;

        let energy_full = read_to_string(format!("{}/energy_full", base))
            .map_err(|_| ())?
            .trim()
            .parse::<u64>()
            .map_err(|_| ())?;

        let status = read_to_string(format!("{}/status", base)).map_err(|_| ())?;
        let is_charging = status.trim() == "Charging";

        if energy_full == 0 {
            return Err(());
        }

        let percentage = (energy_now as f32 / energy_full as f32) * 100.0;

        Ok((percentage, is_charging))
    }
}

impl Display for Battery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}% | {}: {}%",
            if self.bat0_is_charging { "C" } else { "0" },
            self.bat0_percentage,
            if self.bat1_is_charging { "C" } else { "1" },
            self.bat1_percentage
        )
    }
}
