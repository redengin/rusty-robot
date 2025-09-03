#![no_std]

/// autonomous drone controlled by waypoints
pub mod autonomous {
    use rusty_robot_drivers::{
        gps_traits::{self, Gps},
        imu_traits::{self, ImuReader},
    };
    use rusty_robot_robots::systems;

    pub fn step<T>(drone: &T)
    where
        T: imu_traits::ImuReader + gps_traits::Gps + systems::QuadCopterMotors,
    {
        // TODO step an actual flight controller
        let _ = <T as ImuReader>::get_data(drone);
        let _ = <T as Gps>::get_data(drone);
        let velocities_pct: [u8; 4] = [51, 51, 51, 51];
        <T as systems::QuadCopterMotors>::set_data(drone, velocities_pct);
    }
}

/// use a neural network to learn XOR
///     profile how long it takes
pub fn learn_xor() {
    // use fnn::prelude::{SVector, Sigmoid};

    // const INPUT_COUNT: usize = 2;
    // const HIDDEN_LAYERS: usize = 5; // suggested maximum
    // const OUTPUT_COUNT: usize = 1;
    // const PRECISION: u8 = 2;   // epsilon as number of decimal places
    // let mut nn = fnn::FeedForward::<Sigmoid, INPUT_COUNT, HIDDEN_LAYERS, OUTPUT_COUNT>::new();

    // let training_data = [
    //     ([0.0, 0.0], [0.0]),
    //     ([0.0, 1.0], [1.0]),
    //     ([1.0, 0.0], [1.0]),
    //     ([1.0, 1.0], [0.0]),
    // ];

    // // repeat training until learned
    // loop {
    //     // train
    //     for (input, target) in &training_data {
    //         let sv_input = SVector::from_column_slice(input);
    //         let sv_target = SVector::from_column_slice(target);
    //         nn.train(&sv_input, &sv_target, 0.1);
    //     }

    //     // check if it's learned
    //     for (input, target) in &training_data {
    //         let sv_input = SVector::from_column_slice(input);
    //         // nn.train(&sv_input, &sv_target, 0.1);
    //         let output = nn.forward(&sv_input);
    //         if ! approx_equal(output[0], target[0], PRECISION)
    //         {
    //             continue // keep learning
    //         }
    //     }
    //     // learning complete
    //     return 
    // }

    // fn approx_equal (a: f64, b: f64, dp: u8) -> bool {
    //     let p = 10f64.powi(-(dp as i32));
    //     (a-b).abs() < p
    // }
}
