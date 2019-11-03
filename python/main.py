#!/usr/bin/env python
# -*- coding: utf-8 -*-

# -----------------------------------
# Ensure the binary is in python path
# -----------------------------------
import sys
sys.path.insert(1, '../bin')
import digital_ink_features  # Will be dynamically loaded on runtime


# ------------------------------------
# Define list of features to calculate
# ------------------------------------
feature_selection = [
    "weber_feature_05_compactness",
    "rubine_feature_01_cosine_initial_angle",
    "weber_feature_02_length",
    "hbf49_feature_13_downstrokes_trajectory_proportion",
    "hbf49_feature_22_kperpendicularity",
    "willems_feature_24_duration"
]


# --------------------------------------------------
# Optional: define additional arguments for features
# --------------------------------------------------
# Some features can be configured to take additional arguments. Create a map to pass these.
# {
#      "feature_name": {"arg1": val1, "arg2": val2},
#      ...
# }
# If no arguments are provided this way, the default values will be used.
feature_args = {
    "hbf49_feature_13_downstrokes_trajectory_proportion": {
        "t1": 1.,
        "t2": 5.
    },
    "hbf49_feature_22_kperpendicularity": {
        "k": 2
    }
}


# ---------------------------------------
# Example #1: Using a .json file as input
# ---------------------------------------
print()
print("Example #1: loading sample from .json file")
sample_file = '../data/sample.json'
file_result = digital_ink_features.calculate_features_file(sample_file, feature_selection, feature_args)
print("\t", file_result)
print()


# --------------------------------------------------
# Example #2: Using a json formatted string as input
# --------------------------------------------------
json_string = '''
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
'''
print("Example #2: passing json formatted string")
json_result = digital_ink_features.calculate_features_string(json_string, feature_selection, feature_args)
print("\t", json_result)
print()


# ------------------------------
# Example #3: Using native types
# ------------------------------
stroke1 = {
    "x": [0., 1., 1., 0., 0.],
    "y": [0., 0., 1., 1., 0.],
    "timestamp": [1, 2, 3, 4, 5],
    "pressure": [1., 1., 1., 1.]
}
strokes = [stroke1]

print("Example #3: using native types")
native_result = digital_ink_features.calculate_features_dict(strokes, feature_selection, feature_args)
print("\t", native_result)
print()
