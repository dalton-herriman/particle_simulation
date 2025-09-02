struct Particle {
    x_position: f64,
    y_position: f64,
    
    x_velocity: f64,
    y_velocity: f64,

    mass: f64,
}

impl Particle {
    pub fn update(&mut self, force_x: f64, force_y: f64, dt: f64) {
        if self.mass == 0.0 {
            panic!("Mass cannot be zero.");
        }

        // Compute acceleration
        let ax = force_x / self.mass;
        let ay = force_y / self.mass;

        // Update velocity
        self.x_velocity += ax * dt;
        self.y_velocity += ay * dt;

        // Update position
        self.x_position += self.x_velocity * dt;
        self.y_position += self.y_velocity * dt;
    }
}

fn main() {
    let mut particle = Particle {
        x_position: 0.0,
        y_position: 0.0,
        x_velocity: 0.0,
        y_velocity: 0.0,
        mass: 1.0,
    };
    let gravity = -9.81;
    let dt = 0.01; // 10 ms time step

    for _ in 0..100 {
        // Apply gravity in y-direction
        particle.update(0.0, gravity * particle.mass, dt);

        println!(
            "Position: ({:.2}, {:.2}), Velocity: ({:.2}, {:.2})",
            particle.x_position,
            particle.y_position,
            particle.x_velocity,
            particle.y_velocity,
        );
    }
}