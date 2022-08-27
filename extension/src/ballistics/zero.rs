use crate::common::Vector3;

use super::{drag::calculate_retard, BallisticModel, GRAVITY};

pub fn replicate_vanilla_zero(zero_range: f64, muzzle_velocity: f64, air_friction: f64) -> f64 {
    let max_delta_time = 0.05;
    let mut time = 0.0;
    let mut current_shot_position = Vector3::default();
    let mut current_shot_velocity = Vector3::new(muzzle_velocity, 0.0, 0.0);

    while time < 8.0 {
        let dist_left = zero_range - current_shot_position.x();
        let traveled = current_shot_velocity.x() * max_delta_time;
        if dist_left < traveled {
            let delta_time = dist_left / current_shot_velocity.x();
            current_shot_velocity -= Vector3::new(0.0, GRAVITY * delta_time, 0.0);
            current_shot_position += current_shot_velocity * delta_time;
            break;
        } else {
            let delta_time = max_delta_time;
            current_shot_position += current_shot_velocity * delta_time;
            time += delta_time;
            current_shot_velocity += current_shot_velocity
                * (current_shot_velocity.magnitude() * air_friction * delta_time);
            current_shot_velocity -= Vector3::new(0.0, GRAVITY * delta_time, 0.0);
        }
    }
    (-(current_shot_position.y() / zero_range).atan()).to_degrees()
}

const SPEED_OF_SOUND_AT_15C: f64 = 340.275;

pub fn calculate_zero(
    zero_range: f64,
    muzzle_velocity: f64,
    bore_height: f64,
    ballistic_model: BallisticModel,
) -> f64 {
    let mut zero_angle = 0.0f64;
    let delta_time = 1.0 / 100.0f64.max(zero_range);

    for _ in 0..10 {
        let mut lx = 0.0;
        let mut ly = 0.0;

        let mut px = 0.0;
        let mut py = -bore_height / 100.0;

        let gx = zero_angle.sin() * -GRAVITY;
        let gy = zero_angle.cos() * -GRAVITY;

        let mut vx = zero_angle.cos() * muzzle_velocity;
        let mut vy = zero_angle.sin() * muzzle_velocity;

        let mut tof = 0.0;

        while tof < 8.0 && px < zero_range {
            lx = px;
            ly = py;

            let v = vx.hypot(vy);

            let (ax, ay) = match &ballistic_model {
                BallisticModel::Vanilla(air_friction) => {
                    ((vx * v * air_friction) + gx, (vy * v * air_friction) + gy)
                }
                BallisticModel::Advanced(advanced) => {
                    let retard = calculate_retard(
                        advanced.drag_function(),
                        super::atmosphere::calculate_atmospheric_correction(
                            advanced.ballistic_coefficient(),
                            advanced.temperature(),
                            advanced.pressure(),
                            advanced.relative_humidity(),
                            advanced.atmosphere_model(),
                        ),
                        v,
                        SPEED_OF_SOUND_AT_15C,
                    );
                    ((vx / v).mul_add(-retard, gx), (vy / v).mul_add(-retard, gy))
                }
            };

            px += vx * delta_time * 0.5;
            py += vy * delta_time * 0.5;
            vx += ax * delta_time;
            vy += ay * delta_time;
            px += vx * delta_time * 0.5;
            py += vy * delta_time * 0.5;

            tof += delta_time;
        }
        let y = ly + (zero_range - lx) * (py - ly) / (px - lx);
        let offset = -(y / zero_range).atan();
        zero_angle += offset;

        if offset.abs() < 0.00001 {
            break;
        }
    }
    zero_angle.to_degrees()
}

#[cfg(test)]
mod tests {
    use crate::ballistics::{
        AdvancedBallistics, AtmosphereModel, BallisticModel, DragFunction, Temperature,
    };

    #[test]
    fn replicate_vanilla_zero() {
        assert_eq!(
            super::replicate_vanilla_zero(200.0, 89.0, 0.3),
            0.16467323756834434 // old ace: 0.164672
        )
    }

    #[test]
    fn calc_zero_vanilla() {
        assert_eq!(
            super::calculate_zero(200.0, 89.0, 1.5, BallisticModel::Vanilla(0.3)),
            0.13281857110203227 // old ace: 0.132818
        )
    }

    #[test]
    fn calc_zero_advanced() {
        assert_eq!(
            super::calculate_zero(
                200.0,
                89.0,
                1.5,
                BallisticModel::Advanced(AdvancedBallistics {
                    ballistic_coefficient: 0.583,
                    temperature: Temperature::new_celsius(15.0),
                    pressure: 1005.0,
                    relative_humidity: 0.0,
                    atmosphere_model: AtmosphereModel::Icao,
                    drag_function: DragFunction::G1,
                })
            ),
            7.50985540380535 // old ace: 7.51363
        )
    }
}
