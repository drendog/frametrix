mod config;
mod device;
mod error;

use anyhow::Result;
use config::Config;
use device::matrix_ops;
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use std::{fs, path::PathBuf, rc::Rc};
use tracing::{error, info};

slint::include_modules!();

struct AppState {
    config: Config,
    main_window: MainWindow,
}

impl AppState {
    fn new() -> Result<Self> {
        let main_window = MainWindow::new()?;
        let config = Self::load_config()?;
        Ok(Self {
            config,
            main_window,
        })
    }

    // TODO: Find config on ~/.config/led-matrix/config.toml or create it
    fn load_config() -> Result<Config> {
        let config_path = PathBuf::from("config.toml");
        Ok(if let Ok(config_data) = fs::read(&config_path) {
            toml_edit::de::from_slice(&config_data)?
        } else {
            Config::default()
        })
    }

    fn initialize(&self) -> Result<()> {
        self.setup_matrix()?;
        self.setup_patterns()?;
        self.setup_handlers()?;
        self.main_window.invoke_refresh_devices();
        Ok(())
    }

    fn setup_matrix(&self) -> Result<()> {
        let matrix = device::util::create_initial_matrix();
        let matrix_model = device::util::create_matrix_model(&matrix);
        self.main_window
            .global::<MatrixState>()
            .set_matrix(matrix_model);
        self.setup_matrix_handler()
    }

    fn setup_matrix_handler(&self) -> Result<()> {
        let matrix_handler = self.main_window.global::<MatrixState>();
        matrix_handler.on_set_matrix(|matrix_model: ModelRc<ModelRc<bool>>| {
            let matrix_array = device::util::convert_model_to_array(&matrix_model);
            if let Ok(devices) = device::discover_devices() {
                for device in devices {
                    if let Err(e) = device.render_matrix(&matrix_array) {
                        error!(
                            "Failed to render matrix on device {}: {}",
                            device.port_name(),
                            e
                        );
                    }
                }
            }
        });
        Ok(())
    }

    fn setup_patterns(&self) -> Result<()> {
        let patterns_model = Rc::new(VecModel::default());

        for pattern in self.config.patterns.values() {
            let mut item = StandardListViewItem::default();
            item.text = SharedString::from(&pattern.label);
            info!("Adding pattern: {}", &pattern.label);
            patterns_model.push(item);
        }

        self.main_window
            .set_standard_patterns(ModelRc::from(patterns_model));
        Ok(())
    }

    fn setup_handlers(&self) -> Result<()> {
        let window_weak = self.main_window.as_weak();
        let config = self.config.clone();

        self.main_window
            .on_refresh_devices(move || matrix_ops::refresh_devices(&window_weak));

        self.main_window.on_send_pattern(move |pattern_idx| {
            matrix_ops::send_pattern_to_devices(pattern_idx, &config)
        });

        self.main_window
            .on_send_brightness(matrix_ops::send_brightness_to_devices);

        Ok(())
    }

    fn run(self) -> Result<()> {
        Ok(self.main_window.run()?)
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let app = AppState::new()?;
    app.initialize()?;
    app.run()
}
