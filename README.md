# Digital Ink Features
This is a reference implementation of digital ink features for behavior characterization with wrappers for Rust and Python as part of our paper "A categorisation and implementation of digital pen features for behaviour characterisation" (https://arxiv.org/abs/1810.03970).

## Implementation
We distribute pre-compiled binaries which were written in Rust, these can be found in the `bin` folder. We also provide wrappers for Python inside the `python` folder and for Rust inside the `rust` folder. We support 3 input modes for digital ink data:
* Loading a `.json` file from a given filepath (for the specification of the JSON format see below).
* Loading a JSON string (for the specification of the JSON format see below).
* Loading ink in native format of the Python/Rust language.
Inside the `data` folder we provide samples of digital ink encoded in our digital ink JSON format which is explained below.

### Requirements
The example code was tested on Windows 10 (x64) with Python 3.7.

### Features
A full list of features including their mathematical definition can be found in the Appendix section of our technical report (https://arxiv.org/abs/1810.03970). We provide reference implementations for the following feature sets:
* `rubine`
* `weber`
* `hbf49`
* `willems`

As you can see in the example code, you can select which features to compute by supplying a list of `keys` to the binary. A full list of feature keys is provided in the `feature_keys.txt` file. The structure of each key is as follows:
```
{feature set}_feature_{index}_{acronym}
```
Examples:
```
weber_feature_05_compactness
rubine_feature_01_cosine_initial_angle
weber_feature_02_length
hbf49_feature_13_downstrokes_trajectory_proportion
hbf49_feature_22_kperpendicularity
willems_feature_24_duration
```
As per their definition some of these features might require additional arguments which can be provided individually per feature if necessary, otherwise default values will be used. Examples of how to configure these arguments can be found in the example code.

### JSON digital ink format
We provide 2 types of containers for digital ink data: `stroke` and `sketch`. Objects of type `stroke` contain 4 arrays of data that have to be of same length: `x` coordinates as `float`, `y` coordinates as `float`, `timestamp` for timestamps as `long` and `pressure` values as `float`. The `type` field let's us distinguish between `stroke` and `sketch` as JSON itself is untyped. The `meta` field can be used for additional information, such as labels. Be aware that it has to be present event if empty, otherwise parsing errors will occur.

#### Stroke
In general a `stroke` is represented as follows:
```
{
    "type": "stroke",
    "meta": {
        "key_1": value_1,
        "key_2": value_2,
        ...
    },
    "x": [
        x_1: float,
        x_2: float,
        ...
        x_n: float
    ],
    "y": [
        y_1: float,
        y_2: float,
        ...
        y_n: float
    ],
    "timestamp": [
        timestamp_1: long,
        timestamp_2: long,
        ...
        timestamp_n: long
    ],
    "pressure": [
        pressure_1: float,
        pressure_2: float,
        ...
        pressure_n: float
    ]
}
```
Note that fields `x` and `y` are required, while `timestamp` and `pressure` are optional. Be aware that these still need to be present if you want to calculate respective features that require these values. `timestamp` values can be either relative offset values, e.g., starting at `0` or alternatively absolute values such as unix timestamps.

Here is an example with actual values:
```
{
    "type": "stroke",
    "meta": {},
    "x": [0.0, 1.0, 1.0, 0.0, 0.0],
    "y": [0.0, 0.0, 1.0, 1.0, 0.0],
    "timestamp": [1, 2, 3, 4, 5],
    "pressure": [1.0, 1.0, 1.0, 1.0, 1.0]
}
```

#### Sketch
A `sketch` is a ordered collection of multiple `stroke` objects:
```
{
    "type": "sketch",
    "meta": {
        "key_1": value_1,
        "key_2": value_2,
        ...
    },
    "strokes": [
        <stroke_1>,
        <stroke_2>
        ...
        <stroke_n>
    ]
}
```
Here is an example of a `sketch` with sample value:
```
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
```

## Citation
If you use our work in your research please cite the following technical report:
```
@misc{alex2018categorisation,
    title={A categorisation and implementation of digital pen features for behaviour characterisation},
    author={Alexander Prange and Michael Barz and Daniel Sonntag},
    year={2018},
    eprint={1810.03970},
    archivePrefix={arXiv},
    primaryClass={cs.CV}
}
```
