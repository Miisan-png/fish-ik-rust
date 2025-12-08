use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::util::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Fish {
    pub spine: Chain,
    pub body_width: Vec<f32>,
    pub body_color: Color,
    pub fin_color: Color,
    pub head_pos: Vec2,
    pub target_pos: Vec2,
}

impl Fish {
    pub fn new(origin: Vec2) -> Self {
        let scale = 0.4;
        let body_width = vec![68., 81., 84., 83., 77., 64., 51., 38., 32., 19.]
            .iter()
            .map(|w| w * scale)
            .collect();
        Self {
            spine: Chain::new(origin, 12, 64.0 * scale, PI / 8.0),
            body_width,
            body_color: Color::srgba(1.0, 0.4, 0.4, 0.99), 
            fin_color: Color::srgba(1.0, 0.6, 0.6, 0.99),
            head_pos: origin,
            target_pos: origin,
        }
    }

    fn get_pos(&self, i: usize, angle_offset: f32, length_offset: f32) -> Vec2 {
        let scale = 0.4;
        let angle = self.spine.angles[i] + angle_offset;
        let dist = self.body_width[i] + (length_offset * scale);
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
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for mut fish in fish_query.iter_mut() {
            let head_pos = fish.spine.joints[0];
            let diff = world_position - head_pos;
            let dist = diff.length();
            
            let step = 960.0 * time.delta_seconds();
            let move_dist = step.min(dist);

            if move_dist > 0.0 {
                let target = head_pos + diff.normalize() * move_dist;
                fish.target_pos = target;
                fish.spine.resolve(target);
            }
        }
    }
}

