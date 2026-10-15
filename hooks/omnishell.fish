# Chargez ce fichier depuis config.fish avec source /chemin/omnishell.fish.
function _omnishell_postexec --on-event fish_postexec
    # Le statut doit être mémorisé avant tout appel de commande interne.
    set -l previous_status $status
    # Le texte reçu par l'événement inclut les arguments et les retours à la ligne.
    printf '%s' "$argv[1]" | env OMNISHELL_SHELL=fish OMNISHELL_EXIT=$previous_status omnishell capture-hook >/dev/null 2>&1 &
end
