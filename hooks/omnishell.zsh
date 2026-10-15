# Chargez ce fichier à la fin de .zshrc : source /chemin/omnishell.zsh.
# Les hooks sont enregistrés sans remplacer ceux des autres outils.
autoload -Uz add-zsh-hook
# Le module fournit une horloge précise sans lancer un processus date.
zmodload zsh/datetime
# preexec conserve la commande exacte et respecte l'espace initial de confidentialité.
_omnishell_preexec() { _omnishell_pending="$1"; _omnishell_started=$EPOCHREALTIME; }
# precmd reçoit le code de sortie avant qu'une autre commande ne le remplace.
_omnishell_precmd() {
  local previous_status=$?
  # Une première invite ou une commande déjà capturée ne produit aucune écriture.
  [[ -n "${_omnishell_pending:-}" ]] || return 0
  # La durée couvre la commande elle-même, pas le temps passé à lire l’invite.
  local elapsed_ms=$(( int((EPOCHREALTIME - _omnishell_started) * 1000) ))
  # Le processus indépendant ne retarde pas le prochain prompt.
  printf '%s' "$_omnishell_pending" | OMNISHELL_SHELL=zsh OMNISHELL_EXIT="$previous_status" OMNISHELL_DURATION="$elapsed_ms" command omnishell capture-hook >/dev/null 2>&1 &!
  # Le tampon ne doit pas être repris lors d'une simple réaffichage d'invite.
  unset _omnishell_pending
}
# Réimporter le fichier remplace proprement les callbacks homonymes.
add-zsh-hook -d preexec _omnishell_preexec 2>/dev/null
add-zsh-hook -d precmd _omnishell_precmd 2>/dev/null
# Ces deux callbacks ne modifient ni la commande ni son résultat.
add-zsh-hook preexec _omnishell_preexec
add-zsh-hook precmd _omnishell_precmd
