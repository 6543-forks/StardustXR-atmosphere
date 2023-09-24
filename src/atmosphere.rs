use crate::{
	environment::Environment, environment_data::EnvironmentData, play_space::PlaySpaceFinder,
	Config,
};
use mint::Vector3;
use stardust_xr_fusion::{
	client::{Client, FrameInfo, RootHandler},
	core::values::Transform,
	data::PulseSender,
	spatial::Spatial,
	HandlerWrapper,
};
use std::{path::Path, sync::Arc};

#[allow(dead_code)]
pub struct Atmosphere {
	play_space_finder: HandlerWrapper<PulseSender, PlaySpaceFinder>,
	environment: Environment,
	root: Spatial,
}
impl Atmosphere {
	pub fn new(client: &Arc<Client>, config: &Config, env_name: Option<String>) -> Self {
		let data_path = dirs::config_dir()
			.unwrap()
			.join("atmosphere/environments")
			.join(
				env_name
					.as_ref()
					.map(Path::new)
					.unwrap_or(&config.environment),
			)
			.join("env.toml");
		let root = Spatial::create(client.get_root(), Transform::default(), false).unwrap();
		root.set_position(Some(client.get_hmd()), Vector3::from([0.0, 0.0, 0.0]))
			.unwrap();
		root.set_position(None, Vector3::from([0.0, -config.height, 0.0]))
			.unwrap();
		let environment_data = EnvironmentData::load(&data_path).unwrap();
		let environment = Environment::from_data(&root, data_path, environment_data).unwrap();
		dbg!(&environment);
		let play_space_finder = PlaySpaceFinder::new(&client).unwrap();
		Atmosphere {
			root,
			environment,
			play_space_finder,
		}
	}
}

impl RootHandler for Atmosphere {
	fn frame(&mut self, _info: FrameInfo) {
		let Some(play_space) = self.play_space_finder.lock_wrapped().play_space().take() else {return};
		let _ = self.root.set_position(Some(&play_space), [0.0; 3]);
	}
}
