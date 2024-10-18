use rand::distributions::uniform::SampleRange;
use rand::Rng;

pub fn get_random_position_around<R>(pos: (f32, f32), dist: R) -> (f32, f32)
where
    R: SampleRange<f32>,
{
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let dist = rng.gen_range(dist);

    (pos.0 + angle.cos() * dist, pos.1 + angle.sin() * dist)
}
