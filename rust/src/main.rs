extern crate libloading as lib;

use std::collections::HashMap;

// --------------------------------------
// Container representing a single stroke
// --------------------------------------
#[allow(dead_code)]
struct Stroke {
    x: Vec<f64>,
    y: Vec<f64>,
    timestamp: Vec<u64>,
    pressure: Vec<f64>,
}

fn main() {
    // -------------------------------------
    // Load binary depending on architecture
    // -------------------------------------
    #[cfg(windows)]
        let lib = lib::Library::new("../bin/digital_ink_features.dll").unwrap();
    #[cfg(unix)]
        let lib = lib::Library::new("../bin/digital_ink_features.so").unwrap();

    unsafe {
        // --------------------------------------------
        // Create references to functions from binaries
        // --------------------------------------------
        let features_file: lib::Symbol<unsafe extern fn(String, Vec<String>, HashMap<String, HashMap<String, f64>>) -> HashMap<String, f64>> = lib.get(b"features_file").unwrap();
        let features_string: lib::Symbol<unsafe extern fn(String, Vec<String>, HashMap<String, HashMap<String, f64>>) -> HashMap<String, f64>> = lib.get(b"features_string").unwrap();
        let features_strokes: lib::Symbol<unsafe extern fn(Vec<Stroke>, Vec<String>, HashMap<String, HashMap<String, f64>>) -> HashMap<String, f64>> = lib.get(b"features_strokes").unwrap();


        // ------------------------------------
        // Define list of features to calculate
        // ------------------------------------
        let feature_selection = vec!(
            String::from("weber_feature_05_compactness"),
            String::from("rubine_feature_01_cosine_initial_angle"),
            String::from("weber_feature_02_length"),
            String::from("hbf49_feature_13_downstrokes_trajectory_proportion"),
            String::from("hbf49_feature_22_kperpendicularity"),
            String::from("willems_feature_24_duration")
        );


        // --------------------------------------------------
        // Optional: define additional arguments for features
        // --------------------------------------------------
        // Some features can be configured to take additional arguments. Create a map to pass these.
        // {
        //      "feature_name" => {"arg1": val1, "arg2": val2},
        //      ...
        // }
        // If no arguments are provided this way, the default values will be used.
        let mut feature_args: HashMap<String, HashMap<String, f64>> = HashMap::new();
        let feature_args_hbf49_feature_13_downstrokes_trajectory_proportion: HashMap<String, f64> =
            [
                (String::from("t1"), 1.),
                (String::from("t2"), 5.)
            ].iter().cloned().collect();
        let feature_args_hbf49_feature_22_kperpendicularity: HashMap<String, f64> =
            [
                (String::from("k"), 2.)
            ].iter().cloned().collect();
        feature_args.insert(String::from("hbf49_feature_13_downstrokes_trajectory_proportion"), feature_args_hbf49_feature_13_downstrokes_trajectory_proportion);
        feature_args.insert(String::from("hbf49_feature_22_kperpendicularity"), feature_args_hbf49_feature_22_kperpendicularity);


        // ---------------------------------------
        // Example #1: Using a .json file as input
        // ---------------------------------------
        println!();
        println!("Example #1: loading sample from .json file");
        let file_result = features_file(String::from("../data/sample.json"), feature_selection.clone(), feature_args.clone());
        println!("\t{:?}", file_result);
        println!();


        // --------------------------------------------------
        // Example #2: Using a json formatted string as input
        // --------------------------------------------------
        let json_string = String::from(r###"
            {
                "type": "sketch",
                "meta": {},
                "strokes": [
                    {
                        "type": "stroke",
                        "meta": {},
                        "x": [0.0, 1.0, 1.0, 0.0, 0.0],
                        "y": [0.0, 0.0, 1.0, 1.0, 0.0],
                        "timestamp": [1, 2, 3, 4, 5],
                        "pressure": [1.0, 1.0, 1.0, 1.0, 1.0]
                    },
                    {
                        "type": "stroke",
                        "meta": {},
                        "x": [1.0, 1.0, 1.0, 1.0, 1.0],
                        "y": [0.0, 0.0, 0.0, 0.0, 0.0],
                        "timestamp": [1, 2, 3, 4, 5],
                        "pressure": [0.4, 0.2, 0.45, 9.2134, 1.0]
                    }
                ]
            }
        "###);
        println!("Example #2: passing json formatted string");
        let json_result = features_string(json_string, feature_selection.clone(), feature_args.clone());
        println!("\t{:?}", json_result);
        println!();


        // ------------------------------
        // Example #3: Using native types
        // ------------------------------
        let x = vec!(0., 1., 1., 0., 0.);
        let y = vec!(0., 0., 1., 1., 0.);
        let timestamp = vec!(1, 2, 3, 4, 5);
        let pressure = vec!(1., 1., 1., 1., 1.);
        let stroke1 = Stroke { x, y, timestamp, pressure };
        let strokes = vec!(stroke1);

        println!("Example #3: using native types");
        let native_result = features_strokes(strokes, feature_selection.clone(), feature_args.clone());
        println!("\t{:?}", native_result);
        println!();
    }
}

