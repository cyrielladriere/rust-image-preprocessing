# rust-image-preprocessing
This repo implements a simple Rust image processing library. The goal of this project is to get a better understanding of the Rust programming language. 

# Current capabilities
Currently implemented image information features:
- Calculate brightness of an image
- Calculate blur score based on the laplacian variance of the image

Currently implemented image transformation features:
- Get and save the negative version of an image 
- Get and save gamma log transformation of an image (makes the image much brighter)
- Get and save gamma power law transformation (makes the image darker, dependent on the gamma parameter)

# Transformation examples
In this section we will transform a "Pulp Fiction" movie poster using different techniques implemented in this repository. The original image looks like this:
![Pulp Fiction movie poster](images/pulp_fiction.png)

## Negative image
Achieved using the `get_negative` function inside `image_transformation.rs`.
![Pulp Fiction movie poster](images/negative_image.png)

## Gamma log transform
Achieved using the `get_gamma_log_transform` function inside `image_transformation.rs`.
![Pulp Fiction movie poster](images/gamma_log_transform.png)

## Gamma log transform
Achieved using the `get_gamma_powlaw_transform` function with `gamma=0.7` inside `image_transformation.rs`.
![Pulp Fiction movie poster](images/gamma_powlaw_transform.png)