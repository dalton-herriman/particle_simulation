use plotters::prelude::*;
use std::{thread, time};
use std::fs;

struct Particle {
    x_position: f64,
    y_position: f64,

    x_velocity: f64,
    y_velocity: f64,

    mass: f64,

    lifespan: f64, // seconds
    age: f64,      // seconds

    isdead: bool,

    color: (u8, u8, u8), // RGB tuple

    position_history: Vec<(f64, f64)>, // vector of all previous locations
}

impl Particle {
    pub fn update(&mut self, force_x: f64, force_y: f64, dt: f64) {
        if self.mass == 0.0 {
            panic!("Mass cannot be zero.");
        }

        if self.isdead {
            return;
        }

        self.age += dt;
        if self.age >= self.lifespan {
            self.isdead = true;
            return;
        }

        // Save current position
        self.position_history.push((self.x_position, self.y_position));

        // Limit history length to 50 points
        if self.position_history.len() > 50 {
            self.position_history.remove(0);
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

fn draw_plotter(
    particle: &Particle,
    width: u32,
    height: u32,
    filename: &str,
    x_range: (f64, f64),
    y_range: (f64, f64),
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Particle Simulation", ("Arial", 20))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;

    chart.configure_mesh().draw()?;

    // Draw trail as small circles
    for &(x, y) in &particle.position_history {
        chart.draw_series(PointSeries::of_element(
            vec![(x, y)],
            3,
            &RGBColor(particle.color.0, particle.color.1, particle.color.2).mix(0.5),
            &|c, s, st| {
                return EmptyElement::at(c)
                    + Circle::new((0, 0), s, st.filled());
            },
        ))?;
    }

    // Draw current position as a bigger circle
    chart.draw_series(PointSeries::of_element(
        vec![(particle.x_position, particle.y_position)],
        8,
        &RGBColor(particle.color.0, particle.color.1, particle.color.2),
        &|c, s, st| {
            return EmptyElement::at(c)
                + Circle::new((0, 0), s, st.filled());
        },
    ))?;

    root.present()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 800;
    let height = 600;

    // Create a subdirectory for frames
    let frames_dir = "frames";
    if !std::path::Path::new(frames_dir).exists() {
        fs::create_dir(frames_dir)?;
    }

    let mut particle = Particle {
        x_position: (width / 2) as f64,
        y_position: (height / 2) as f64,
        x_velocity: 150.0, // adjust to pixel units/sec for plotters scale
        y_velocity: 0.0,
        mass: 1.0,
        lifespan: 20.0,
        age: 0.0,
        isdead: false,
        color: (255, 0, 0),
        position_history: Vec::new(),
    };

    let gravity = 981.0; // pixels/sec^2 (scaled up)
    let dt = 0.1; // 100 ms timestep

    // Define coordinate range for plotters (in pixels)
    let x_range = (0.0, width as f64);
    let y_range = (0.0, height as f64);

    let mut frame_count = 0;

    while !particle.isdead {
        particle.update(0.0, gravity * particle.mass, dt);

        // Save frames in the subdirectory
        let filename = format!("{}/frame_{:04}.png", frames_dir, frame_count);
        draw_plotter(&particle, width, height, &filename, x_range, y_range)?;

        println!(
            "Frame {}: Position: ({:.2}, {:.2}), Velocity: ({:.2}, {:.2}), Age: {:.2}",
            frame_count,
            particle.x_position,
            particle.y_position,
            particle.x_velocity,
            particle.y_velocity,
            particle.age,
        );

        frame_count += 1;

        thread::sleep(time::Duration::from_millis(100));
    }

    println!("Particle died after {:.2} seconds.", particle.age);

    Ok(())
}
