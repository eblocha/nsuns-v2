use nsuns_server::profiles::model::Profile;

#[derive(Debug, Default)]
pub struct ProfileWorld {
    pub profile: Option<Profile>,
    pub profiles: Vec<Profile>,
}

impl ProfileWorld {
    pub fn unwrap_profile(&self) -> &Profile {
        self.profile
            .as_ref()
            .expect("No profile injected into global state")
    }
}
