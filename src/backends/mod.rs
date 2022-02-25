pub mod io;
pub mod video;

pub enum ExitType {
    Full, // Used at exit
    SwitchOver // Used when switching backend
}