use bevy::asset::Asset;
use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::ecs::{IntoQuerySystem, ResMut};
use bevy::prelude::*;

/// Adds "asset count" diagnostic to an App
#[derive(Default)]
pub struct AssetCountDiagnosticsPlugin<T: Asset> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Asset> Plugin for AssetCountDiagnosticsPlugin<T> {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_startup_system(Self::setup_system.system())
            .add_system(Self::diagnostic_system.system());
    }
}

impl<T: Asset> AssetCountDiagnosticsPlugin<T> {
    pub fn diagnostic_id() -> DiagnosticId {
        DiagnosticId(T::TYPE_UUID)
    }

    pub fn setup_system(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(
            Self::diagnostic_id(),
            &format!("asset_count {}", T::TYPE_UUID),
            20,
        ));
    }

    pub fn diagnostic_system(mut diagnostics: ResMut<Diagnostics>, assets: Res<Assets<T>>) {
        diagnostics.add_measurement(Self::diagnostic_id(), assets.len() as f64);
    }
}