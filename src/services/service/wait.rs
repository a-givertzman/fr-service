use log::{error, info};
use testing::stuff::wait::WaitTread;

use super::service_handles::ServiceHandles;

impl WaitTread for ServiceHandles {
    fn wait(self) -> Result<(), Box<dyn std::any::Any + Send>> {
        let mut errors  = vec![];
        for (id, handle) in self {
            info!("Waiting for thread: '{}'...", id);
            let r = handle.join();
            match r {
                Ok(_) => {
                    info!("Waiting for thread: '{}' - finished", id);
                },
                Err(err) => {
                    error!("Waiting for thread '{}' - error: {:#?}", id, err);
                    errors.push(err);
                },
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Box::new(errors))
        }
    }
}