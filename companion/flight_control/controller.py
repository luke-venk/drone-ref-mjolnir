"""
The controller will own an instance of the state machine, repeatedly
stepping through it to feed in inputs and and gets Commands out. The
controller is responsbile for translating perception and telemetry
into state machine events.
"""