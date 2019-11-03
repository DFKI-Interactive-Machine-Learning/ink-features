# Digital Ink Features
This is a reference implementation of digital ink features for behavior characterization with wrappers for Rust and Python as part of our paper "A categorisation and implementation of digital pen features for behaviour characterisation" (https://arxiv.org/abs/1810.03970).

## Implementation
We distribute pre-compiled binaries which were written in Rust, these can be found in the `bin` folder. We also provide wrappers for Python inside the `python` folder and for Rust inside the `rust` folder. We support 3 input modes for digital ink data:
* Loading a `.json` file from a given filepath (for the specification of the JSON format see below).
* Loading a JSON string (for the specification of the JSON format see below).
* Loading ink in native format of the Python/Rust language.

### Requirements
The example code was tested on Windows 10 (x64) with Python 3.7.

### JSON digital ink format
We provide 2 types of containers for digital ink data: `stroke` and `sketch`. 
#### Stroke
#### Sketch


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
