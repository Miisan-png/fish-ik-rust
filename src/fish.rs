use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::prelude::tess::path::path::Builder;
use bevy_prototype_lyon::prelude::tess::math::Point;
use crate::util::*;
use std::f32::consts::PI;

const SCALE: f32 = 0.4;

#[derive(Component)]
pub struct Fish {
    pub spine: Chain,
    pub body_width: Vec<f32>,
    pub body_color: Color,
    pub fin_color: Color,
    pub target: Vec2,
    pub wander_timer: f32,
}

impl Fish {
    pub fn new(origin: Vec2) -> Self {
        let body_width = vec![68., 81., 84., 83., 77., 64., 51., 38., 32., 19.]
            .iter()
            .map(|w| w * SCALE)
            .collect();
        Self {
            spine: Chain::new(origin, 12, 64.0 * SCALE, PI / 8.0),
            body_width,
            body_color: Color::srgba(1.0, 0.4, 0.4, 0.99),
            fin_color: Color::srgba(1.0, 0.6, 0.6, 0.99),
            target: origin,
            wander_timer: 0.0,
        }
    }

    fn get_pos(&self, i: usize, angle_offset: f32, length_offset: f32) -> Vec2 {
        let angle = self.spine.angles[i] + angle_offset;
        let dist = self.body_width[i] + (length_offset * SCALE);
        self.spine.joints[i] + Vec2::new(angle.cos(), angle.sin()) * dist
    }
}

#[derive(Component)]
pub struct FishBodyPart;

#[derive(Component)]
pub struct FishPectoralFinPart;

#[derive(Component)]
pub struct FishVentralFinPart;

#[derive(Component)]
pub struct FishTailFinPart;

#[derive(Component)]
pub struct FishFrontFinPart;

pub fn update_fish_system(
    mut fish_query: Query<&mut Fish>,
    time: Res<Time>,
) {
    let bounds = Vec2::new(500.0, 350.0);
    
    for mut fish in fish_query.iter_mut() {
        fish.wander_timer -= time.delta_secs();
        
        let head = fish.spine.joints[0];
        let dist_to_target = (fish.target - head).length();
        
        if dist_to_target < 50.0 || fish.wander_timer <= 0.0 {
            let t = time.elapsed_secs();
            fish.target = Vec2::new(
                (t * 0.7).sin() * bounds.x + (t * 1.3).cos() * 100.0,
                (t * 0.5).cos() * bounds.y + (t * 1.1).sin() * 80.0,
            );
            fish.wander_timer = 2.0 + (t * 3.7).sin().abs() * 2.0;
        }
        
        let diff = fish.target - head;
        let speed = 180.0 + (time.elapsed_secs() * 2.0).sin() * 60.0;
        let step = speed * time.delta_secs();
        
        if diff.length() > 1.0 {
            let target = head + diff.normalize() * step.min(diff.length());
            fish.spine.resolve(target);
        }
    }
}

