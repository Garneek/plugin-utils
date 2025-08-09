extern crate nih_plug;

use std::sync::Arc;

use nih_plug::params::Params;
use nih_plug::plugin::ProcessStatus;
use nih_plug::prelude::AudioIOLayout;
use nih_plug::prelude::Plugin;

mod params;
use nih_plug_egui::EguiState;
use params::EguiUtilsExampleParams;

mod editor;

pub struct EguiUtilsExamplePlugin {
    params: Arc<EguiUtilsExampleParams>,
    editor_state: Arc<EguiState>,
}

impl Default for EguiUtilsExamplePlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(EguiUtilsExampleParams::default()),
            editor_state: editor::default_state(),
        }
    }
}

impl Plugin for EguiUtilsExamplePlugin {
    const NAME: &'static str = "EguiUtilsExamplePlugin";
    const VENDOR: &'static str = "";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = "";
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    const AUDIO_IO_LAYOUTS: &'static [nih_plug::prelude::AudioIOLayout] =
        &[AudioIOLayout::const_default()];
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut nih_plug::prelude::Buffer,
        _aux: &mut nih_plug::prelude::AuxiliaryBuffers,
        _context: &mut impl nih_plug::prelude::ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }

    fn editor(
        &mut self,
        _async_executor: nih_plug::prelude::AsyncExecutor<Self>,
    ) -> Option<Box<dyn nih_plug::prelude::Editor>> {
        editor::create(self.params.clone(), self.editor_state.clone())
    }
}
