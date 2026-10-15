# Chargez ce fichier à la fin de .zshrc : source /chemin/omnishell.zsh.
# Les hooks sont enregistrés sans remplacer ceux des autres outils.
autoload -Uz add-zsh-hook
# preexec conserve la commande exacte et respecte l'espace initial de confidentialité.
_omnishell_preexec() { _omnishell_pending="$1"; }
# precmd reçoit le code de sortie avant qu'une autre commande ne le remplace.
_omnishell_precmd() {
  local previous_status=$?
  # Une première invite ou une commande déjà capturée ne produit aucune écriture.
  [[ -n "${_omnishell_pending:-}" ]] || return 0
  # Le processus indépendant ne retarde pas le prochain prompt.
  printf '%s' "$_omnishell_pending" | OMNISHELL_SHELL=zsh OMNISHELL_EXIT="$previous_status" command omnishell capture-hook >/dev/null 2>&1 &!
  # Le tampon ne doit pas être repris lors d'une simple réaffichage d'invite.
  unset _omnishell_pending
}
# Réimporter le fichier remplace proprement les callbacks homonymes.
add-zsh-hook -d preexec _omnishell_preexec 2>/dev/null
add-zsh-hook -d precmd _omnishell_precmd 2>/dev/null
# Ces deux callbacks ne modifient ni la commande ni son résultat.
add-zsh-hook preexec _omnishell_preexec
add-zsh-hook precmd _omnishell_precmd
