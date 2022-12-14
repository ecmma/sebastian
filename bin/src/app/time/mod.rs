use sebastian_core::time::TimeTableConfig;

use super::AppConfig;

/// Access your course's timetable.
#[derive(clap::Parser, Clone, Debug)]
pub struct Time {
    #[clap(subcommand)]
    pub action: TimeTableAction,

    #[clap(skip)]
    pub user_config: Option<TimeTableConfig>,
}

#[derive(clap::Parser, Clone, Debug)]
pub enum TimeTableAction {
    /// Initialize the timetable configuration.
    Init,

    /// Show the timetable.
    Show,
}


#[allow(unused)]
impl Time {
    pub(crate) async fn run(
        &mut self,
        app_config: AppConfig,
        user_config: Option<TimeTableConfig>,
    ) -> anyhow::Result<(AppConfig, TimeTableConfig)> {
        todo!()
    }
}
