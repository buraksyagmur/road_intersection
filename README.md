# Traffic Simulation

This project simulates a traffic intersection with two roads crossing each other, each having one lane in each direction. The simulation involves vehicles following specific rules and traffic lights controlling the flow of vehicles. The goal is to prevent collisions and ensure smooth traffic movement. 

## Environment and Rules

### Roads
```console
                        North
                    |  ↓  |  ↑  |
                    |  ↓  |  ↑  |
                    |     |     |
                    |     |     |
                    |     |     |
                    |     |     |
     _______________|     |     |_______________
     ← ←                                     ← ←
East ---------------             --------------- West
     → →                                     → →
     _______________             _______________
                    |     |     |
                    |     |     |
                    |     |     |
                    |     |     |
                    |     |     |
                    |  ↓  |  ↑  |
                    |  ↓  |  ↑  |
                        South
```
- The simulation includes two roads that intersect, forming a crossroads.
- Each road has one lane in each direction.
- Traffic entering the intersection can select a route by turning left, turning right, or continuing straight.

### Traffic Lights

- Traffic lights are positioned at the entrance of each lane where it meets the intersection.
- The traffic lights in this simulation have only two colors: red and green.
- You can implement any algorithm to control the traffic lights system, but congestion should be kept low (8 or fewer vehicles).
- The primary function of the traffic light system is to avoid collisions between vehicles passing through the intersection.

### Vehicles

- Vehicles in the simulation follow specific rules and behavior:
  - Each vehicle is painted in a color that represents the route it will follow. 
  - Vehicles have a fixed velocity.
  - A safety distance from other vehicles must be maintained. If a vehicle in front stops, the following vehicle must also stop to maintain a safe distance.
  - Vehicles must stop at a red traffic light and proceed when it turns green.
  - Vehicles cannot change their selected route.
  - There are no special privileges for any vehicle types. No emergency vehicles are considered in this simulation.

## Commands

- Keyboard commands are used to control and interact with the simulation:
  - Arrow Keys:
    - ↑ (Up): Spawn a vehicle moving towards the intersection from the south.
    - ↓ (Down): Spawn a vehicle moving towards the intersection from the north.
    - → (Right): Spawn a vehicle moving towards the intersection from the west.
    - ← (Left): Spawn a vehicle moving towards the intersection from the east.
  - r: Spawn a vehicle with a random direction towards the intersection.
  - Esc (Escape): End the simulation.
- Vehicles should be spawned with a safe distance between them to avoid collisions.

## How to Run the Simulation

- Launch the simulation and observe the traffic movement and behavior at the intersection.
- Use the keyboard commands to spawn vehicles from different directions and observe their routes and interactions.
- Monitor the traffic lights and their impact on the flow of vehicles.
- Use the Esc key to end the simulation.

Enjoy simulating the traffic flow at the intersection!

