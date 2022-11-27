// ただの Mutex の Sample なのでやってることに特に意味はない

use shaku::{module, Component, Interface, HasComponent};
use std::sync::{Arc, Mutex};

trait Logger: Interface {
    fn log(&self, content: &str);
}

trait DateLogger: Interface {
    fn log_date(&self);
}

#[derive(Component)]
#[shaku(interface = Logger)]
struct LoggerImpl {
    #[shaku(default = Mutex::default())]
    resource: Mutex<Vec<String>>,
}

impl Logger for LoggerImpl {
    fn log(&self, content: &str) {
        self.resource.lock().unwrap().push(content.to_string());
        println!("{}", content);
    }
}

#[derive(Component)]
#[shaku(interface = DateLogger)]
struct DateLoggerImpl {
    #[shaku(inject)]
    logger: Arc<dyn Logger>,
    #[shaku(default)]
    today: String,
    #[shaku(default)]
    year: usize,
}

impl DateLogger for DateLoggerImpl {
    fn log_date(&self) {
        self.logger.log(&format!("Today is {}, {}", self.today, self.year));
    }
}

module! {
    MyModule {
        components = [LoggerImpl, DateLoggerImpl],
        providers = []
    }
}

fn main() {
    let module = MyModule::builder()
        .with_component_parameters::<DateLoggerImpl>(DateLoggerImplParameters {
            today: "July 25".to_string(),
            year: 2001,
        })
        .build();

    let date_logger: &dyn DateLogger = module.resolve_ref();
    date_logger.log_date();
}
