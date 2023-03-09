#[macro_export]
macro_rules! inject {
    ($comp:path: $($profile:path),*) => {
        {
            $(
                if profile_name::<$profile>().eq(&inexor_rgf_core_di::APP_PROFILE.as_str()) {
                    inexor_rgf_core_di::Provider::<$comp>::create(&mut inexor_rgf_core_di::Container::<$profile>::new())
                } else
            )*
            { inexor_rgf_core_di::Provider::<$comp>::create(&mut inexor_rgf_core_di::Container::<inexor_rgf_core_di::profiles::Default>::new()) }
        }
    }
}

#[macro_export]
macro_rules! wrap {
    ($wrapped_type:path as $wrapper_name:ident) => {
        pub struct $wrapper_name($wrapped_type);
        impl Deref for $wrapper_name {
            type Target = $wrapped_type;
            fn deref(&self) -> &Self::Target {
                return &self.0;
            }
        }
    };
}
