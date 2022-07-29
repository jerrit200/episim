use std::io::Write;

use crate::{
    simulator::Simulator,
    statistics::{DataFrame, Demographics},
    util::print_progress,
    CONFIG,
};

use super::Runner;

pub struct NoGraphics {
    pub simulator: Simulator,
}

impl Runner for NoGraphics {
    fn new(simulator: Simulator) -> NoGraphics {
        NoGraphics { simulator }
    }

    fn run(&mut self, debug: bool, show_progress: bool, export: bool) {
        let mut dataframe = DataFrame::new(CONFIG.core.population_size as usize);
        dataframe.push_data(&self.simulator);

        for i in 0..CONFIG.core.time_limit {
            if show_progress {
                let progress = i as f32 / CONFIG.core.time_limit as f32 * 100.0;
                print_progress(progress);
                print!("\r");
            }

            self.simulator.step();

            dataframe.push_data(&self.simulator);
        }

        let demographics = Demographics::from_simulator(&self.simulator);

        if debug {
            println!("{}", dataframe);
            println!("{}", demographics);
        }

        if export {
            // Save data frame as csv file.
            let mut file = std::fs::File::create(format!(
                "export/{}_{}.csv",
                CONFIG.name().split("/").last().unwrap(),
                chrono::offset::Local::now().format("%Y-%m-%d_%H-%M-%S")
            ))
            .expect("Unable to create file");

            file.write_all(dataframe.to_csv().as_bytes())
                .expect("Unable to write to file");

            let _ = dataframe.save_as_chart();
        }
    }
}