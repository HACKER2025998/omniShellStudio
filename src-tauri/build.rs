// Le moteur CLI se compile sans les bibliothèques graphiques du système.
fn main() {
    // Tauri génère ses ressources uniquement lors d'une compilation desktop explicite.
    #[cfg(feature = "desktop")]
    tauri_build::build();
}