pub fn draw_fish_system(
    mut fish_query: Query<(&Fish, &Children)>,
    mut path_query: Query<&mut Path>,
    body_part_query: Query<&FishBodyPart>,
    pectoral_fin_part_query: Query<&FishPectoralFinPart>,
    ventral_fin_part_query: Query<&FishVentralFinPart>,
    tail_fin_part_query: Query<&FishTailFinPart>,
    front_fin_part_query: Query<&FishFrontFinPart>,
) {
    let scale = 0.4;
    
    for (fish, children) in fish_query.iter_mut() {
        let spine = &fish.spine;
        if spine.joints.is_empty() { continue; }

        let head_to_mid1 = relative_angle_diff(spine.angles[0], spine.angles[6]);
        let head_to_mid2 = relative_angle_diff(spine.angles[0], spine.angles[7]);
        let head_to_tail = head_to_mid1 + relative_angle_diff(spine.angles[6], spine.angles[11]);

        for &child in children.iter() {
            if let Ok(mut path) = path_query.get_mut(child) {
                let mut builder = PathBuilder::new();

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
                         builder.move_to(points[0]);
                         for i in 0..points.len()-3 {
                             add_catmull_rom(&mut builder, points[i], points[i+1], points[i+2], points[i+3]);
                         }
                    }
                    
                } else if pectoral_fin_part_query.get(child).is_ok() {
                     let add_transformed_ellipse = |b: &mut PathBuilder, pos: Vec2, rot: f32, w: f32, h: f32| {
                         let half_w = w / 2.0;
                         let half_h = h / 2.0;
                         let p0 = Vec2::new(half_w, 0.0);
                         let p1 = Vec2::new(0.0, half_h);
                         let p2 = Vec2::new(-half_w, 0.0);
                         let p3 = Vec2::new(0.0, -half_h);
                         
                         let k_w = 0.55228 * half_w;
                         let k_h = 0.55228 * half_h;
                         
                         let t = |v: Vec2| -> Vec2 {
                             let r = Vec2::new(v.x * rot.cos() - v.y * rot.sin(), v.x * rot.sin() + v.y * rot.cos());
                             pos + r
                         };
                         
                         b.move_to(t(p0));
                         b.cubic_bezier_to(t(Vec2::new(half_w, k_h)), t(Vec2::new(k_w, half_h)), t(p1));
                         b.cubic_bezier_to(t(Vec2::new(-k_w, half_h)), t(Vec2::new(-half_w, k_h)), t(p2));
                         b.cubic_bezier_to(t(Vec2::new(-half_w, -k_h)), t(Vec2::new(-k_w, -half_h)), t(p3));
                         b.cubic_bezier_to(t(Vec2::new(k_w, -half_h)), t(Vec2::new(half_w, -k_h)), t(p0));
                         b.close(); 
                     };
                     
                     add_transformed_ellipse(&mut builder, fish.get_pos(3, PI/3.0, 0.0), spine.angles[2] - PI/4.0, 160.0 * scale, 64.0 * scale);
                     add_transformed_ellipse(&mut builder, fish.get_pos(3, -PI/3.0, 0.0), spine.angles[2] + PI/4.0, 160.0 * scale, 64.0 * scale);

                } else if ventral_fin_part_query.get(child).is_ok() {
                     let add_transformed_ellipse = |b: &mut PathBuilder, pos: Vec2, rot: f32, w: f32, h: f32| {
                         let half_w = w / 2.0;
                         let half_h = h / 2.0;
                         let p0 = Vec2::new(half_w, 0.0);
                         let p1 = Vec2::new(0.0, half_h);
                         let p2 = Vec2::new(-half_w, 0.0);
                         let p3 = Vec2::new(0.0, -half_h);
                         
                         let k_w = 0.55228 * half_w;
                         let k_h = 0.55228 * half_h;
                         
                         let t = |v: Vec2| -> Vec2 {
                             let r = Vec2::new(v.x * rot.cos() - v.y * rot.sin(), v.x * rot.sin() + v.y * rot.cos());
                             pos + r
                         };
                         
                         b.move_to(t(p0));
                         b.cubic_bezier_to(t(Vec2::new(half_w, k_h)), t(Vec2::new(k_w, half_h)), t(p1));
                         b.cubic_bezier_to(t(Vec2::new(-k_w, half_h)), t(Vec2::new(-half_w, k_h)), t(p2));
                         b.cubic_bezier_to(t(Vec2::new(-half_w, -k_h)), t(Vec2::new(-k_w, -half_h)), t(p3));
                         b.cubic_bezier_to(t(Vec2::new(k_w, -half_h)), t(Vec2::new(half_w, -k_h)), t(p0));
                         b.close(); 
                     };
                     
                     add_transformed_ellipse(&mut builder, fish.get_pos(7, PI/2.0, 0.0), spine.angles[6] - PI/4.0, 96.0 * scale, 32.0 * scale);
                     add_transformed_ellipse(&mut builder, fish.get_pos(7, -PI/2.0, 0.0), spine.angles[6] + PI/4.0, 96.0 * scale, 32.0 * scale);

                } else if tail_fin_part_query.get(child).is_ok() {
                     let mut tail_pts = Vec::new();
                     for i in 8..12 {
                         let tail_width = 1.5 * head_to_tail * (i as f32 - 8.0).powi(2) * scale;
                         let angle = spine.angles[i] - PI/2.0;
                         tail_pts.push(spine.joints[i] + Vec2::new(angle.cos(), angle.sin()) * tail_width);
                     }
                     for i in (8..12).rev() {
                         let tail_width = ((-13.0f32).max((13.0f32).min(head_to_tail * 6.0))) * scale;
                         let angle = spine.angles[i] + PI/2.0;
                         tail_pts.push(spine.joints[i] + Vec2::new(angle.cos(), angle.sin()) * tail_width);
                     }
                     
                     if tail_pts.len() > 3 {
                        builder.move_to(tail_pts[0]);
                         for i in 0..tail_pts.len()-1 {
                             builder.line_to(tail_pts[i+1]);
                         }
                         builder.close();
                     }

                } else if front_fin_part_query.get(child).is_ok() {
                     let p4 = spine.joints[4];
                     let p5 = spine.joints[5];
                     let p6 = spine.joints[6];
                     let p7 = spine.joints[7];
                     
                     builder.move_to(p4);
                     builder.cubic_bezier_to(p5, p6, p7); 
                     
                     let p6_top = p6 + Vec2::new((spine.angles[6] - PI/2.0).cos(), (spine.angles[6] - PI/2.0).sin()) * head_to_mid2 * 16.0 * scale;
                     let p5_top = p5 + Vec2::new((spine.angles[5] - PI/2.0).cos(), (spine.angles[5] - PI/2.0).sin()) * head_to_mid1 * 16.0 * scale;
                     
                     builder.cubic_bezier_to(p6_top, p5_top, p4);
                }

                *path = builder.build();
            }
        }
    }
}

fn add_catmull_rom(builder: &mut PathBuilder, p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) {
    let tau = 0.5;
    let m1 = (p2 - p0) * tau;
    let m2 = (p3 - p1) * tau;
    
    let cp1 = p1 + m1 / 3.0;
    let cp2 = p2 - m2 / 3.0;
    
    builder.cubic_bezier_to(cp1, cp2, p2);
}