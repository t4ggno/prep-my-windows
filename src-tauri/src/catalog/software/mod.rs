use crate::models::{
    PolicyDefinition, ScheduledTaskDefinition, SoftwareDefinition, SoftwareMarker,
};

mod adobe;
mod browsers;
mod hardware;
mod microsoft;
mod startup;

fn bundle(
    id: &'static str,
    name: &'static str,
    markers: &'static [SoftwareMarker],
    policies: Vec<PolicyDefinition>,
    tasks: &[(&'static str, &'static str, &'static str)],
) -> SoftwareDefinition {
    SoftwareDefinition {
        id,
        name,
        markers,
        policies: policies
            .into_iter()
            .map(PolicyDefinition::off_by_default)
            .collect(),
        tasks: tasks
            .iter()
            .map(|&(id, name, path)| ScheduledTaskDefinition {
                id,
                name,
                path,
                default_enabled: false,
            })
            .collect(),
    }
}

pub fn software() -> Vec<SoftwareDefinition> {
    vec![
        hardware::amd(),
        hardware::nvidia(),
        hardware::lenovo(),
        browsers::chrome(),
        browsers::firefox(),
        browsers::brave(),
        adobe::acrobat(),
        adobe::creative_cloud(),
        microsoft::office(),
        microsoft::visual_studio(),
        microsoft::visual_studio_code(),
        startup::steam(),
        startup::discord(),
        startup::spotify(),
        startup::epic_games(),
        startup::gog_galaxy(),
        startup::logitech(),
    ]
}
