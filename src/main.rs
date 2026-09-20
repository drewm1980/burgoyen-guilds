use chrono::{Utc, TimeZone};
use parry3d::query::{Ray, RayCast};
use parry3d::shape::TriMesh;
use nalgebra::{Point3, Vector3};
use solar_positioning::{spa, time::DeltaT};
use std::fs::File;
use std::io::{Read, Write};

const LAT: f64 = 51.0500;
const LON: f64 = 3.7303;
const ELEVATION: f64 = 10.0;

// Heights (above ground)
const H_HOUSE: f32 = 8.69;
const H_VILLA: f32 = 11.22;
const H_NEIGHBOR: f32 = 9.0; // 3 stories approx

struct Building {
    name: String,
    mesh: TriMesh,
}

fn create_box(name: &str, x: f32, y: f32, w: f32, h: f32, z_h: f32) -> Building {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    // 0: BL, 1: BR, 2: TR, 3: TL (Bottom)
    vertices.push(Point3::new(x, y, 0.0));
    vertices.push(Point3::new(x + w, y, 0.0));
    vertices.push(Point3::new(x + w, y + h, 0.0));
    vertices.push(Point3::new(x, y + h, 0.0));
    
    // 4: BL, 5: BR, 6: TR, 7: TL (Top)
    vertices.push(Point3::new(x, y, z_h));
    vertices.push(Point3::new(x + w, y, z_h));
    vertices.push(Point3::new(x + w, y + h, z_h));
    vertices.push(Point3::new(x, y + h, z_h));

    // Indices for TriMesh (Triangles)
    let faces = vec![
        [0, 1, 4], [4, 1, 5], // Front
        [1, 2, 5], [5, 2, 6], // Right
        [2, 3, 6], [6, 3, 7], // Back
        [3, 0, 7], [7, 0, 4], // Left
        [4, 5, 7], [7, 5, 6], // Top
    ];
    for f in faces { indices.push(f); }

    Building { name: name.to_string(), mesh: TriMesh::new(vertices, indices) }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating Synthetic 3D Model from CAD Anchors...");

    let mut buildings = Vec::new();

    // 1. PROJECT BUILDINGS (Based on label anchors)
    // North Wing (Woningen 1, 2, 3)
    buildings.push(create_box("North Wing", 1000.0, 1000.0, 400.0, 150.0, H_HOUSE));
    // East Wing (Woningen 4, 5, 6)
    buildings.push(create_box("East Wing", 1400.0, 700.0, 150.0, 400.0, H_HOUSE));
    // South Wing (Woningen 7, 8)
    buildings.push(create_box("South Wing", 1100.0, 600.0, 300.0, 150.0, H_HOUSE));
    // Common House
    buildings.push(create_box("Villa", 1150.0, 800.0, 150.0, 100.0, H_VILLA));

    // 2. NEIGHBORS (shading the garden)
    buildings.push(create_box("Neighbor East", 1700.0, 0.0, 500.0, 2000.0, H_NEIGHBOR));
    buildings.push(create_box("Neighbor West", 0.0, 0.0, 800.0, 2000.0, H_NEIGHBOR));
    buildings.push(create_box("Neighbor North", 0.0, 1500.0, 3000.0, 500.0, H_NEIGHBOR));

    println!("Simulating Solar Exposure for the Food Forest Zone...");

    // Target Area: East of Unit 3 (approx X: 1400-1700, Y: 1000-1300)
    let mut results = File::create("viz/food_forest_sun.csv")?;
    writeln!(results, "x,y,hours_summer,hours_winter")?;

    let solstice_summer = Utc.with_ymd_and_hms(2026, 6, 21, 12, 0, 0).unwrap();
    let solstice_winter = Utc.with_ymd_and_hms(2026, 12, 21, 12, 0, 0).unwrap();
    
    // Sampling loop
    for gy in (1000..1400).step_by(20) {
        for gx in (1400..1800).step_by(20) {
            let pt = Point3::new(gx as f32, gy as f32, 0.5);
            
            // Summer Hours (6:00 to 21:00)
            let mut s_hits = 0;
            for hour in 6..21 {
                let time = Utc.with_ymd_and_hms(2026, 6, 21, hour, 0, 0).unwrap();
                let delta_t = DeltaT::estimate_from_date_like(time.date_naive()).unwrap();
                let pos = spa::solar_position(time, LAT, LON, ELEVATION, delta_t, None)?;
                if pos.zenith_angle() < 90.0 {
                    let az = (90.0 - pos.azimuth()).to_radians();
                    let el = (90.0 - pos.zenith_angle()).to_radians();
                    let dir = Vector3::new(az.cos() as f32, az.sin() as f32, el.sin() as f32);
                    let mut shaded = false;
                    for b in &buildings {
                        if b.mesh.cast_local_ray(&Ray::new(pt, dir), 3000.0, true).is_some() {
                            shaded = true; break;
                        }
                    }
                    if !shaded { s_hits += 1; }
                }
            }

            // Winter Hours (9:00 to 16:00)
            let mut w_hits = 0;
            for hour in 9..16 {
                let time = Utc.with_ymd_and_hms(2026, 12, 21, hour, 0, 0).unwrap();
                let delta_t = DeltaT::estimate_from_date_like(time.date_naive()).unwrap();
                let pos = spa::solar_position(time, LAT, LON, ELEVATION, delta_t, None)?;
                if pos.zenith_angle() < 90.0 {
                    let az = (90.0 - pos.azimuth()).to_radians();
                    let el = (90.0 - pos.zenith_angle()).to_radians();
                    let dir = Vector3::new(az.cos() as f32, az.sin() as f32, el.sin() as f32);
                    let mut shaded = false;
                    for b in &buildings {
                        if b.mesh.cast_local_ray(&Ray::new(pt, dir), 3000.0, true).is_some() {
                            shaded = true; break;
                        }
                    }
                    if !shaded { w_hits += 1; }
                }
            }
            writeln!(results, "{},{},{},{}", gx, gy, s_hits, w_hits)?;
        }
    }

    println!("Simulation complete. Results in viz/food_forest_sun.csv");
    Ok(())
}
