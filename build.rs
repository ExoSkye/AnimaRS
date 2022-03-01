fn main() {
    #[cfg(feature = "graphical-tests")]
    cargo_emit::warning!("You have enabled the graphical tests. These tests require that the computer is capable of OpenGL, on platforms that don't, they will error");
}