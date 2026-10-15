# Chargez ce fichier depuis .bashrc après avoir placé omnishell dans PATH.
_omnishell_prompt() {
  local previous_status=$?
  local entry
  # history n'est lu qu'après exécution ; aucune DEBUG trap intrusive n'est installée.
  entry="$(HISTTIMEFORMAT= builtin history 1)"
  # L'identifiant distingue deux exécutions consécutives de la même commande.
  if [[ "$entry" =~ ^[[:space:]]*([0-9]+)[[:space:]]+(.*)$ ]]; then
    # Une commande exclue de l'historique n'est pas récupérée par un autre moyen.
    [[ "${BASH_REMATCH[1]}" != "${_omnishell_last_id:-}" ]] || return 0
    _omnishell_last_id="${BASH_REMATCH[1]}"
    # La commande passe par stdin et n'apparaît pas dans la liste des processus.
    printf '%s' "${BASH_REMATCH[2]}" | OMNISHELL_SHELL=bash OMNISHELL_EXIT="$previous_status" command omnishell capture-hook >/dev/null 2>&1 &
  fi
}
# Bash accepte un tableau de callbacks ; la chaîne existante est conservée en premier.
if [[ "${PROMPT_COMMAND[*]:-}" != *'_omnishell_prompt'* ]]; then
  # Transformer une chaîne en tableau garde la commande initiale comme premier élément.
  PROMPT_COMMAND=("${PROMPT_COMMAND[@]}" _omnishell_prompt)
fi
