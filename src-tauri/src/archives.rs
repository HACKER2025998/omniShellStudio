//! Préparation d'archives locales de dossiers sans suivre de liens hors du périmètre choisi.
use crate::{db, Result}; // Les archives vivent dans le répertoire privé de l'application.
use std::path::{Path, PathBuf}; // Les noms internes sont toujours relatifs à la racine partagée.
// Le parcours est borné en profondeur et nombre de fichiers pour éviter une récursion incontrôlée.
fn append(
    // @ligne Renseigne le champ « builder » de la structure utilisée ici de la préparation sûre des dossiers.
    builder: &mut tar::Builder<zstd::Encoder<'_, std::fs::File>>,
    // @ligne Renseigne le champ « root » de la structure utilisée ici de la préparation sûre des dossiers.
    root: &Path,
    // @ligne Renseigne le champ « path » de la structure utilisée ici de la préparation sûre des dossiers.
    path: &Path,
    // @ligne Renseigne le champ « depth » de la structure utilisée ici de la préparation sûre des dossiers.
    depth: usize,
    // @ligne Renseigne le champ « count » de la structure utilisée ici de la préparation sûre des dossiers.
    count: &mut usize,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
) -> Result<()> {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la préparation sûre des dossiers.
    if depth > 64 || *count >= 100_000 {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la préparation sûre des dossiers.
        return Err("Dossier trop profond ou supérieur à 100 000 entrées".into());
    } // Une limite produit un diagnostic explicite.
    let metadata = std::fs::symlink_metadata(path)?; // Observe le lien lui-même, sans suivre une cible externe.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la préparation sûre des dossiers.
    if metadata.file_type().is_symlink() {
        // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la préparation sûre des dossiers.
        return Ok(());
    } // Les liens symboliques ne quittent jamais la racine sélectionnée.
    *count += 1; // Compte aussi les dossiers vides.
    let relative = path.strip_prefix(root.parent().ok_or("Racine de partage invalide")?)?; // L'archive contient un dossier racine nommé.
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la préparation sûre des dossiers.
    if metadata.is_dir() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
        builder.append_dir(relative, path)?;
        // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la préparation sûre des dossiers.
        for entry in std::fs::read_dir(path)? {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            append(builder, root, &entry?.path(), depth + 1, count)?;
        // @ligne Termine cette construction et revient au contexte parent de la préparation sûre des dossiers.
        }
    // @ligne Termine cette construction et revient au contexte parent de la préparation sûre des dossiers.
    }
    // Le flux reste en mémoire constante.
    else if metadata.is_file() {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
        builder.append_path_with_name(path, relative)?;
    } // Ignore sockets, devices et fichiers spéciaux.
    Ok(()) // Les erreurs de lecture interrompent la préparation au lieu de publier une archive tronquée.
