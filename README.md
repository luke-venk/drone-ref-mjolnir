# Using Drones to Detect Distances and Infractions in Field Events
## Project Overview
The goal of this project is to develop a drone-based monitoring system to track infractions and measure distances in field throwing events – shot put, discus, hammer throw, and javelin. This system will assist human officials to detect infractions both where the athelete steps out of bounds or the object lands out of bounds. Furthermore, the system will report how far from the throwing circle the implement landed. As current officiating relies heavily on human judgement, which is error-prone, we hope our solution will make refereeing such events more accurate and efficient, without requiring technical expertise from the user.

Our project will incorporate flying a drone equipped with computer vision as well as a ground system to detect foot infractions. Furthermore, our project will require a mobile app to serve as a translation layer to communicate with the drone's SDK. Finally, our system will include a graphical user interface for the referee to understand our system's decisions.

## Team Mjölnir
The following engineers contributed to this project.
- Eloghosa Eguakun
- Yash Jain
- Alex Lozano
- Rushil Randhar
- Gautam Rao
- Arushi Sadam
- Owen Scott
- Luke Venkataramanan
- Max Wiesenfeld

## Repository Structure
This section outlines the purpose of each directory in the repository. Anyone is welcome to update the structure as necessary. I didn't put critical thought into each of these, the goal was just to add some structure and organization.

### computer_vision/
This section converts raw sensor data into interpretable information. OpenCV or ML code will go here to enable feature detection, tracking, camera calibration, etc.

Inputs:
* Camera frames
* Kalman filtered sensors

Outputs:
* Pixel position of object

### flight_control/ 
This section decides what the drone should do next, based on computer vision outputs. This is where the state machine and high-level drone logic lives.

Inputs:
* Computer vision outputs
* Drone state
* Operator commands

Outputs:
* High-level commands to move a drone (move to given position, set velocity, etc.)
* Take off
* Land

### drone/          
This section turns high level commands from flight control to drone-specific mobile SDK calls, allowing us to remotely control the drone based on our custom logic. This is where the translation layer to interface with the drone's mobile SDK lives.  

Inputs:
* Flight control commands

Outputs:
* Actually moving the drone
* Telemetry indicating state

### comms/          
This section will allow each of our modules to communicate with each other, regardless of what the payload is.

### frontend/       
This section is where our graphical user interface will live.  
