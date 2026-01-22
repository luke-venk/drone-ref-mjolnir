"""
Runs the flight control loop at a certain frequency and uses the controller
to step through the state machine and publish resulting commands.
"""

# Control loop frequency will be 20 Hz.
FREQUENCY = 20

def step():
    """
    (1) Read the latest drone perception output.
    (2) Read latest drone state telemetry.
    (3) See if operator issued a manual command.
    (4) Update the state machine.
    (5) Publish the command.
    """