// @ligne Termine cette construction et revient au contexte parent de la préparation sûre des dossiers.
}
// La compression s'effectue avant exposition, ce qui rend HTTP Range fiable sur l'archive finale.
pub fn prepare(paths: Vec<String>, room_id: &str) -> Result<Vec<PathBuf>> {
    // @ligne Prépare « cache » pour la prochaine étape de la préparation sûre des dossiers.
    let cache = db::path().with_file_name("archives").join(room_id);
    std::fs::create_dir_all(&cache)?; // Chaque salle possède son espace de préparation.
    let mut output = Vec::new(); // La liste finale devient l'allowlist des téléchargements.
    // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas de la préparation sûre des dossiers.
    for (index, path) in paths.into_iter().enumerate() {
        let path = std::fs::canonicalize(path)?; // Une racine explicitement choisie peut être un lien ; son contenu reste ensuite borné.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la préparation sûre des dossiers.
        if path.is_file() {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            output.push(path);
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            continue;
        } // Un fichier simple n'est jamais recopié inutilement.
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la préparation sûre des dossiers.
        if !path.is_dir() {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la préparation sûre des dossiers.
            return Err("Seuls les fichiers et dossiers ordinaires peuvent être partagés".into());
        // @ligne Termine cette construction et revient au contexte parent de la préparation sûre des dossiers.
        }
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : de la préparation sûre des dossiers.
        if fs2::available_space(&cache)? < 128 * 1024 * 1024 {
            // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la préparation sûre des dossiers.
            return Err("Moins de 128 Mio disponibles pour préparer les dossiers".into());
        } // L'erreur ENOSPC reste également propagée pendant la compression.
        // @ligne Prépare « name » pour la prochaine étape de la préparation sûre des dossiers.
        let name = path
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            .file_name()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            .ok_or("Le partage d'une racine de disque est refusé")?
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            .to_string_lossy();
        let archive = cache.join(format!("{index}-{name}.tar.zst")); // Deux dossiers homonymes ne s'écrasent pas.
        let encoder = zstd::Encoder::new(std::fs::File::create(&archive)?, 3)?; // Compression rapide adaptée à une salle de TP.
        let mut builder = tar::Builder::new(encoder); // tar écrit directement dans le flux zstd.
        append(&mut builder, &path, &path, 0, &mut 0)?; // N'inclut jamais un chemin absolu dans l'archive.
        builder.into_inner()?.finish()?.sync_all()?; // Le fichier n'est publié qu'après finalisation et synchronisation.
        output.push(archive); // Le serveur peut ensuite reprendre ce fichier par Range.
    // @ligne Termine cette construction et revient au contexte parent de la préparation sûre des dossiers.
    }
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel de la préparation sûre des dossiers.
    Ok(output)
// @ligne Termine cette construction et revient au contexte parent de la préparation sûre des dossiers.
}
#[cfg(test)] // Les tests ouvrent réellement l'archive pour contrôler les chemins internes.
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
mod tests {
    // @ligne Importe les outils nécessaires à la mise en œuvre de la préparation sûre des dossiers.
    use super::*;
    // @ligne Applique cette règle de compilation ou de sérialisation à l’élément suivant de la préparation sûre des dossiers.
    #[test]
    // @ligne Déclare « archives », le point d’entrée de cette opération de la préparation sûre des dossiers.
    fn archives_directories_with_relative_entries() {
        // @ligne Prépare « root » pour la prochaine étape de la préparation sûre des dossiers.
        let root = std::env::temp_dir().join(format!("omni-archive-{}", rand::random::<u64>()));
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
        std::fs::create_dir_all(root.join("cours")).unwrap();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
        std::fs::write(root.join("cours/a.txt"), "bonjour").unwrap();
        // @ligne Prépare « file » pour la prochaine étape de la préparation sûre des dossiers.
        let file = std::fs::File::create(root.join("out.tar.zst")).unwrap();
        // @ligne Prépare « builder » pour la prochaine étape de la préparation sûre des dossiers.
        let mut builder = tar::Builder::new(zstd::Encoder::new(file, 1).unwrap());
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
        append(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            &mut builder,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            &root.join("cours"),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            &root.join("cours"),
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            0,
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            &mut 0,
        // @ligne Termine cette construction et revient au contexte parent de la préparation sûre des dossiers.
        )
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
        .unwrap();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
        builder.into_inner().unwrap().finish().unwrap();
        // @ligne Prépare « archive » pour la prochaine étape de la préparation sûre des dossiers.
        let mut archive = tar::Archive::new(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            zstd::Decoder::new(std::fs::File::open(root.join("out.tar.zst")).unwrap()).unwrap(),
        // @ligne Termine cette construction et revient au contexte parent de la préparation sûre des dossiers.
        );
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
        assert!(archive
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            .entries()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            .unwrap()
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
            .all(|e| !e.unwrap().path().unwrap().is_absolute()));
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre de la préparation sûre des dossiers.
        std::fs::remove_dir_all(root).unwrap();
    } // Aucun fichier extérieur n'est nécessaire.
// @ligne Termine cette construction et revient au contexte parent de la préparation sûre des dossiers.
}
