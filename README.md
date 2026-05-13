# Using Computer Vision to Detect Distances and Infractions in Throwing Events
## Project Goal
The goal of this project is to engineer a system to assist officials in detecting infractions and measuring distances in the 4 throwing events, including shot put, discus throw, hammer throw, and javelin throw. The system is designed to identify foot infractions (i.e., stepping out of bounds) as well as determine whether the implement lands outside the legal sector. Additionally, the system reports the distance from the throwing circle to the landing point.

Current officiating in these events relies heavily on human judgment, which can be error-prone and inconsistent. Our objective is to improve the accuracy and efficiency of officiating while minimizing the technical burden placed on referees.

The system integrates ground cameras equipped with computer vision, a ground-based sensing and processing pipeline, and a graphical user interface allowing referees to understand system decisions in an intuitive and accessible manner.

### System Overview
#### Computer Vision
We use 2 [LUCID Vision Labs™ Atlas ATP124S](https://www.edmundoptics.com/p/lucid-vision-labst-atlas-atp124s-mc-sony-imx545-123mp-ip67-monochrome-camera/49821/) cameras that are time synchronized to capture frames at 30 frames per second. These frames are then sent to our backend which processes each frames using a series of computer vision stages (lens undistortion, forwarding downsampled copies, Mog2, and contours) in parallel. Our backend will then determine the pixels on the frame that correspond to the shot put, where it landed, whether an infraction occurred, and forward all this information to the frontend.

#### Circle Infraction Detection
As part of our project, we also aim to detect whether or not an athlete steps out of bounds during the throw or not. To achieve this, we use capacitive touch sensors that are housed within a 3D printed casing, that when touched, send signals to an Arudino Uno via I2C. The Arduino then sends signals to our server running on a PC via UART.

### Team Mjölnir
The team's name "Mjölnir" is named after the legendary hammer of Thor, the Norse god of thunder. Mjölnir infamously returns to Thor when he throws it, and since our system helps officiate hammer throw, we thought the name was fitting. Also, in the Marvel Cinematic Universe, the synthetic android "Vision" is one of the few beings able to lift Mjölnir — computer vision...

The following engineers contributed to this project.
- Owen Scott
- Luke Venkataramanan
- Yash Jain
- Max Wiesenfeld
- Rushil Randhar
- Arushi Sadam
- Gautam Rao
- Alex Lozano
- Eloghosa Eguakun

## Usage
### Releases
To install our compiled binaries directly, please see the [releases page](https://github.com/luke-venk/mjolnir/releases/tag/v1.0.0).

### Documentation
For technical documentation on how to use our product, please refer to the following READMEs:
1. [README.dev.md](README.dev.md): Instructions to get start with development (dependencies, extensions, etc.)
2. [README.bazel.md](README.bazel.md): Instructions to build our project using Bazel
3. [README.cameras.md](README.cameras.md): Instructions to run our camera tools using Bazel

## Repository Structure
This section outlines the purpose of each directory in the repository.

### Backend
We wrote the backend in Rust, and its responsibilities include the following:
* Providing tools for recording and streaming using the cameras.
* Running per-camera computer vision pipelines that process frames in parallel.
* Using comptuer vision outputs to determine where the object landed on the field.
* Listening for messages from the Arduino communicating circle infractions.
* Running the Axum web server to serve our frontend.

### Circle Infractions
The Arduino software that continuously reads from capacitive touch sensors, determines which touches are infractions, and sends them to the backend over UART lives here.

### Frontend
The graphical user interface for our web application will live here, and the frontend is written in Next.js (TypeScript).

### Analysis
Code that helps inform our design decisions lives here (e.g., MATLAB scripts to determine camera error, Python scripts to validate computer vision pipelines, etc.). These are not included in our final binaries.
