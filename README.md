# Using Drones to Detect Distances and Infractions in Field Events
## Project Overview
The goal of this project is to develop a drone-based monitoring system to assist officials in detecting infractions and measuring distances in field throwing events, including shot put, discus, hammer throw, and javelin. The system is designed to identify foot infractions (e.g., stepping out of bounds) as well as determine whether the implement lands outside the legal sector. Additionally, the system reports the distance from the throwing circle to the landing point.

Current officiating in these events relies heavily on human judgment, which can be error-prone and inconsistent. Our objective is to improve the accuracy and efficiency of officiating while minimizing the technical burden placed on referees.

The system integrates an aerial drone equipped with computer vision, a ground-based sensing and processing pipeline, and a mobile application that serves as a translation layer to interface with the drone’s SDK. Furthermore, a graphical user interface allows referees to view detections, measurements, and system decisions in an intuitive and accessible manner.

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
This section outlines the purpose of each directory in the repository.

The repository is organized by runtime environment, grouping software according to the **computer on which it executes**. This approach reflects the fact that each machine has distinct programming languages, dependencies, and build systems. As a result, each top-level directory is largely self-contained.

Each runtime directory includes a `comms/` subdirectory to handle networking and message transport. Message definitions themselves are specified in a root-level `interfaces/` directory, which defines language-agnostic contracts shared across all systems. This separation prevents one runtime from depending on another runtime’s implementation details simply to interpret state or commands.

### companion/
The companion PC runs computer vision and high-level flight control logic (autonomy). The software stack here will be primarily Python.

#### computer_vision/
This section converts raw sensor data into interpretable information. OpenCV or ML code will go here to enable feature detection, tracking, camera calibration, etc.

Inputs:
* Camera frames
* State estimates

Outputs:
* Pixel position of object

#### flight_control/ 
This section decides what the drone should do next, based on computer vision outputs. This is where the state machine and high-level drone logic lives.

Inputs:
* Computer vision outputs
* Drone state
* Operator commands

Outputs:
* High-level commands to move a drone (move to given position, set velocity, etc.)
* Take off, Land, etc.

### mobile/
This directory features all software that will run on our Android mobile device. The mobile device is primarily responsible for translating high level commands from flight control to drone-specific mobile SDK calls, allowing us to remotely control the drone based on our custom logic. The software stack will primarily feature Kotlin and Java, and require a Gradle build system.

Inputs:
* Flight control commands

Outputs:
* Execution of drone motion via SDK commands
* Telemetry reporting drone state and health


### ground/       
This directory contains software that runs on the ground station used by the referee. The ground station provides a graphical user interface for monitoring detections, infractions, and distance measurements. The software stack primarily uses React and requires a Node.js runtime.

Inputs:
* Drone reports of infractions and measured distances

Outputs:
* Visualized results and status information presented to the referee
