use crate::RcAny;
use crate::Wrc;
use config::Config;
use config::Environment;
use config::File;
use config::FileFormat;
use lazy_static::lazy_static;
use regex::Regex;
use std::any::type_name;
use std::any::TypeId;
use std::collections::HashMap;
use std::env;
use std::env::args;
use std::marker::PhantomData;

pub mod profiles {
    pub struct Default;
    pub struct Dev;
    pub struct Test;
}

pub trait Component {
    fn __inexor_rgf_core_di_create<P>(container: &mut Container<P>) -> Self;
    fn __inexor_rgf_core_di_inject_deferred<P>(container: &mut Container<P>, component: &Self);
}

pub trait Provider<T: ?Sized> {
    type Impl;
    fn get(&mut self) -> Wrc<Self::Impl>;
    fn create(&mut self) -> Self::Impl;

    fn get_ref(&mut self) -> &Self::Impl {
        // Value under RC is still stored in container, so it can be safely returned as a reference
        // that has the same life as container reference
        unsafe { Wrc::as_ptr(&Self::get(self)).as_ref().unwrap() }
    }
    fn create_boxed(&mut self) -> Box<Self::Impl> {
        Box::new(Self::create(self))
    }
}

pub struct Container<P> {
    profile: PhantomData<P>,
    pub config: Config,
    pub components: HashMap<TypeId, RcAny>,
}

impl<P> Container<P> {
    pub fn new() -> Container<P> {
        let mut builder = Config::builder().add_source(File::new("config/default", FileFormat::Toml).required(false));
        let profile = profile_name::<P>();
        if profile.ne(&"default".to_string()) {
            builder = builder.add_source(File::with_name(&format!("config/{}", profile)).required(false));
        }
        builder = builder.add_source(Environment::with_prefix("INEXOR"));

        Container {
            config: builder.build().expect("Failed to read default config file"),
            profile: PhantomData::<P>,
            components: HashMap::new(),
        }
    }
}

lazy_static! {
    pub static ref APP_PROFILE: String = parse_profile();
}

fn parse_profile() -> String {
    let builder = Config::builder().add_source(File::with_name("config/default").required(false));

    let profile_arg = args().position(|arg| arg.as_str() == "--profile").and_then(|arg_pos| args().nth(arg_pos + 1));

    let config = builder.build().expect("Failed to parse profile");
    let parsed_profile = profile_arg
        .or(env::var("PROFILE").ok())
        .or(config.get_string("profile").ok())
        .unwrap_or("default".to_string());

    log::info!("Using profile: {}", parsed_profile);

    parsed_profile
}

pub fn parse_args() -> Config {
    let mut builder = Config::builder();

    let mut args = args().peekable();
    loop {
        let arg = args.next();
        if arg.is_some() {
            let arg = arg.unwrap();
            if arg.starts_with("--") {
                let value = args.peek();
                if value.is_none() || value.unwrap().starts_with("--") {
                    builder = builder.set_override(&arg[2..], true).expect("Failed to parse arg");
                } else {
                    let arg = args.next().unwrap();
                    builder = builder.set_override(&arg[2..], args.next().unwrap()).expect("Failed to parse arg");
                }
            }
        } else {
            break;
        }
    }

    builder.build().expect("Failed to parse args")
}

pub fn profile_name<T>() -> String {
    let profile_type_name = type_name::<T>().to_lowercase();

    Regex::new(r".*::").unwrap().replace(profile_type_name.as_str(), "").to_string()
}
