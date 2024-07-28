# rust-image-preprocessing
This repo implements a simple Rust image processing library. The goal of this project is to get a better understanding of the Rust programming language. 

Currently implemented image information features:
- Calculate brightness of an image
- Calculate blur score based on the laplacian variance of the image

Currently implemented image transformation features:
- Get and save the negative version of an image 
- Get and save gamma log transformation of an image (makes the image much brighter)
- Get and save gamma power law transformation (makes the image darker, dependent on the gamma parameter)