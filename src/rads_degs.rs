use core::f32::consts::PI;

macro_rules! declare {
    (@cross $trait:ident $little:ident $is_not:ident $big:ident $op:ident $expr:expr) => {
        auto trait $is_not {}

        impl !$is_not for $op {}

        impl const $trait for $big {
            type Output = f32;

            ///
            /// Convert radians to degrees.
            /// See <a href="https://en.wikipedia.org/wiki/Radian">Radian</a> and <a href="https://en.wikipedia.org/wiki/Degree_(angle)">Degree</a>.
            ///
            fn $little(self) -> $op {
                const F: f32 = $expr;
                $op(self.0 * F)
            }
        }
    };

    (@base $big:ident $little:ident $trait:ident $is_not:ident) => {
        #[derive(Copy, Clone)]
        pub struct $big(f32);

        impl const From<f32> for $big {
            fn from(x: f32) -> Self {
                Self(x)
            }
        }

        impl $big {
            pub const fn get(self) -> f32 {
                self.0
            }
        }

        #[const_trait]
        pub trait $trait {
            type Output: Copy;

            fn $little(self) -> $big;
        }

        impl const $trait for f32 {
            type Output = Self;

            fn $little(self) -> $big {
                $big::from(self)
            }
        }
    };
}

declare!(@base Degrees degrees ToDegrees IsNotDegrees);
declare!(@base Radians radians ToRadians IsNotRadians);
declare!(@cross ToRadians radians IsNotDegrees Degrees Radians PI / 180.0);
declare!(@cross ToDegrees degrees IsNotRadians Radians Degrees 180.0 / PI);
