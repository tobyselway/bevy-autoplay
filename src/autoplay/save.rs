use std::{
    error,
    fs::{create_dir_all, File},
    io::{BufReader, Write},
    path::Path,
};

use bevy::{log::info, prelude::*};
use rmp_serde::Serializer;
use serde::Serialize;

use super::{
    session::Session, storable::Storable, AutoplayState, LoadFromFile, LoadFromFileAndPlay,
    SaveToFile,
};

impl Storable for Session {
    fn save(&self, filename: &str) -> Result<(), Box<dyn error::Error>> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf))?;
        let path = Path::new(filename);
        let prefix = path.parent().unwrap();
        create_dir_all(prefix)?;
        let mut file = File::create(filename)?;
        file.write_all(&buf)?;
        Ok(())
    }

    fn load(&mut self, filename: &str) -> Result<(), Box<dyn error::Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        *self = rmp_serde::from_read(reader)?;
        Ok(())
    }
}

pub fn save_recording(mut ev_save: EventReader<SaveToFile>, session: Res<Session>) {
    for ev in ev_save.read() {
        let filename = ev.0.clone();
        session.save(&filename).unwrap();
        info!("Saved recording to {}", filename);
    }
}

pub fn load_recording(mut ev_load: EventReader<LoadFromFile>, mut session: ResMut<Session>) {
    for ev in ev_load.read() {
        let filename = ev.0.clone();
        session.load(&filename).unwrap();
        info!("Loaded recording {}", filename);
    }
}

pub fn load_recording_and_play(
    mut ev_load: EventReader<LoadFromFileAndPlay>,
    mut session: ResMut<Session>,
    mut next_session_state: ResMut<NextState<AutoplayState>>,
) {
    for ev in ev_load.read() {
        let filename = ev.0.clone();
        session.load(&filename).unwrap();
        info!("Loaded recording {}", filename);
        next_session_state.set(AutoplayState::Playing);
    }
}
