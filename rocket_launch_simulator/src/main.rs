/*
Name : Naitik Makawna
Roll No : 20BCP121
*/


use std::{thread, time};

//setting target distance
const TARGET : u32 = 100;

//structure of rocket simulator
struct RocketLaunchSimulator {
    stage: &'static str,
    fuel: u32,
    altitude: u32,
    speed: u32,
}

impl RocketLaunchSimulator {

    //to initiate a simulation as default
    fn new() -> Self {
        RocketLaunchSimulator {
            stage: "Pre-Launch",
            fuel: 100,
            altitude: 0,
            speed: 0,
        }
    }
    

    // This function checks rocket specification before launch
    fn pre_launch_checks(&self) -> bool {
        println!("Running pre-launch checks...");
        // perform pre-launch checks here
        let mut status = false;

        if self.stage == "Pre-Launch" && self.fuel == 100 && self.altitude == 0 && self.speed == 0 {
            status = true;
        }
        println!("All systems are 'Go' for launch.");
        return status;
    }

    //launch function which starts simulation calling run_simulation function
    fn launch(&mut self) {
        if self.stage != "Pre-Launch" {
            println!("Error: Launch already initiated.");
            return;
        }

        println!("Launching...");
        self.stage = "0";

        self.run_simulation();
    }


    //to run simulation
    fn run_simulation(&mut self) {
        loop {
            self.stage_updation();
            thread::sleep(time::Duration::from_secs(1));

            //mission success condition
            if self.altitude == TARGET {
                break;
            }

            //misson failing condition
            if self.fuel == 0 && self.altitude < TARGET {
                println!("Mission Failed due to insufficient fuel.");
                break;
            }
        }
    }


    // stage updation function to update stages and related values in simulation
    fn stage_updation(&mut self) {
        // Simulate the rocket parameters update for each second
        self.fuel -= 10;
        self.altitude += 10;
        
        //stage changing  conditions
        if self.stage == "0" && self.altitude <= 10 {
            self.stage = "0";
            self.speed = 500;
           
        }

        if self.stage == "0" && self.altitude > 10 {
            self.stage = "1";
            self.speed = 1000;
            println!("Stage 0 complete. Separating stage. Entering Stage 1.");
        }

        if self.stage == "1" && self.altitude > 50 {
            self.stage = "2";
            self.speed = 1500;
            println!("Stage 1 complete. Separating stage. Entering Stage 2.");
        }

        println!(
            "Stage: {}, Fuel: {}%, Altitude: {} km, Speed: {} km/h",
            self.stage, self.fuel, self.altitude, self.speed
        );

        //mission complition conditions
        if self.stage == "2" && self.altitude >= TARGET {
            self.stage = "3";
            println!("Stage 2 complete. Separating stage. Entering Stage 3");
            println!("Orbit achieved! Mission Successful");
        }
    }


    //fast forward function to skip the simulation till 'X' seconds
    fn fast_forward(&mut self, seconds: u32) {
        for _ in 0..seconds {
            self.fuel -= 10;
            self.altitude += 10;
        }

        // determining the current stage after fast forwarding
        if self.altitude == 10 {
            self.stage = "1";
            self.speed = 500;
        } else if self.altitude >= 50 {
            self.stage = "2";
            self.speed = 1000;
        } else if self.altitude >= 100 {
            self.stage = "3";
            self.speed = 1500;
        }

        println!("After fast forwarding...");

    }

}

fn main() {
    
    //initializing rocket simulator in ideal condition
    let mut simulator = RocketLaunchSimulator::new();

    // //Rocket specifications are not ideal
    // let mut simulator = RocketLaunchSimulator {
    //     stage : "Pre-Launch",
    //     fuel : 80,
    //     altitude : 0,
    //     speed : 0,
    // };
    
    //user input for command
    let mut user_input = String::new();
    println!("Enter command:");
    std::io::stdin().read_line(&mut user_input).expect("Failed to read input");
    let user_input = user_input.trim();
    
    //matching user input 
    match user_input {
        // to complusory check system before launching
        "start_checks" => {

            //pre-launch check
            if simulator.pre_launch_checks() == true {
                loop {
                    let mut user_input = String::new();
                    println!("Enter command:");
                    std::io::stdin().read_line(&mut user_input).expect("Failed to read input");
                    let user_input = user_input.trim();
                    
                    //matching user imput
                    match user_input {
    
                        "launch" => {
                            simulator.launch();
                            break;
                        },
                        
                        //for fast forwarding
                        _ if user_input.starts_with("fast_forward") => {

                            let seconds: u32 = user_input.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);

                            //calling fast forward function
                            simulator.fast_forward(seconds);
                            // Continue simulation from the next second
                            simulator.run_simulation();
                            break;
                        },
                        
                        //exception handling
                        _ => println!("Invalid command. Please enter 'launch' or 'fast_forward X'."),
    
                    }
                }
            } else {
                //exception handling
                println!("Rocket is not in ideal condition for launch");
            }

        },
        
        //exception handling
        _ => println!("Invalid command. Please enter 'pre_launch_checks' to check your system first"),
    }
    
}
