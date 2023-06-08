// Demonstrates each shape type with a selection of options and possible use cases

#![allow(dead_code)]
use std::{
    f32::consts::{PI, TAU},
    f64::*,
    ops::Range,
};

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

pub trait Pastel {
    fn pastel(&self) -> Color;
}

impl Pastel for Color {
    fn pastel(&self) -> Color {
        *self + Color::WHITE * 0.25
    }
}

pub fn gallery(mut painter: ShapePainter, seconds: f32, entries: Range<i32>) {
    let seconds = seconds % (2.0 * TAU);
    let start_pos = painter.transform;

    for i in entries {
        let (x, y) = ((i % 5 - 2) as f32, (i / 5 - 1) as f32);
        let diag_vec = Vec3::X + Vec3::Y;
        painter.transform = start_pos;
        painter.translate((Vec3::X * x - Vec3::Y * y) * 4.0);

        match i {
            // Line examples
            0 => {
                painter.color = Color::MIDNIGHT_BLUE.pastel() * 1.6;
                painter.cap = Cap::Round;
                painter.line(Vec3::new(-1.0, -1.0, 0.0), Vec3::new(1.0, -1.0, 0.0));
            }
            // Rect examples
            6 => {
                let bar_fill = 0.1 + 0.9 * (seconds.sin() + 1.0) / 2.0;
                let square_fill = ((seconds * 3.).sin() + 1.0) / 2.0;

                painter.hollow = true;
                painter.color = Color::SEA_GREEN.pastel();
                painter.thickness = 0.2 + 1.3 * square_fill;
                painter.corner_radii = Vec4::splat(1.0);
                painter.rotate_z(bar_fill * TAU);
                painter.rect(Vec2::splat(2.5));
            }
            // Circle examples
            2 => {
                painter.hollow = false;
                painter.color = Color::ORANGE.pastel() * 0.7;
                painter.translate(diag_vec * 0.8);
                painter.circle(0.8);

                painter.hollow = true;
                painter.thickness = 0.2;
                painter.color = Color::ORANGE.pastel() * 1.4;
                painter.translate(-diag_vec * 1.6);
                painter.circle(0.8);
            }
            7 => {
                let circle_fill = ((seconds * 2.).sin() + 1.0) / 2.0;

                painter.hollow = true;
                painter.color = Color::ORANGE.pastel();
                painter.thickness = 0.5;
                painter.circle(1.5 * circle_fill);
            }
            12 => {
                fn draw_bubble(
                    painter: &mut ShapePainter,
                    seconds: f32,
                    position: Vec3,
                    scale: f32,
                ) {
                    let seconds = seconds % PI;
                    let circle_size = (seconds).powf(2.8) / 40. * scale;

                    painter.thickness = f32::powf(2.5, 2.8) / 40.0 * scale - circle_size;
                    painter.hollow = true;
                    painter.color = Color::ORANGE.pastel() + Color::WHITE * circle_size;
                    painter.translate(position + Vec3::Y * circle_size * 2.0 * scale);
                    painter.circle(circle_size);
                }
                painter.translate(Vec3::NEG_Y * 0.6);
                let start_pos = painter.transform;
                draw_bubble(&mut painter, seconds, Vec3::X + Vec3::NEG_Y * 0.6, 1.1);
                painter.transform = start_pos;
                draw_bubble(&mut painter, seconds + 0.5, Vec3::NEG_Y, 1.7);
                painter.transform = start_pos;
                draw_bubble(
                    &mut painter,
                    seconds + PI / 3.0,
                    Vec3::NEG_X + Vec3::NEG_Y,
                    1.3,
                );
                painter.transform = start_pos;
                draw_bubble(
                    &mut painter,
                    seconds + PI / 2.0,
                    Vec3::NEG_X * 0.5 + Vec3::NEG_Y * 1.2,
                    1.9,
                );
                painter.transform = start_pos;
                draw_bubble(
                    &mut painter,
                    seconds + PI / 1.2,
                    Vec3::X * 0.7 + Vec3::NEG_Y * 1.4,
                    1.4,
                );
            }
            // Arc examples
            3 => {
                painter.hollow = false;
                painter.cap = Cap::None;
                painter.color = Color::CRIMSON.pastel() * 0.7;
                painter.translate(diag_vec * 0.8);
                painter.arc(0.8, 0.0, TAU * (2. / 3.));

                painter.hollow = true;
                painter.cap = Cap::Round;
                painter.thickness = 0.4;
                painter.color = Color::CRIMSON.pastel() * 1.4;
                painter.translate(-diag_vec * 1.6);
                painter.arc(0.8, 0.0, TAU * (2. / 3.));
            }
            8 => {
                let start_angle = seconds * 3.0;
                let end_angle = start_angle + PI * (seconds.sin() + 1.) * 0.75 + 0.5 * PI;

                painter.thickness = 0.5;
                painter.hollow = true;
                painter.color = Color::CRIMSON.pastel();
                painter.cap = Cap::None;
                painter.arc(1.5, start_angle, end_angle);
            }
            13 => {
                let meter_fill = ((seconds).sin() + 1.0) / 2.0;
                let meter_size = PI * 1.5;

                let start_angle = -meter_size / 2.0;
                let end_angle = -meter_size / 2.0 + meter_fill * meter_size;

                painter.hollow = true;
                painter.cap = Cap::Round;
                painter.thickness = 0.4;
                painter.color = Color::CRIMSON.pastel() * (1.0 / (0.8 + meter_fill * 0.6));
                painter.arc(1.3, start_angle, end_angle);

                painter.cap = Cap::None;
                painter.thickness = 0.2;
                painter.color = Color::WHITE;
                painter.arc(1.6, start_angle, -start_angle);
                painter.arc(0.8, start_angle, -start_angle);

                let offset = Quat::from_rotation_z(start_angle) * Vec3::Y * 1.1;
                painter.translate(offset);
                painter.arc(0.5, start_angle + PI * 1.5, start_angle + 2.5 * PI);
                painter.translate(-offset);

                painter.translate(Quat::from_rotation_z(-start_angle) * Vec3::Y * 1.1);
                painter.arc(0.5, start_angle + PI, start_angle + 2.0 * PI);
            }
            // Polygon examples
            4 => {
                painter.thickness = 0.4;
                painter.hollow = true;
                painter.color = Color::PURPLE.pastel() * 0.6;
                painter.translate(diag_vec);
                painter.ngon(4., 0.8);

                painter.roundness = 0.1;
                painter.hollow = false;
                painter.color = Color::PURPLE.pastel() * 0.8;
                painter.translate(-Vec3::X * 2.0);
                painter.ngon(3., 0.8);

                painter.hollow = true;
                painter.color = Color::PURPLE.pastel();
                painter.translate(-Vec3::Y * 2.0);
                painter.ngon(5., 0.8);

                painter.roundness = 0.0;
                painter.hollow = false;
                painter.color = Color::PURPLE.pastel() * 1.2;
                painter.translate(Vec3::X * 2.0);
                painter.ngon(6., 0.8);
            }
            9 => {
                painter.hollow = true;
                painter.thickness = 0.5;
                painter.color = Color::PURPLE.pastel();
                painter.roundness = 0.5;
                painter.ngon(3. + (seconds.sin() + 1.) * 3., 1.5);
            }
            14 => {
                const HEX_RADIUS: f32 = 0.35;
                const BOUNDS: f32 = 1.8;

                let down_vec = Quat::from_rotation_z(-PI * 7.0 / 6.0) * Vec3::Y * HEX_RADIUS * 2.0;
                let right_vec = Quat::from_rotation_z(-PI * 5.0 / 6.0) * Vec3::Y * HEX_RADIUS * 2.0;

                fn draw_gon(painter: &mut ShapePainter, origin: Vec3, sides: f32, radius: f32) {
                    let dist = ((painter.transform.translation - origin) / painter.transform.scale)
                        .length();
                    if dist <= BOUNDS {
                        let ratio = 1.0 - f32::max(dist, 0.5) / BOUNDS;
                        painter.color = Color::PURPLE.pastel();
                        painter.color.set_a(ratio);
                        painter.ngon(sides, radius * f32::powf(ratio, 0.2) * 0.8);
                    }
                }

                fn draw_position(painter: &mut ShapePainter, pos: Vec3, right: Vec3) {
                    let origin_tf = painter.transform;

                    painter.translate(pos);
                    draw_gon(painter, origin_tf.translation, 6.0, HEX_RADIUS);

                    let tri_radius = 2.0 * HEX_RADIUS / (2.0 * f32::sqrt(3.0));
                    let normal = right.normalize().cross(Vec3::Z) * tri_radius;

                    painter.translate(right * 0.5 - normal);
                    draw_gon(painter, origin_tf.translation, 3.0, tri_radius);

                    painter.translate(normal * 2.0);
                    painter.rotate_z(PI);
                    draw_gon(painter, origin_tf.translation, 3.0, tri_radius);

                    painter.transform = origin_tf;
                }

                painter.hollow = false;
                painter.roundness = 0.0;

                let offset = down_vec * (seconds % PI) / PI
                    + right_vec * (seconds % (PI / 3.0)) / (PI / 3.0);
                for x in -6..6 {
                    for y in -6..6 {
                        draw_position(
                            &mut painter,
                            down_vec * y as f32 + right_vec * x as f32 + offset,
                            right_vec,
                        );
                    }
                }
            }
            _ => {}
        }
    }
    painter.reset();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Off)
        .add_startup_system(setup)
        .add_system(draw_line)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 16.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn draw_line(mut painter: ShapePainter) {
    // Draw a line
    painter.thickness = 0.025;
    painter.color = Color::WHITE;
    painter.cap = Cap::Round;
    painter.line(Vec3::new(-2.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0));

    // Set the painter to draw two points
    painter.hollow = false;
    painter.thickness = 0.025;
    painter.color = Color::YELLOW;

    // Add point to its left end
    painter.translate(Vec3::new(-2., 0.0, 0.0));
    painter.circle(0.1);
    // Add a point to its right end
    painter.translate(Vec3::new(4., 0.0, 0.0));
    painter.circle(0.1);

    // Move the painter back to the center
    painter.translate(Vec3::new(-2., 0.0, 0.0));

    // Set the painter to draw two circles
    painter.hollow = true;
    painter.thickness = 0.025;

    // Draw a circle with the left point as its center
    painter.translate(Vec3::new(-2.0, 0.0, 0.0));
    painter.circle(4.);

    // Draw a circle with the right point as its center
    painter.translate(Vec3::new(4.0, 0.0, 0.0));
    painter.circle(4.);

    // Move the painter back to the center
    painter.translate(Vec3::new(-2.0, 0.0, 0.0));

    // Get the intersection point between the circles
    let intersection_point: Vec3 =
        Vec3::new(0., (((4.0) * (4.0) - (2.0) * (2.0)) as f32).sqrt(), 0.0);

    // Add an intersection point
    painter.hollow = false;
    painter.thickness = 0.025;
    painter.translate(intersection_point);
    painter.circle(0.1);

    // Move back
    painter.translate(intersection_point * -1.);

    // Set up the painter to draw lines
    painter.thickness = 0.025;
    painter.color = Color::WHITE;
    painter.cap = Cap::Round;

    // Draw two line to the intersection point
    painter.line(Vec3::new(-2., 0., 0.), intersection_point);
    painter.line(Vec3::new(2., 0., 0.), intersection_point);
}
