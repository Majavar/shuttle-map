use noise::{MultiFractal, NoiseFn, Seedable};

pub struct Fractal<N>
where
    N: NoiseFn<[f64; 2]>,
{
    octaves: usize,
    frequency: f64,
    lacunarity: f64,
    persistence: f64,

    noise: N,
}

impl<N> MultiFractal for Fractal<N>
where
    N: NoiseFn<[f64; 2]>,
{
    fn set_octaves(self, octaves: usize) -> Self {
        Self { octaves, ..self }
    }

    fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }

    fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self { lacunarity, ..self }
    }

    fn set_persistence(self, persistence: f64) -> Self {
        Self {
            persistence,
            ..self
        }
    }
}

impl<N> Seedable for Fractal<N>
where
    N: NoiseFn<[f64; 2]> + Seedable,
{
    fn set_seed(self, seed: u32) -> Self {
        Self {
            noise: self.noise.set_seed(seed),
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.noise.seed()
    }
}

impl<N> NoiseFn<[f64; 2]> for Fractal<N>
where
    N: NoiseFn<[f64; 2]>,
{
    fn get(&self, point: [f64; 2]) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;

        for _ in 0..self.octaves {
            value += self.noise.get([point[0] * frequency, point[1] * frequency]) * amplitude;

            frequency *= self.lacunarity;
            amplitude *= self.persistence;
        }

        let scale = 2.0 - self.persistence.powi(self.octaves as i32 -1);
        value / scale
    }
}

impl<N> Fractal<N>
where
    N: NoiseFn<[f64; 2]> + Default,
{
    pub const DEFAULT_OCTAVE_COUNT: usize = 10;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;
    pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;
    pub const DEFAULT_PERSISTENCE: f64 = 0.5;

    pub fn new() -> Self {
        Self {
            octaves: Self::DEFAULT_OCTAVE_COUNT,
            frequency: Self::DEFAULT_FREQUENCY,
            lacunarity: Self::DEFAULT_LACUNARITY,
            persistence: Self::DEFAULT_PERSISTENCE,
            noise: Default::default(),
        }
    }
}

impl<N> Default for Fractal<N>
where
    N: NoiseFn<[f64; 2]> + Default,
{
    fn default() -> Self {
        Self::new()
    }
}
