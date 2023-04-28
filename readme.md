 
# Image and Video Renamer

This Rust program scans a specified folder for image and video files, prompts the user for a title, and renames them with the provided title followed by a unique numeric value.

## Supported file formats

- Images: JPG, JPEG, PNG, GIF, BMP
- Videos: MP4, AVI, MKV, FLV, MOV

## Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)

## Usage

1. Clone this repository to your local machine:
git clone git@github.com:ctaylor83/image-sorter.git

2. Change to the project directory:
cd rename_images

3. Build the project:
cargo build --release


4. Run the program, replacing `<path>` with the directory you want to scan for image and video files:
./target/release/rename_images <path>


5. The program will prompt you to enter a title for the images and videos. Input your desired title and press Enter.

6. The program will then proceed to rename the files with the provided title followed by a unique numeric value and print the results.

## Note

This is a simple solution and plans are to further improved to handle edge cases, errors, and different file formats in a later version.
