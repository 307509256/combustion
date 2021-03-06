//! Contains all the default values for Scene objects

use std::collections::HashMap;

use nalgebra::*;

use common::traits::DefaultName;

use common::color::Color;

use super::*;

impl DefaultName for Node {
    fn default_name() -> String {
        "Untitled Node".to_string()
    }
}

impl DefaultName for Scene {
    fn default_name() -> String {
        "Untitled Scene".to_string()
    }
}

impl DefaultName for Light {
    fn default_name() -> String {
        "Untitled Light".to_string()
    }
}

impl DefaultName for Material {
    fn default_name() -> String {
        "Untitled Material".to_string()
    }
}

/// Associated functions for getting default values for material structures
pub trait DefaultMaterial {}

impl DefaultMaterial for Material {}

/// Associated functions for getting default values for light structures
pub trait DefaultLight {
    /// Returns the default value for zdistance
    #[inline(always)]
    fn default_zdistance() -> (f32, f32) {
        (0.0, 1000.0)
    }

    /// Returns the default value for position
    #[inline(always)]
    fn default_position() -> Point3<f32> {
        Point3::new(0.0, 1.0, 0.0)
    }

    /// Returns the default value for direction
    #[inline(always)]
    fn default_direction() -> Vector3<f32> {
        Vector3::new(0.0, -1.0, 0.0)
    }

    /// Returns the default value for kind
    #[inline(always)]
    fn default_kind() -> LightKind {
        LightKind::Spotlight
    }

    /// Returns the default value for effect_radius
    #[inline(always)]
    fn default_effect_radius() -> f32 {
        1000.0
    }

    /// Returns the default value for inner_cone
    #[inline(always)]
    fn default_inner_cone() -> f32 {
        0.0
    }

    /// Returns the default value for outer_cone
    #[inline(always)]
    fn default_outer_cone() -> f32 {
        15.0
    }

    /// Returns the default value for intensity
    #[inline(always)]
    fn default_intensity() -> f32 {
        1.0
    }
}

impl DefaultLight for Light {}

impl Default for Light {
    fn default() -> Light {
        Light {
            name: Light::default_name(),
            zdistance: Light::default_zdistance(),
            position: Light::default_position(),
            direction: Light::default_direction(),
            color: Color::white(),
            ambient: Color::none(),
            kind: Light::default_kind(),
            effect_radius: Light::default_effect_radius(),
            inner_cone: Light::default_inner_cone(),
            outer_cone: Light::default_outer_cone(),
            intensity: Light::default_intensity(),
            properties: HashMap::default(),
        }
    }
}

impl Default for Material {
    fn default() -> Material {
        Material {
            name: Material::default_name()
        }
    }
}