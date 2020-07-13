/// Different ways to calculate the distance between 2 points.
pub enum DistanceCalculator {
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Chebyshev_distance)
    Chebyshev,
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Manhattan_distance)
    Manhattan,
}

impl DistanceCalculator {
    /// Returns the distance between 2 points in 2d
    /// ```
    ///# use rust_roguelike_core::math::distance::DistanceCalculator;
    /// assert_eq!(DistanceCalculator::Chebyshev.calculate_2d(10, 20, 14, 29), 9);
    /// assert_eq!(DistanceCalculator::Chebyshev.calculate_2d(14, 29, 10, 20), 9);
    /// assert_eq!(DistanceCalculator::Manhattan.calculate_2d(10, 20, 14, 29), 13);
    /// assert_eq!(DistanceCalculator::Manhattan.calculate_2d(14, 29, 10, 20), 13);
    /// ```
    pub fn calculate_2d(&self, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> u32 {
        (match self {
            DistanceCalculator::Chebyshev => (to_x - from_x).abs().max((to_y - from_y).abs()),
            DistanceCalculator::Manhattan => (to_x - from_x).abs() + (to_y - from_y).abs(),
        }) as u32
    }
}