pub fn draw_fish_system(
    mut fish_query: Query<(&Fish, &Children)>,
    mut shape_query: Query<&mut Shape>,
    body_part_query: Query<&FishBodyPart>,
    pectoral_fin_part_query: Query<&FishPectoralFinPart>,
    ventral_fin_part_query: Query<&FishVentralFinPart>,
    tail_fin_part_query: Query<&FishTailFinPart>,
    front_fin_part_query: Query<&FishFrontFinPart>,
) {
    for (fish, children) in fish_query.iter_mut() {
        let spine = &fish.spine;
        if spine.joints.is_empty() { continue; }

        let head_to_mid1 = relative_angle_diff(spine.angles[0], spine.angles[6]);
        let head_to_mid2 = relative_angle_diff(spine.angles[0], spine.angles[7]);
        let head_to_tail = head_to_mid1 + relative_angle_diff(spine.angles[6], spine.angles[11]);

        for child in children.iter() {
            let Ok(mut shape) = shape_query.get_mut(child) else { continue };
            let mut builder = Builder::new();

            if body_part_query.get(child).is_ok() {
                let mut points = Vec::new();
                for i in 0..10 {
                    points.push(fish.get_pos(i, PI / 2.0, 0.0));
                }
                points.push(fish.get_pos(9, PI, 0.0));
                for i in (0..10).rev() {
                    points.push(fish.get_pos(i, -PI / 2.0, 0.0));
                }
                points.push(fish.get_pos(0, -PI / 6.0, 0.0));
                points.push(fish.get_pos(0, 0.0, 4.0));
                points.push(fish.get_pos(0, PI / 6.0, 0.0));
                points.push(fish.get_pos(0, PI / 2.0, 0.0));
                points.push(fish.get_pos(1, PI / 2.0, 0.0));
                points.push(fish.get_pos(2, PI / 2.0, 0.0));

                if points.len() > 3 {
                    builder.begin(to_point(points[0]));
                    for i in 0..points.len() - 3 {
                        add_catmull_rom(&mut builder, points[i], points[i + 1], points[i + 2], points[i + 3]);
                    }
                    builder.end(false);
                }
            } else if pectoral_fin_part_query.get(child).is_ok() {
                add_ellipse(&mut builder, fish.get_pos(3, PI / 3.0, 0.0), spine.angles[2] - PI / 4.0, 160.0 * SCALE, 64.0 * SCALE);
                add_ellipse(&mut builder, fish.get_pos(3, -PI / 3.0, 0.0), spine.angles[2] + PI / 4.0, 160.0 * SCALE, 64.0 * SCALE);
            } else if ventral_fin_part_query.get(child).is_ok() {
                add_ellipse(&mut builder, fish.get_pos(7, PI / 2.0, 0.0), spine.angles[6] - PI / 4.0, 96.0 * SCALE, 32.0 * SCALE);
                add_ellipse(&mut builder, fish.get_pos(7, -PI / 2.0, 0.0), spine.angles[6] + PI / 4.0, 96.0 * SCALE, 32.0 * SCALE);
            } else if tail_fin_part_query.get(child).is_ok() {
                let mut tail_pts = Vec::new();
                for i in 8..12 {
                    let tail_width = 1.5 * head_to_tail * (i as f32 - 8.0).powi(2) * SCALE;
                    let angle = spine.angles[i] - PI / 2.0;
                    tail_pts.push(spine.joints[i] + Vec2::new(angle.cos(), angle.sin()) * tail_width);
                }
                for i in (8..12).rev() {
                    let tail_width = head_to_tail.clamp(-13.0, 13.0) * 6.0 * SCALE;
                    let angle = spine.angles[i] + PI / 2.0;
                    tail_pts.push(spine.joints[i] + Vec2::new(angle.cos(), angle.sin()) * tail_width);
                }

                if tail_pts.len() > 1 {
                    builder.begin(to_point(tail_pts[0]));
                    for pt in &tail_pts[1..] {
                        builder.line_to(to_point(*pt));
                    }
                    builder.close();
                }
            } else if front_fin_part_query.get(child).is_ok() {
                let p4 = spine.joints[4];
                let p5 = spine.joints[5];
                let p6 = spine.joints[6];
                let p7 = spine.joints[7];

                builder.begin(to_point(p4));
                builder.cubic_bezier_to(to_point(p5), to_point(p6), to_point(p7));

                let p6_top = p6 + Vec2::new((spine.angles[6] - PI / 2.0).cos(), (spine.angles[6] - PI / 2.0).sin()) * head_to_mid2 * 16.0 * SCALE;
                let p5_top = p5 + Vec2::new((spine.angles[5] - PI / 2.0).cos(), (spine.angles[5] - PI / 2.0).sin()) * head_to_mid1 * 16.0 * SCALE;

                builder.cubic_bezier_to(to_point(p6_top), to_point(p5_top), to_point(p4));
                builder.end(false);
            }

            shape.path = builder.build();
        }
    }
}

fn to_point(v: Vec2) -> Point {
    Point::new(v.x, v.y)
}

fn add_ellipse(b: &mut Builder, pos: Vec2, rot: f32, w: f32, h: f32) {
    let (hw, hh) = (w / 2.0, h / 2.0);
    let (kw, kh) = (0.55228 * hw, 0.55228 * hh);
    let (cos_r, sin_r) = (rot.cos(), rot.sin());

    let t = |v: Vec2| {
        let r = Vec2::new(v.x * cos_r - v.y * sin_r, v.x * sin_r + v.y * cos_r);
        pos + r
    };

    let p0 = t(Vec2::new(hw, 0.0));
    b.begin(to_point(p0));
    b.cubic_bezier_to(to_point(t(Vec2::new(hw, kh))), to_point(t(Vec2::new(kw, hh))), to_point(t(Vec2::new(0.0, hh))));
    b.cubic_bezier_to(to_point(t(Vec2::new(-kw, hh))), to_point(t(Vec2::new(-hw, kh))), to_point(t(Vec2::new(-hw, 0.0))));
    b.cubic_bezier_to(to_point(t(Vec2::new(-hw, -kh))), to_point(t(Vec2::new(-kw, -hh))), to_point(t(Vec2::new(0.0, -hh))));
    b.cubic_bezier_to(to_point(t(Vec2::new(kw, -hh))), to_point(t(Vec2::new(hw, -kh))), to_point(p0));
    b.close();
}

fn add_catmull_rom(builder: &mut Builder, p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) {
    let tau = 0.5;
    let m1 = (p2 - p0) * tau;
    let m2 = (p3 - p1) * tau;
    builder.cubic_bezier_to(to_point(p1 + m1 / 3.0), to_point(p2 - m2 / 3.0), to_point(p2));
}
