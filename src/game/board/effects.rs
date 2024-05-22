use bevy::prelude::*;

#[derive(Component)]
pub struct TranslateEffect {
    pub starting_location: Vec2,
    pub ending_location: Vec2,
    t: f32,
    t_mod: f32,
    pub delete_on_complete: bool,
}

impl TranslateEffect {
    pub fn new(starting_location: Vec2, ending_location: Vec2, time: f32) -> Self {
        Self {
            starting_location,
            ending_location,
            t: 0.0,
            t_mod: 1.0 / time,
            delete_on_complete: false,
        }
    }

    pub fn delete_on_complete(mut self) -> Self {
        self.delete_on_complete = true;
        self
    }

    /// Returns a vector if still work is to be performed,
    /// otherwise force effect is complete and component can be removed (after applying ending_location for snapping effect)
    pub fn update(&mut self, dt: f32) -> Option<Vec2> {
        self.t += self.t_mod * dt;
        if self.t >= 1.0 {
            None
        } else {
            Some(self.starting_location.lerp(self.ending_location, self.t))
        }
    }
}

fn handle_translate_effect(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut TranslateEffect)>,
    time: Res<Time>,
) {
    query
        .iter_mut()
        .for_each(|(ent, mut trans, mut translate_effect)| {
            if let Some(new_location) = translate_effect.update(time.delta_seconds()) {
                trans.translation = new_location.extend(trans.translation.z);
            } else {
                if translate_effect.delete_on_complete {
                    commands.entity(ent).despawn_recursive();
                } else {
                    commands.entity(ent).remove::<TranslateEffect>();
                    trans.translation =
                        translate_effect.ending_location.extend(trans.translation.z);
                }
            }
        });
}

/// An instaneous force applied to a sprite, with an elastic centering force
#[derive(Component)]
pub struct ElasticForce {
    /// Initial position
    pub starting_location: Vec2,

    /// Current location
    location: Vec2,
    /// Current velocity
    velocity: Vec2,
    /// Has the system reversed yet?
    reversed: bool,
}

impl ElasticForce {
    /// The coefficient of the elastic force bringing it back to center
    const SPRING_K: f32 = 6.0;

    const DISTANCE_THRESHOLD: f32 = 1.0;

    /// The starting location of the component and the force to instantaneously apply
    pub fn new(starting_location: Vec2, instant_force: Vec2) -> Self {
        Self {
            starting_location,
            location: starting_location,
            velocity: instant_force,
            reversed: false,
        }
    }

    /// Returns a vector if still work is to be performed,
    /// otherwise force effect is complete and component can be removed (after applying starting_location for snapping effect)
    pub fn update(&mut self, dt: f32) -> Option<Vec2> {
        let opposing_force = self.location - self.starting_location;

        let extension: f32 = opposing_force.length_squared();
        // Hooke's Law
        let accel = -Self::SPRING_K * extension;

        let vel_sign = self.velocity.signum();

        // Apply accel vector
        self.velocity += accel * opposing_force * dt;

        // if signs changed after application, it's on it's way back
        if self.velocity.signum() != vel_sign {
            self.reversed = true;
        }

        // Apply location per unit of time
        self.location += self.velocity * dt;

        if extension <= Self::DISTANCE_THRESHOLD && self.reversed {
            None
        } else {
            Some(self.location)
        }
    }
}

fn handle_elastic_force(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut ElasticForce)>,
    time: Res<Time>,
) {
    query
        .iter_mut()
        .for_each(|(ent, mut trans, mut elastic_force)| {
            if let Some(new_location) = elastic_force.update(time.delta_seconds()) {
                trans.translation = new_location.extend(trans.translation.z);
            } else {
                commands.entity(ent).remove::<ElasticForce>();
                trans.translation = elastic_force.starting_location.extend(trans.translation.z);
            }
        });
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_elastic_force, handle_translate_effect));
    }
}
