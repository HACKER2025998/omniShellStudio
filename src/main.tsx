// React pilote la fenêtre ; aucune ressource distante n'est chargée au démarrage.
import React, { useState, useEffect, useCallback, useRef } from "react";
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
import { createRoot } from "react-dom/client";
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
import {
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Terminal,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Search,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Command as CommandIcon,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Container,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Network,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  FolderOpen,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Database,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Settings,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Plus,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  ArrowUpRight,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Copy,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Star,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Check,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  RefreshCw,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  ChevronRight,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Shield,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Trash2,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Download,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  X,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Wifi,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Sun,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  PanelRightClose,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Play,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  FileCode,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Link,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  Users,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  StopCircle,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
} from "lucide-react";
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
import { api, download, type Command, type Snapshot } from "./api";
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
import { GraphCanvas } from "./components/GraphCanvas";
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
import { NetworkWorkspace } from "./components/NetworkWorkspace";
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
import { DatabaseConnection } from "./components/DatabaseConnection";
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
import { ShareTools } from "./components/ShareTools";
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
import { CommandDetails } from "./components/CommandDetails";
// @ligne Importe les outils nécessaires à la mise en œuvre du bureau et des parcours utilisateur.
import "./style.css";
// Les catégories sont partagées visuellement par navigation et filtres.
const categories: Record<string, string> = {
  // @ligne Renseigne le champ « containers » de la structure utilisée ici du bureau et des parcours utilisateur.
  containers: "Conteneurs",
  // @ligne Renseigne le champ « network » de la structure utilisée ici du bureau et des parcours utilisateur.
  network: "Réseau & sécurité",
  // @ligne Renseigne le champ « system » de la structure utilisée ici du bureau et des parcours utilisateur.
  system: "Système & paquets",
  // @ligne Renseigne le champ « files » de la structure utilisée ici du bureau et des parcours utilisateur.
  files: "Fichiers & navigation",
  // @ligne Renseigne le champ « git » de la structure utilisée ici du bureau et des parcours utilisateur.
  git: "Git & versions",
  // @ligne Renseigne le champ « database » de la structure utilisée ici du bureau et des parcours utilisateur.
  database: "Bases de données",
  // @ligne Renseigne le champ « ai » de la structure utilisée ici du bureau et des parcours utilisateur.
  ai: "IA & Python",
  // @ligne Renseigne le champ « development » de la structure utilisée ici du bureau et des parcours utilisateur.
  development: "Développement",
  // @ligne Renseigne le champ « storage » de la structure utilisée ici du bureau et des parcours utilisateur.
  storage: "Stockage & sauvegardes",
  // @ligne Renseigne le champ « remote » de la structure utilisée ici du bureau et des parcours utilisateur.
  remote: "Accès distants",
  // @ligne Renseigne le champ « observability » de la structure utilisée ici du bureau et des parcours utilisateur.
  observability: "Mesures & observabilité",
  // @ligne Renseigne le champ « other » de la structure utilisée ici du bureau et des parcours utilisateur.
  other: "Autres commandes",
// @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
};
// Chaque module possède une identité courte et une aide contextualisée.
const modules = [
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  {
    // @ligne Renseigne le champ « id » de la structure utilisée ici du bureau et des parcours utilisateur.
    id: "vault",
    // @ligne Renseigne le champ « name » de la structure utilisée ici du bureau et des parcours utilisateur.
    name: "Command Vault",
    // @ligne Renseigne le champ « icon » de la structure utilisée ici du bureau et des parcours utilisateur.
    icon: Terminal,
    // @ligne Renseigne le champ « tag » de la structure utilisée ici du bureau et des parcours utilisateur.
    tag: "Votre mémoire de travail",
    // @ligne Renseigne le champ « title » de la structure utilisée ici du bureau et des parcours utilisateur.
    title: "Les bonnes commandes. À portée de main.",
    // @ligne Renseigne le champ « desc » de la structure utilisée ici du bureau et des parcours utilisateur.
    desc: "Retrouvez, comprenez et réutilisez ce qui fonctionne.",
  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
  },
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  {
    // @ligne Renseigne le champ « id » de la structure utilisée ici du bureau et des parcours utilisateur.
    id: "docker",
    // @ligne Renseigne le champ « name » de la structure utilisée ici du bureau et des parcours utilisateur.
    name: "Docker & Infra",
    // @ligne Renseigne le champ « icon » de la structure utilisée ici du bureau et des parcours utilisateur.
    icon: Container,
    // @ligne Renseigne le champ « tag » de la structure utilisée ici du bureau et des parcours utilisateur.
    tag: "Infrastructure locale",
    // @ligne Renseigne le champ « title » de la structure utilisée ici du bureau et des parcours utilisateur.
    title: "Votre infrastructure, même hors ligne.",
    // @ligne Renseigne le champ « desc » de la structure utilisée ici du bureau et des parcours utilisateur.
    desc: "Captures persistantes et raccourcis prêts à copier.",
  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
  },
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  {
    // @ligne Renseigne le champ « id » de la structure utilisée ici du bureau et des parcours utilisateur.
    id: "netmap",
    // @ligne Renseigne le champ « name » de la structure utilisée ici du bureau et des parcours utilisateur.
    name: "NetMap Studio",
    // @ligne Renseigne le champ « icon » de la structure utilisée ici du bureau et des parcours utilisateur.
    icon: Network,
    // @ligne Renseigne le champ « tag » de la structure utilisée ici du bureau et des parcours utilisateur.
    tag: "Explorer le réseau",
    // @ligne Renseigne le champ « title » de la structure utilisée ici du bureau et des parcours utilisateur.
    title: "Donnez un visage à votre réseau.",
    // @ligne Renseigne le champ « desc » de la structure utilisée ici du bureau et des parcours utilisateur.
    desc: "Observez les services accessibles, sans inventer les liens physiques.",
  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
  },
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  {
    // @ligne Renseigne le champ « id » de la structure utilisée ici du bureau et des parcours utilisateur.
    id: "share",
    // @ligne Renseigne le champ « name » de la structure utilisée ici du bureau et des parcours utilisateur.
    name: "ClassShare LAN",
    // @ligne Renseigne le champ « icon » de la structure utilisée ici du bureau et des parcours utilisateur.
    icon: FolderOpen,
    // @ligne Renseigne le champ « tag » de la structure utilisée ici du bureau et des parcours utilisateur.
    tag: "Travailler ensemble",
    // @ligne Renseigne le champ « title » de la structure utilisée ici du bureau et des parcours utilisateur.
    title: "Une salle. Un réseau. Vos fichiers.",
    // @ligne Renseigne le champ « desc » de la structure utilisée ici du bureau et des parcours utilisateur.
    desc: "Partagez sur votre LAN et gardez la main sur les permissions.",
  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
  },
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  {
    // @ligne Renseigne le champ « id » de la structure utilisée ici du bureau et des parcours utilisateur.
    id: "dba",
    // @ligne Renseigne le champ « name » de la structure utilisée ici du bureau et des parcours utilisateur.
    name: "DBA & Merise",
    // @ligne Renseigne le champ « icon » de la structure utilisée ici du bureau et des parcours utilisateur.
    icon: Database,
    // @ligne Renseigne le champ « tag » de la structure utilisée ici du bureau et des parcours utilisateur.
    tag: "Comprendre les données",
    // @ligne Renseigne le champ « title » de la structure utilisée ici du bureau et des parcours utilisateur.
    title: "Du SQL à une structure lisible.",
    // @ligne Renseigne le champ « desc » de la structure utilisée ici du bureau et des parcours utilisateur.
    desc: "Analysez votre DDL localement et conservez vos modèles.",
  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
  },
// @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
];
// Un composant unique garantit le même retour de copie partout dans l'atelier.
function CopyButton({
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  text,
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  label = "Copier",
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  onError,
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
}: {
  // @ligne Renseigne le champ « text » de la structure utilisée ici du bureau et des parcours utilisateur.
  text: string;
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  label?: string;
  // @ligne Renseigne le champ « onError » de la structure utilisée ici du bureau et des parcours utilisateur.
  onError: (s: string) => void;
// @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
}) {
  const [copied, setCopied] = useState(false); // État bref de confirmation, propre à chaque bouton.
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
  return (
    // @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React.
    <button
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      className="subtle"
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      onClick={async (e) => {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        e.stopPropagation();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        try {
          // @ligne Prépare « value » pour la prochaine étape du bureau et des parcours utilisateur.
          let value = text;
          // @ligne Parcourt les éléments ou événements indiqués ici pour traiter chaque cas du bureau et des parcours utilisateur.
          for (const match of text.matchAll(/\{\{(\w+)\}\}/g)) {
            // @ligne Prépare « answer » pour la prochaine étape du bureau et des parcours utilisateur.
            const answer = window.prompt("Valeur pour " + match[1]);
            // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
            if (answer === null) return;
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            value = value.replaceAll(match[0], answer);
          // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
          }
          // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
          await navigator.clipboard.writeText(value);
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          setCopied(true);
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          setTimeout(() => setCopied(false), 2200);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        } catch {
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          onError(
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            "Presse-papier inaccessible. Sélectionnez le texte pour le copier.",
          // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
          );
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        }
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      }}
    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
    >
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
      {copied ? <Check size={15} /> : <Copy size={15} />}{" "}
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
      {copied ? "Copié !" : label}
    {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
    </button>
  ); // La substitution ne déclenche jamais une exécution.
// @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
}
// Une erreur de rendu garde un message lisible au lieu d'une fenêtre blanche.
class ErrorBoundary extends React.Component<
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  { children: React.ReactNode },
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  { error: boolean }
// @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
> {
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  state = { error: false };
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  static getDerivedStateFromError() {
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
    return { error: true };
  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
  }
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  render() {
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
    return this.state.error ? (
      // @ligne Construit ou ferme l’élément « main » de l’interface ; son contenu reste géré par React.
      <main className="fatal">
        {/* @ligne Construit ou ferme l’élément « h1L’atelier » de l’interface ; son contenu reste géré par React. */}
        <h1>L’atelier a rencontré une erreur.</h1>
        {/* @ligne Construit ou ferme l’élément « pVos » de l’interface ; son contenu reste géré par React. */}
        <p>Vos données sont conservées sur disque.</p>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button onClick={() => location.reload()}>Rouvrir l’atelier</button>
      {/* @ligne Construit ou ferme l’élément « main » de l’interface ; son contenu reste géré par React. */}
      </main>
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    ) : (
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      this.props.children
    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
    );
  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
  }
// @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
}
// L'application utilise des requêtes courtes et des états de chargement explicites.
function App() {
  // @ligne Prépare les valeurs locales pour la prochaine étape du bureau et des parcours utilisateur.
  const [module, setModule] = useState("vault"),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [query, setQuery] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [category, setCategory] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [favorites, setFavorites] = useState(false),
    [sudoOnly, setSudoOnly] = useState(false), // Le filtre de privilège est indépendant des catégories.
    [since, setSince] = useState(""), // Une date vide ne limite pas la recherche.
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [commands, setCommands] = useState<Command[]>([]),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [total, setTotal] = useState(0),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [offset, setOffset] = useState(0),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [stats, setStats] = useState<any>({ total: 0, categories: [] }),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [selected, setSelected] = useState<Command | null>(null),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [error, setError] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [notice, setNotice] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [busy, setBusy] = useState(false),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [settings, setSettings] = useState(false),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [interfaces, setInterfaces] = useState<any[]>([]),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [ip, setIp] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [docker, setDocker] = useState<Snapshot[]>([]),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [graph, setGraph] = useState<any>(null),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [cidr, setCidr] = useState("192.168.1.0/24"),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [room, setRoom] = useState<any>(null),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [peers, setPeers] = useState<any[]>([]),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [paths, setPaths] = useState(""),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [maxFileMib, setMaxFileMib] = useState(2048),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [maxRoomMib, setMaxRoomMib] = useState(10240),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [reserveMib, setReserveMib] = useState(64),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [ddl, setDdl] = useState(
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      "CREATE TABLE utilisateurs (\n  id INTEGER PRIMARY KEY,\n  nom VARCHAR(120) NOT NULL\n);\n\nCREATE TABLE projets (\n  id INTEGER PRIMARY KEY,\n  titre VARCHAR(200),\n  auteur_id INTEGER REFERENCES utilisateurs(id)\n);",
    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
    ),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [model, setModel] = useState<any>(null),
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    [modelName, setModelName] = useState("Mon modèle"),
    [models, setModels] = useState<any[]>([]); // Chaque domaine conserve son état lors d'un changement d'onglet.
  // @ligne Prépare « searchRef » pour la prochaine étape du bureau et des parcours utilisateur.
  const searchRef = useRef<HTMLInputElement>(null),
    requestId = useRef(0); // Permet le focus clavier et écarte les recherches obsolètes.
  const current = modules.find((m) => m.id === module)!; // Les identifiants proviennent uniquement de la navigation.
  // @ligne Prépare « notify » pour la prochaine étape du bureau et des parcours utilisateur.
  const notify = (message: string) => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    setNotice(message);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    setTimeout(() => setNotice(""), 5000);
  }; // Retour d'action distinct des erreurs persistantes.
  // @ligne Prépare « run » pour la prochaine étape du bureau et des parcours utilisateur.
  const run = async <T,>(job: () => Promise<T>): Promise<T | undefined> => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    setBusy(true);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    setError("");
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    try {
      // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
      return await job();
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    } catch (e) {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      setError(String(e));
      // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
      return undefined;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    } finally {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      setBusy(false);
    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
    }
  }; // Les échecs de service n'effacent pas les caches affichés.
  // @ligne Prépare « loadCommands » pour la prochaine étape du bureau et des parcours utilisateur.
  const loadCommands = useCallback(async () => {
    // @ligne Prépare « id » pour la prochaine étape du bureau et des parcours utilisateur.
    const id = ++requestId.current;
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    try {
      // @ligne Prépare « data » pour la prochaine étape du bureau et des parcours utilisateur.
      const data = await api<{ items: Command[]; total: number }>("commands", {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        query,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        category,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        favorites,
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        offset,
        // @ligne Renseigne le champ « privileged » de la structure utilisée ici du bureau et des parcours utilisateur.
        privileged: sudoOnly ? true : null,
        // @ligne Renseigne le champ « since » de la structure utilisée ici du bureau et des parcours utilisateur.
        since: since ? Math.floor(new Date(since).getTime() / 1000) : null,
      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
      });
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
      if (id === requestId.current) {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        setCommands(data.items);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        setTotal(data.total);
      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
      }
      // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
      setStats(await api("stats"));
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    } catch (e) {
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      setError(String(e));
    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
    }
  }, [query, category, favorites, offset, sudoOnly, since]); // La dernière requête seule remplace la liste.
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  useEffect(() => {
    // @ligne Prépare « timer » pour la prochaine étape du bureau et des parcours utilisateur.
    const timer = setTimeout(loadCommands, 150);
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
    return () => clearTimeout(timer);
  }, [loadCommands]); // Réduit les accès disque durant la frappe.
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  useEffect(() => {
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    setOffset(0);
  }, [query, category, favorites, sudoOnly, since]); // Un nouveau filtre revient à la première page.
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  useEffect(() => {
    // @ligne Prépare « key » pour la prochaine étape du bureau et des parcours utilisateur.
    const key = (e: KeyboardEvent) => {
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
      if ((e.ctrlKey || e.metaKey) && e.key === "k") {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        e.preventDefault();
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        setModule("vault");
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        searchRef.current?.focus();
      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
      }
      // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
      if (e.key === "Escape") {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        setSelected(null);
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        setSettings(false);
      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
      }
    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
    };
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    window.addEventListener("keydown", key);
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    api<any[]>("interfaces")
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      .then((data) => {
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        setInterfaces(data);
        // @ligne Prépare « first » pour la prochaine étape du bureau et des parcours utilisateur.
        const first = data.find((i) => /^\d+\./.test(i.ip));
        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
        if (first) {
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          setIp(first.ip);
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          setCidr(first.ip.split(".").slice(0, 3).join(".") + ".0/24");
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        }
      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
      })
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      .catch(() => {});
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
    return () => window.removeEventListener("keydown", key);
  }, []); // Aucun scan réseau au démarrage.
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  useEffect(() => {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
    if (module === "docker")
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      api<Snapshot[]>("docker_cache")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        .then(setDocker)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        .catch((e) => setError(String(e)));
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
    if (module === "netmap")
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      api<Snapshot[]>("net_cache")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        .then((data) => {
          // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
          if (data[0]) setGraph(data[0].data);
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        })
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        .catch(() => {});
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
    if (module === "dba")
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      api<any[]>("sql_models")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        .then(setModels)
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        .catch(() => {});
  }, [module]); // Les vues utilisent d'abord les données hors ligne.
  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
  useEffect(() => {
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
    if (module !== "share") return;
    // @ligne Prépare « refresh » pour la prochaine étape du bureau et des parcours utilisateur.
    const refresh = () =>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      api("share_status")
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        .then((data) => {
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          setRoom(data.room);
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          setPeers(data.peers);
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        })
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        .catch(() => {});
    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
    refresh();
    // @ligne Prépare « timer » pour la prochaine étape du bureau et des parcours utilisateur.
    const timer = setInterval(refresh, 2000);
    // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
    return () => clearInterval(timer);
  }, [module]); // Le polling administrateur ne sort pas de l'IPC local.
  // @ligne Prépare « favorite » pour la prochaine étape du bureau et des parcours utilisateur.
  const favorite = async (c: Command) => {
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
    await run(() => api("favorite", { id: c.id }));
    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
    await loadCommands();
    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
    if (selected?.id === c.id) setSelected({ ...c, favorite: !c.favorite });
  }; // Met à jour aussi l'inspecteur ouvert.
  // @ligne Prépare « exportJson » pour la prochaine étape du bureau et des parcours utilisateur.
  const exportJson = (name: string, value: unknown) =>
    download(name, JSON.stringify(value, null, 2)); // Exports lisibles et indépendants du service.
  // @ligne Rend le résultat ou l’erreur à l’appelant sans masquer l’état réel du bureau et des parcours utilisateur.
  return (
    // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
    <div className="app">
      {/* L'en-tête reste fixe et accessible depuis chaque module. */}
      <header>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        <div className="brand">
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          <span className="logo">
            {/* @ligne Construit ou ferme l’élément « CommandIcon » de l’interface ; son contenu reste géré par React. */}
            <CommandIcon size={22} />
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          </span>
          {/* @ligne Construit ou ferme l’élément « strong » de l’interface ; son contenu reste géré par React. */}
          <strong>
            OmniShell<span> Studio</span>
          {/* @ligne Construit ou ferme l’élément « strong » de l’interface ; son contenu reste géré par React. */}
          </strong>
          {/* @ligne Construit ou ferme l’élément « smallLOCALsmall » de l’interface ; son contenu reste géré par React. */}
          <small>LOCAL</small>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        </div>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        <div className="search">
          {/* @ligne Construit ou ferme l’élément « Search » de l’interface ; son contenu reste géré par React. */}
          <Search size={17} />
          {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
          <input
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            ref={searchRef}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            aria-label="Rechercher une commande"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            placeholder="Retrouver une commande, un outil…"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            value={query}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            onChange={(e) => {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              setQuery(e.target.value);
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              setModule("vault");
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            }}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          />
          {/* @ligne Construit ou ferme l’élément « kbd⌃ » de l’interface ; son contenu reste géré par React. */}
          <kbd>⌃ K</kbd>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        </div>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button className="primary" onClick={() => setModule("share")}>
          {/* @ligne Construit ou ferme l’élément « Plus » de l’interface ; son contenu reste géré par React. */}
          <Plus size={16} /> Quick Share
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          className="icon-button"
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          aria-label="Réglages"
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          onClick={() => setSettings(true)}
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        >
          {/* @ligne Construit ou ferme l’élément « Settings » de l’interface ; son contenu reste géré par React. */}
          <Settings size={19} />
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
      {/* @ligne Construit ou ferme l’élément « header » de l’interface ; son contenu reste géré par React. */}
      </header>
      {/* La barre latérale présente les modules, puis la taxonomie du coffre. */}
      <aside className="sidebar">
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        <div className="workspace">
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          <span className="avatar">OS</span>
          {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
          <div>
            {/* @ligne Construit ou ferme l’élément « strongMon » de l’interface ; son contenu reste géré par React. */}
            <strong>Mon espace local</strong>
            {/* @ligne Construit ou ferme l’élément « smallPersonnel » de l’interface ; son contenu reste géré par React. */}
            <small>Personnel · hors ligne</small>
          {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
          </div>
          {/* @ligne Construit ou ferme l’élément « ChevronRight » de l’interface ; son contenu reste géré par React. */}
          <ChevronRight size={14} />
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        </div>
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        <p className="eyebrow">ATELIER</p>
        {/* @ligne Construit ou ferme l’élément « nav » de l’interface ; son contenu reste géré par React. */}
        <nav>
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
          {modules.map((m) => (
            // @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React.
            <button
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              key={m.id}
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              className={module === m.id ? "nav active" : "nav"}
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              onClick={() => {
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                setModule(m.id);
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                setSelected(null);
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                setError("");
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              }}
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            >
              {/* @ligne Construit ou ferme l’élément « m.icon » de l’interface ; son contenu reste géré par React. */}
              <m.icon size={18} />
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
              {m.name}
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
              {module === m.id && <span className="nav-dot" />}
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            </button>
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          ))}
        {/* @ligne Construit ou ferme l’élément « nav » de l’interface ; son contenu reste géré par React. */}
        </nav>
        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
        <p className="eyebrow category-heading">EXPLORER LES COMMANDES</p>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        <button
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          className={"category " + (!category ? "chosen" : "")}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          onClick={() => {
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            setCategory("");
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            setModule("vault");
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          }}
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        >
          {/* @ligne Construit ou ferme l’élément « spanToutes » de l’interface ; son contenu reste géré par React. */}
          <span>Toutes les catégories</span>
          {/* @ligne Construit ou ferme l’élément « small{stats.total}small » de l’interface ; son contenu reste géré par React. */}
          <small>{stats.total}</small>
        {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
        </button>
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
        {Object.entries(categories).map(([key, name]) => (
          // @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React.
          <button
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            key={key}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            className={"category " + (category === key ? "chosen" : "")}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            onClick={() => {
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              setCategory(key);
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              setModule("vault");
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            }}
          // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
          >
            {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
            <span>
              {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
              <span className={"dot " + key} />
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
              {name}
            {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
            </span>
            {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
            <small>
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
              {stats.categories.find((c: any) => c[0] === key)?.[1] || 0}
            {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
            </small>
          {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
          </button>
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        ))}
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        <div className="local-card">
          {/* @ligne Construit ou ferme l’élément « Shield » de l’interface ; son contenu reste géré par React. */}
          <Shield size={19} />
          {/* @ligne Construit ou ferme l’élément « strongVotre » de l’interface ; son contenu reste géré par React. */}
          <strong>Votre machine. Vos données.</strong>
          {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
          <p>
            Aucun cloud, aucune télémétrie.
            {/* @ligne Construit ou ferme l’élément « br » de l’interface ; son contenu reste géré par React. */}
            <br />
            Un atelier qui reste avec vous.
          {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
          </p>
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          <span>
            {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
            <span className="status-dot" /> Local-first, par conception
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          </span>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        </div>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        <div className="sidebar-footer">
          {/* @ligne Construit ou ferme l’élément « Sun » de l’interface ; son contenu reste géré par React. */}
          <Sun size={16} /> Daylight workspace <small>v1.0 RC1</small>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        </div>
      {/* @ligne Construit ou ferme l’élément « aside » de l’interface ; son contenu reste géré par React. */}
      </aside>
      {/* Le canevas central explique l'état réel de chaque service. */}
      <main
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        className={
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          selected && module === "vault" ? "content with-inspector" : "content"
        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
        }
      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
      >
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        <div className="breadcrumb">
          Mon espace <ChevronRight size={12} /> {current.name}
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          <span>
            {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
            <span className="status-dot" /> Stockage local
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          </span>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        </div>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        <div className="page-heading">
          {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
          <div>
            {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
            <p className="overline">{current.tag}</p>
            {/* @ligne Construit ou ferme l’élément « h1{current.title}h1 » de l’interface ; son contenu reste géré par React. */}
            <h1>{current.title}</h1>
            {/* @ligne Construit ou ferme l’élément « p{current.desc}p » de l’interface ; son contenu reste géré par React. */}
            <p>{current.desc}</p>
          {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
          </div>
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          <span className="heading-icon">
            {/* @ligne Construit ou ferme l’élément « current.icon » de l’interface ; son contenu reste géré par React. */}
            <current.icon size={31} />
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          </span>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        </div>
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
        {error && (
          // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
          <div className="alert" role="alert">
            {/* @ligne Construit ou ferme l’élément « Shield » de l’interface ; son contenu reste géré par React. */}
            <Shield size={17} />
            {/* @ligne Construit ou ferme l’élément « span{error}span » de l’interface ; son contenu reste géré par React. */}
            <span>{error}</span>
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            <button aria-label="Fermer le message" onClick={() => setError("")}>
              {/* @ligne Construit ou ferme l’élément « X » de l’interface ; son contenu reste géré par React. */}
              <X size={15} />
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            </button>
          {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
          </div>
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        )}
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
        {module === "vault" && (
          // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
          <>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="metrics">
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              <div>
                {/* @ligne Construit ou ferme l’élément « spanCommandes » de l’interface ; son contenu reste géré par React. */}
                <span>Commandes mémorisées</span>
                {/* @ligne Construit ou ferme l’élément « strong » de l’interface ; son contenu reste géré par React. */}
                <strong>
                  {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                  {stats.total.toLocaleString("fr-FR")}
                  {/* @ligne Construit ou ferme l’élément « smalldans » de l’interface ; son contenu reste géré par React. */}
                  <small>dans votre coffre</small>
                {/* @ligne Construit ou ferme l’élément « strong » de l’interface ; son contenu reste géré par React. */}
                </strong>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              </div>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              <div>
                {/* @ligne Construit ou ferme l’élément « spanOrganisation » de l’interface ; son contenu reste géré par React. */}
                <span>Organisation automatique</span>
                {/* @ligne Construit ou ferme l’élément « strong » de l’interface ; son contenu reste géré par React. */}
                <strong>
                  {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                  {stats.categories.length}
                  {/* @ligne Construit ou ferme l’élément « smallcatégories » de l’interface ; son contenu reste géré par React. */}
                  <small>catégories utilisées</small>
                {/* @ligne Construit ou ferme l’élément « strong » de l’interface ; son contenu reste géré par React. */}
                </strong>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              </div>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              <div>
                {/* @ligne Construit ou ferme l’élément « spanConçu » de l’interface ; son contenu reste géré par React. */}
                <span>Conçu pour durer</span>
                {/* @ligne Construit ou ferme l’élément « strong » de l’interface ; son contenu reste géré par React. */}
                <strong>
                  100 %<small>sur votre machine</small>
                {/* @ligne Construit ou ferme l’élément « strong » de l’interface ; son contenu reste géré par React. */}
                </strong>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              </div>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="toolbar">
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              <div className="tabs">
                {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                <button
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  className={!favorites ? "tab selected" : "tab"}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onClick={() => setFavorites(false)}
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                >
                  Toutes les commandes <small>{total}</small>
                {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                </button>
                {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                <button
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  className={favorites ? "tab selected" : "tab"}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onClick={() => setFavorites(true)}
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                >
                  {/* @ligne Construit ou ferme l’élément « Star » de l’interface ; son contenu reste géré par React. */}
                  <Star size={14} /> Favoris
                {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                </button>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              </div>
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              <button
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                disabled={busy}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                onClick={() =>
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  run(async () => {
                    // @ligne Prépare « r » pour la prochaine étape du bureau et des parcours utilisateur.
                    const r = await api("import");
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    notify(JSON.stringify(r.sources));
                    // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                    await loadCommands();
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  })
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                }
              // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
              >
                {/* @ligne Construit ou ferme l’élément « RefreshCw » de l’interface ; son contenu reste géré par React. */}
                <RefreshCw size={15} className={busy ? "spin" : ""} /> Importer
                les historiques
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              </button>
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              <button
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                aria-label="Exporter la page de commandes"
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                onClick={() => exportJson("omnishell-commandes.json", commands)}
              // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
              >
                {/* @ligne Construit ou ferme l’élément « Download » de l’interface ; son contenu reste géré par React. */}
                <Download size={16} />
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              </button>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="vault-filters">
              {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
              <label className="inline-label">
                {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
                <input
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  type="checkbox"
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  checked={sudoOnly}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onChange={(e) => setSudoOnly(e.target.checked)}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                />{" "}
                Avec élévation
              {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
              </label>
              {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
              <label className="inline-label">
                Depuis le{" "}
                {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
                <input
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  type="date"
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  value={since}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onChange={(e) => setSince(e.target.value)}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                />
              {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
              </label>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="list-label">
              {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
              <span>
                {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                {category ? categories[category] : "VOTRE MÉMOIRE DE TERMINAL"}
              {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
              </span>
              {/* @ligne Construit ou ferme l’élément « spanLes » de l’interface ; son contenu reste géré par React. */}
              <span>Les plus récentes d’abord</span>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="command-list">
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
              {commands.map((c) => (
                // @ligne Construit ou ferme l’élément « article » de l’interface ; son contenu reste géré par React.
                <article
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  tabIndex={0}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onKeyDown={(e) => {
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
                    if (e.key === "Enter") setSelected(c);
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  }}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onClick={() => setSelected(c)}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  key={c.id}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  className={
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    "command-card " + (selected?.id === c.id ? "focused" : "")
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  }
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                >
                  {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
                  <span className={"binary-icon " + c.category}>
                    {/* @ligne Construit ou ferme l’élément « Terminal » de l’interface ; son contenu reste géré par React. */}
                    <Terminal size={19} />
                  {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
                  </span>
                  {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                  <div className="command-body">
                    {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                    <div className="command-title">
                      {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
                      <span className="badge">{c.binary || "shell"}</span>
                      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                      {c.privileged && <span className="badge sudo">SUDO</span>}
                      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                      {c.raw.includes("[REDACTED]") && (
                        // @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React.
                        <span className="badge safe">Secrets masqués</span>
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      )}
                      {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
                      <small>
                        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                        {new Date(c.last_seen * 1000).toLocaleDateString(
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          "fr-FR",
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        )}
                      {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
                      </small>
                    {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                    </div>
                    {/* @ligne Construit ou ferme l’élément « code{c.raw}code » de l’interface ; son contenu reste géré par React. */}
                    <code>{c.raw}</code>
                    {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                    <div className="metadata">
                      {/* @ligne Construit ou ferme l’élément « span{categories[c.category]}span » de l’interface ; son contenu reste géré par React. */}
                      <span>{categories[c.category]}</span>
                      {/* @ligne Construit ou ferme l’élément « i » de l’interface ; son contenu reste géré par React. */}
                      <i /> {c.count} capture{c.count > 1 ? "s" : ""} <i />{" "}
                      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                      {c.shell || "Shell inconnu"}
                      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                      {c.cwd && (
                        // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
                        <>
                          {/* @ligne Construit ou ferme l’élément « i » de l’interface ; son contenu reste géré par React. */}
                          <i />
                          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
                          <span className="path">{c.cwd}</span>
                        {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
                        </>
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      )}
                    {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                    </div>
                  {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                  </div>
                  {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                  <div className="card-actions">
                    {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                    <button
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      aria-label="Basculer favori"
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      className={c.favorite ? "star favorite" : "star"}
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      onClick={(e) => {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        e.stopPropagation();
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        favorite(c);
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      }}
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    >
                      {/* @ligne Construit ou ferme l’élément « Star » de l’interface ; son contenu reste géré par React. */}
                      <Star
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        size={17}
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        fill={c.favorite ? "currentColor" : "none"}
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      />
                    {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                    </button>
                    {/* @ligne Construit ou ferme l’élément « CopyButton » de l’interface ; son contenu reste géré par React. */}
                    <CopyButton text={c.raw} onError={setError} />
                    {/* @ligne Construit ou ferme l’élément « ChevronRight » de l’interface ; son contenu reste géré par React. */}
                    <ChevronRight size={15} />
                  {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                  </div>
                {/* @ligne Construit ou ferme l’élément « article » de l’interface ; son contenu reste géré par React. */}
                </article>
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              ))}
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
              {commands.length === 0 && (
                // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
                <div className="empty">
                  {/* @ligne Construit ou ferme l’élément « Terminal » de l’interface ; son contenu reste géré par React. */}
                  <Terminal size={38} />
                  {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
                  <h2>
                    {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                    {query
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      ? "Aucune commande correspondante"
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      : "Votre mémoire de terminal commence ici."}
                  {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
                  </h2>
                  {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                  <p>
                    Importez vos historiques existants ou ajoutez le hook
                    documenté pour capturer vos prochaines commandes.
                  {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                  </p>
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  <button
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    className="primary"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onClick={() =>
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      run(async () => {
                        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                        await api("import");
                        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                        await loadCommands();
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        notify("Import terminé.");
                      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                      })
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    }
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  >
                    {/* @ligne Construit ou ferme l’élément « Plus » de l’interface ; son contenu reste géré par React. */}
                    <Plus size={16} /> Importer mon historique
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  </button>
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                </div>
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              )}
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="pagination">
              {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
              <span>
                {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                {total ? offset + 1 : 0}–{Math.min(offset + 100, total)} sur{" "}
                {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                {total}
              {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
              </span>
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              <button
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                disabled={offset === 0}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                onClick={() => setOffset(Math.max(0, offset - 100))}
              // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
              >
                Précédent
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              </button>
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              <button
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                disabled={offset + 100 >= total}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                onClick={() => setOffset(offset + 100)}
              // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
              >
                Suivant
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              </button>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="tip">
              {/* @ligne Construit ou ferme l’élément « span✦span » de l’interface ; son contenu reste géré par React. */}
              <span>✦</span>
              {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
              <p>
                {/* @ligne Construit ou ferme l’élément « strongLe » de l’interface ; son contenu reste géré par React. */}
                <strong>Le petit réflexe qui change tout.</strong> Ajoutez vos
                commandes utiles aux favoris. Votre prochain dépannage vous dira
                merci.
              {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
              </p>
              {/* @ligne Construit ou ferme l’élément « kbdCtrl » de l’interface ; son contenu reste géré par React. */}
              <kbd>Ctrl K</kbd>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
          {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
          </>
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        )}
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
        {module === "docker" && (
          // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
          <>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="toolbar">
              {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
              <span className="badge sudo">
                Cache persistant · actualisation manuelle
              {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
              </span>
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              <button
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                disabled={busy}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                className="primary"
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                onClick={() =>
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  run(async () => {
                    // @ligne Prépare « r » pour la prochaine étape du bureau et des parcours utilisateur.
                    const r = await api("docker_sync");
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    setDocker(r.snapshots);
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    notify(`${r.updated} conteneur(s) actualisé(s).`);
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  })
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                }
              // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
              >
                {/* @ligne Construit ou ferme l’élément « RefreshCw » de l’interface ; son contenu reste géré par React. */}
                <RefreshCw size={16} /> Synchroniser Docker
              {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
              </button>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="grid">
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
              {docker.map((s) => (
                // @ligne Construit ou ferme l’élément « article » de l’interface ; son contenu reste géré par React.
                <article className="panel" key={s.data.id}>
                  {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
                  <span className="badge">{s.data.state}</span>
                  {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
                  <h2>
                    {/* @ligne Construit ou ferme l’élément « Container » de l’interface ; son contenu reste géré par React. */}
                    <Container size={22} /> {s.data.name}
                  {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
                  </h2>
                  {/* @ligne Construit ou ferme l’élément « code{s.data.image}code » de l’interface ; son contenu reste géré par React. */}
                  <code>{s.data.image}</code>
                  {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                  <p className="muted">
                    Capture du{" "}
                    {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                    {new Date(s.captured_at * 1000).toLocaleString("fr-FR")}
                  {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                  </p>
                  {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
                  <details>
                    {/* @ligne Construit ou ferme l’élément « summaryPorts, » de l’interface ; son contenu reste géré par React. */}
                    <summary>Ports, volumes & environnement masqué</summary>
                    {/* @ligne Construit ou ferme l’élément « pre » de l’interface ; son contenu reste géré par React. */}
                    <pre>
                      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                      {JSON.stringify(
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        {
                          // @ligne Renseigne le champ « ports » de la structure utilisée ici du bureau et des parcours utilisateur.
                          ports: s.data.ports,
                          // @ligne Renseigne le champ « mounts » de la structure utilisée ici du bureau et des parcours utilisateur.
                          mounts: s.data.mounts,
                          // @ligne Renseigne le champ « environment » de la structure utilisée ici du bureau et des parcours utilisateur.
                          environment: s.data.environment,
                        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                        },
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        null,
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        2,
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      )}
                    {/* @ligne Construit ou ferme l’élément « pre » de l’interface ; son contenu reste géré par React. */}
                    </pre>
                  {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
                  </details>
                  {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                  <div className="action-grid">
                    {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                    {Object.entries(s.data.actions || {}).map(
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      ([name, value]) => (
                        // @ligne Construit ou ferme l’élément « CopyButton » de l’interface ; son contenu reste géré par React.
                        <CopyButton
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          key={name}
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          text={String(value)}
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          label={name}
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          onError={setError}
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        />
                      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                      ),
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    )}
                  {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                  </div>
                  {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
                  <details>
                    {/* @ligne Construit ou ferme l’élément « summaryReconstruction » de l’interface ; son contenu reste géré par React. */}
                    <summary>Reconstruction et origine capturée</summary>
                    {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                    {s.data.captured_origin && (
                      // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
                      <>
                        {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                        <p className="help">{s.data.origin}</p>
                        {/* @ligne Construit ou ferme l’élément « pre{s.data.captured_origin}pre » de l’interface ; son contenu reste géré par React. */}
                        <pre>{s.data.captured_origin}</pre>
                        {/* @ligne Construit ou ferme l’élément « CopyButton » de l’interface ; son contenu reste géré par React. */}
                        <CopyButton
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          text={s.data.captured_origin}
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          onError={setError}
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        />
                      {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
                      </>
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    )}
                    {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                    {s.data.compose?.map((file: any) => (
                      // @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React.
                      <details key={file.path}>
                        {/* @ligne Construit ou ferme l’élément « summary{file.path}summary » de l’interface ; son contenu reste géré par React. */}
                        <summary>{file.path}</summary>
                        {/* @ligne Construit ou ferme l’élément « pre{file.content}pre » de l’interface ; son contenu reste géré par React. */}
                        <pre>{file.content}</pre>
                      {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
                      </details>
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    ))}
                    {/* @ligne Construit ou ferme l’élément « pre{s.data.reconstructed}pre » de l’interface ; son contenu reste géré par React. */}
                    <pre>{s.data.reconstructed}</pre>
                    {/* @ligne Construit ou ferme l’élément « CopyButton » de l’interface ; son contenu reste géré par React. */}
                    <CopyButton
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      text={s.data.reconstructed}
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      onError={setError}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    />
                  {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
                  </details>
                  {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
                  <details>
                    {/* @ligne Construit ou ferme l’élément « summaryDerniers » de l’interface ; son contenu reste géré par React. */}
                    <summary>Derniers logs masqués</summary>
                    {/* @ligne Construit ou ferme l’élément « pre{s.data.logs}pre » de l’interface ; son contenu reste géré par React. */}
                    <pre>{s.data.logs}</pre>
                  {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
                  </details>
                {/* @ligne Construit ou ferme l’élément « article » de l’interface ; son contenu reste géré par React. */}
                </article>
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              ))}
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
            {docker.length === 0 && (
              // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
              <div className="empty">
                {/* @ligne Construit ou ferme l’élément « Container » de l’interface ; son contenu reste géré par React. */}
                <Container size={40} />
                {/* @ligne Construit ou ferme l’élément « h2Vos » de l’interface ; son contenu reste géré par React. */}
                <h2>Vos conteneurs ont aussi une mémoire.</h2>
                {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                <p>
                  Démarrez Docker, puis synchronisez. Les captures resteront
                  disponibles après son arrêt.
                {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                </p>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              </div>
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            )}
          {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
          </>
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        )}
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
        {module === "netmap" && (
          // @ligne Construit ou ferme l’élément « NetworkWorkspace » de l’interface ; son contenu reste géré par React.
          <NetworkWorkspace
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            initial={graph}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            onUpdate={setGraph}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            interfaces={interfaces}
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            onError={setError}
          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
          />
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        )}
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
        {module === "share" && (
          // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
          <>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="panel">
              {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
              <h2>
                {/* @ligne Construit ou ferme l’élément « FolderOpen » de l’interface ; son contenu reste géré par React. */}
                <FolderOpen size={22} /> Préparer une salle
              {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
              </h2>
              {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
              <p>
                Choisissez des fichiers ou dossiers. Les dossiers sont préparés
                en archives tar.zst, sans suivre les liens externes.
              {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
              </p>
              {/* @ligne Construit ou ferme l’élément « ShareTools » de l’interface ; son contenu reste géré par React. */}
              <ShareTools
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                onPaths={(selected) =>
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  setPaths((previous) =>
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    [previous, ...selected].filter(Boolean).join("\n"),
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  )
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                }
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                onRoom={setRoom}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                onError={setError}
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              />
              {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
              <label>
                Fichiers et dossiers à partager
                {/* @ligne Construit ou ferme l’élément « textarea » de l’interface ; son contenu reste géré par React. */}
                <textarea
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  value={paths}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onChange={(e) => setPaths(e.target.value)}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  placeholder={
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    "/home/moi/Documents/TP-reseau.zip\n/home/moi/Documents/cours.pdf"
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  }
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                />
              {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
              </label>
              {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
              <details>
                {/* @ligne Construit ou ferme l’élément « summaryQuotas » de l’interface ; son contenu reste géré par React. */}
                <summary>Quotas de la salle</summary>
                {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                <label>
                  Maximum par fichier (Mio)
                  {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
                  <input
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    type="number"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    min="1"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    value={maxFileMib}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onChange={(e) => setMaxFileMib(Number(e.target.value))}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  />
                {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                </label>
                {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                <label>
                  Maximum total reçu (Mio)
                  {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
                  <input
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    type="number"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    min="1"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    value={maxRoomMib}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onChange={(e) => setMaxRoomMib(Number(e.target.value))}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  />
                {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                </label>
                {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                <label>
                  Réserve disque (Mio)
                  {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
                  <input
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    type="number"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    min="16"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    value={reserveMib}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onChange={(e) => setReserveMib(Number(e.target.value))}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  />
                {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                </label>
              {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
              </details>
              {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
              <label>
                Interface LAN
                {/* @ligne Construit ou ferme l’élément « select » de l’interface ; son contenu reste géré par React. */}
                <select value={ip} onChange={(e) => setIp(e.target.value)}>
                  {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                  {interfaces
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    .filter((i) => /^\d+\./.test(i.ip))
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    .map((i) => (
                      // @ligne Construit ou ferme l’élément « option » de l’interface ; son contenu reste géré par React.
                      <option value={i.ip} key={i.name + i.ip}>
                        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                        {i.name} · {i.ip}
                      {/* @ligne Construit ou ferme l’élément « option » de l’interface ; son contenu reste géré par React. */}
                      </option>
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    ))}
                {/* @ligne Construit ou ferme l’élément « select » de l’interface ; son contenu reste géré par React. */}
                </select>
              {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
              </label>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              <div className="actions">
                {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                <button
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  className="primary"
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  disabled={busy || !paths.trim() || !ip}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onClick={() =>
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    run(async () => {
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      setRoom(
                        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                        await api("share_start", {
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          ip,
                          // @ligne Renseigne le champ « limits » de la structure utilisée ici du bureau et des parcours utilisateur.
                          limits: {
                            // @ligne Renseigne le champ « max_file_bytes » de la structure utilisée ici du bureau et des parcours utilisateur.
                            max_file_bytes: maxFileMib * 1024 * 1024,
                            // @ligne Renseigne le champ « max_room_bytes » de la structure utilisée ici du bureau et des parcours utilisateur.
                            max_room_bytes: maxRoomMib * 1024 * 1024,
                            // @ligne Renseigne le champ « reserve_bytes » de la structure utilisée ici du bureau et des parcours utilisateur.
                            reserve_bytes: reserveMib * 1024 * 1024,
                          // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                          },
                          // @ligne Renseigne le champ « paths » de la structure utilisée ici du bureau et des parcours utilisateur.
                          paths: paths
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                            .split("\n")
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                            .map((p) => p.trim())
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                            .filter(Boolean),
                        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                        }),
                      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                      );
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      notify("Salle ouverte sur votre interface LAN.");
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    })
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  }
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                >
                  {/* @ligne Construit ou ferme l’élément « Plus » de l’interface ; son contenu reste géré par React. */}
                  <Plus size={16} /> Ouvrir la salle
                {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                </button>
                {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                {room && (
                  // @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React.
                  <button
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    className="danger"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onClick={() =>
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      run(async () => {
                        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                        await api("share_stop");
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        setRoom(null);
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        setPeers([]);
                      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                      })
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    }
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  >
                    {/* @ligne Construit ou ferme l’élément « StopCircle » de l’interface ; son contenu reste géré par React. */}
                    <StopCircle size={16} /> Fermer la salle
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  </button>
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                )}
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              </div>
              {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
              <p className="help">
                Transport TLS avec certificat local auto-signé. Communiquez le
                certificat aux participants pour valider la première connexion.
                Aucune règle de pare-feu n’est modifiée automatiquement.
              {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
              </p>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
            {room && (
              // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
              <>
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                <div className="room-card">
                  {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                  <div>
                    {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
                    <span className="overline">VOTRE CODE DE SALLE</span>
                    {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                    <div className="pin">
                      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                      {room.pin.slice(0, 3)} {room.pin.slice(3)}
                    {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                    </div>
                    {/* @ligne Construit ou ferme l’élément « code{room.url}code » de l’interface ; son contenu reste géré par React. */}
                    <code>{room.url}</code>
                    {/* @ligne Construit ou ferme l’élément « CopyButton » de l’interface ; son contenu reste géré par React. */}
                    <CopyButton
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      text={room.url}
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      label="Copier le lien"
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      onError={setError}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    />
                    {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                    <p>
                      Le code accompagne le lien ; il ne remplace pas une
                      adresse réseau.
                    {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                    </p>
                    {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
                    <details>
                      {/* @ligne Construit ou ferme l’élément « summaryCertificat » de l’interface ; son contenu reste géré par React. */}
                      <summary>Certificat et dossier de réception</summary>
                      {/* @ligne Construit ou ferme l’élément « pre{room.certificate}pre » de l’interface ; son contenu reste géré par React. */}
                      <pre>{room.certificate}</pre>
                      {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
                      <small>
                        Empreinte BLAKE3 du PEM : {room.fingerprint}
                      {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
                      </small>
                      {/* @ligne Construit ou ferme l’élément « pre{room.upload_dir}pre » de l’interface ; son contenu reste géré par React. */}
                      <pre>{room.upload_dir}</pre>
                    {/* @ligne Construit ou ferme l’élément « details » de l’interface ; son contenu reste géré par React. */}
                    </details>
                  {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                  </div>
                  {/* @ligne Construit ou ferme l’élément « img » de l’interface ; son contenu reste géré par React. */}
                  <img
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    alt="QR code du lien vers la salle"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    width="180"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    src={
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      "data:image/svg+xml;charset=utf-8," +
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      encodeURIComponent(room.qr)
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    }
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  />
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                </div>
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                <div className="panel">
                  {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
                  <h2>
                    {/* @ligne Construit ou ferme l’élément « Users » de l’interface ; son contenu reste géré par React. */}
                    <Users size={20} /> Participants · {peers.length}
                  {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
                  </h2>
                  {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                  {peers.length === 0 && (
                    // @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React.
                    <p className="muted">
                      Votre salle attend ses premiers participants.
                    {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                    </p>
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  )}
                  {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                  {peers.map((p) => (
                    // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
                    <div className="peer" key={p.id}>
                      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                      <div>
                        {/* @ligne Construit ou ferme l’élément « strong{p.name » de l’interface ; son contenu reste géré par React. */}
                        <strong>{p.name || "Invité"}</strong>
                        {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
                        <small>
                          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                          {((p.downloaded_bytes || 0) / 1024 / 1024).toFixed(1)}{" "}
                          Mio reçus ·{" "}
                          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                          {((p.uploaded_bytes || 0) / 1024 / 1024).toFixed(1)}{" "}
                          Mio envoyés · Dernière activité :{" "}
                          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                          {new Date(p.last_seen * 1000).toLocaleTimeString(
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                            "fr-FR",
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          )}
                        {/* @ligne Construit ou ferme l’élément « small » de l’interface ; son contenu reste géré par React. */}
                        </small>
                      {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                      </div>
                      {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                      <label>
                        {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
                        <input
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          type="checkbox"
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          checked={p.upload}
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          disabled={p.banned}
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          onChange={(e) =>
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                            run(() =>
                              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                              api("share_permission", {
                                // @ligne Renseigne le champ « id » de la structure utilisée ici du bureau et des parcours utilisateur.
                                id: p.id,
                                // @ligne Renseigne le champ « upload » de la structure utilisée ici du bureau et des parcours utilisateur.
                                upload: e.target.checked,
                                // @ligne Renseigne le champ « banned » de la structure utilisée ici du bureau et des parcours utilisateur.
                                banned: false,
                              // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                              }),
                            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                            )
                          // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                          }
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        />{" "}
                        Autoriser les blocs d’upload
                      {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                      </label>
                      {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                      <button
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        disabled={p.banned}
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        className="danger"
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        onClick={() =>
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          run(() =>
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                            api("share_permission", {
                              // @ligne Renseigne le champ « id » de la structure utilisée ici du bureau et des parcours utilisateur.
                              id: p.id,
                              // @ligne Renseigne le champ « upload » de la structure utilisée ici du bureau et des parcours utilisateur.
                              upload: false,
                              // @ligne Renseigne le champ « banned » de la structure utilisée ici du bureau et des parcours utilisateur.
                              banned: true,
                            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                            }),
                          // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                          )
                        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                        }
                      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                      >
                        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                        {p.banned ? "Expulsé" : "Expulser"}
                      {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                      </button>
                    {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                    </div>
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  ))}
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                </div>
              {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
              </>
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            )}
          {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
          </>
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        )}
        {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
        {module === "dba" && (
          // @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React.
          <>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="sql-layout">
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              <div className="panel">
                {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
                <h2>
                  {/* @ligne Construit ou ferme l’élément « FileCode » de l’interface ; son contenu reste géré par React. */}
                  <FileCode size={20} /> Votre schéma SQL
                {/* @ligne Construit ou ferme l’élément « h2 » de l’interface ; son contenu reste géré par React. */}
                </h2>
                {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                <label>
                  Nom du modèle
                  {/* @ligne Construit ou ferme l’élément « input » de l’interface ; son contenu reste géré par React. */}
                  <input
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    value={modelName}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onChange={(e) => setModelName(e.target.value)}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  />
                {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                </label>
                {/* @ligne Construit ou ferme l’élément « textarea » de l’interface ; son contenu reste géré par React. */}
                <textarea
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  aria-label="Script DDL"
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  className="sql-editor"
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  spellCheck={false}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  value={ddl}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onChange={(e) => setDdl(e.target.value)}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                />
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                <div className="actions">
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  <button
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    disabled={busy}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    className="primary"
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onClick={() =>
                      // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                      run(async () => setModel(await api("sql_parse", { ddl })))
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    }
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  >
                    {/* @ligne Construit ou ferme l’élément « Play » de l’interface ; son contenu reste géré par React. */}
                    <Play size={15} /> Analyser
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  </button>
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  <button
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    disabled={busy || !modelName.trim()}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onClick={() =>
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      run(async () => {
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        setModel(
                          // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                          await api("sql_save", { ddl, name: modelName }),
                        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                        );
                        // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                        setModels(await api("sql_models"));
                        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                        notify("Modèle enregistré.");
                      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                      })
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    }
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  >
                    Enregistrer
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  </button>
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                </div>
                {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                {models.length > 0 && (
                  // @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React.
                  <label>
                    Reprendre un modèle
                    {/* @ligne Construit ou ferme l’élément « select » de l’interface ; son contenu reste géré par React. */}
                    <select
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      value=""
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      onChange={(e) => {
                        // @ligne Prépare « m » pour la prochaine étape du bureau et des parcours utilisateur.
                        const m = models.find((m) => m.name === e.target.value);
                        // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
                        if (m) {
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          setDdl(m.ddl);
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          setModelName(m.name);
                        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                        }
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      }}
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    >
                      {/* @ligne Construit ou ferme l’élément « option » de l’interface ; son contenu reste géré par React. */}
                      <option value="">Choisir…</option>
                      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                      {models.map((m) => (
                        // @ligne Construit ou ferme l’élément « option » de l’interface ; son contenu reste géré par React.
                        <option key={m.name}>{m.name}</option>
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      ))}
                    {/* @ligne Construit ou ferme l’élément « select » de l’interface ; son contenu reste géré par React. */}
                    </select>
                  {/* @ligne Construit ou ferme l’élément « label » de l’interface ; son contenu reste géré par React. */}
                  </label>
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                )}
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              </div>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              <div>
                {/* @ligne Construit ou ferme l’élément « DatabaseConnection » de l’interface ; son contenu reste géré par React. */}
                <DatabaseConnection
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onModel={(data) => {
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    setModel(data);
                    // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
                    if (data.ddl) setDdl(data.ddl);
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  }}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onError={setError}
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                />
                {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                {model ? (
                  // @ligne Construit ou ferme l’élément « GraphCanvas » de l’interface ; son contenu reste géré par React.
                  <GraphCanvas
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    name={"modele-" + modelName}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    spec={{
                      // @ligne Renseigne le champ « nodes » de la structure utilisée ici du bureau et des parcours utilisateur.
                      nodes: model.tables.map((t: any) => ({
                        // @ligne Renseigne le champ « id » de la structure utilisée ici du bureau et des parcours utilisateur.
                        id: t.name,
                        // @ligne Renseigne le champ « title » de la structure utilisée ici du bureau et des parcours utilisateur.
                        title: t.name,
                        // @ligne Renseigne le champ « kind » de la structure utilisée ici du bureau et des parcours utilisateur.
                        kind: "table",
                        // @ligne Renseigne le champ « lines » de la structure utilisée ici du bureau et des parcours utilisateur.
                        lines: t.columns.map(
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          (c: any) =>
                            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                            `${c.name} : ${c.type} ${c.constraints.join(" ")}`,
                        // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                        ),
                      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                      })),
                      // @ligne Renseigne le champ « edges » de la structure utilisée ici du bureau et des parcours utilisateur.
                      edges: (model.relations || []).map((r: any) => ({
                        // @ligne Renseigne le champ « source » de la structure utilisée ici du bureau et des parcours utilisateur.
                        source: r.from,
                        // @ligne Renseigne le champ « target » de la structure utilisée ici du bureau et des parcours utilisateur.
                        target: r.to,
                        // @ligne Renseigne le champ « label » de la structure utilisée ici du bureau et des parcours utilisateur.
                        label:
                          // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                          r.source_cardinality + " / " + r.target_cardinality,
                      // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                      })),
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    }}
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onError={setError}
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  />
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                ) : (
                  // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
                  <div className="empty">
                    {/* @ligne Construit ou ferme l’élément « Database » de l’interface ; son contenu reste géré par React. */}
                    <Database size={32} />
                    {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                    <p>
                      Vos tables apparaîtront ici. Le SQL est analysé, jamais
                      exécuté.
                    {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                    </p>
                  {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                  </div>
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                )}
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              </div>
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
            {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
            {model && (
              // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
              <div className="panel">
                {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                <p className="help">{model.notice}</p>
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                <div className="actions">
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  <button
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onClick={() => exportJson(modelName + ".json", model)}
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  >
                    {/* @ligne Construit ou ferme l’élément « Download » de l’interface ; son contenu reste géré par React. */}
                    <Download size={15} /> Modèle JSON
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  </button>
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  <button
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onClick={() =>
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      download(modelName + ".mmd", model.uml, "text/plain")
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    }
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  >
                    UML Mermaid
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  </button>
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  <button
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    onClick={() =>
                      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                      download(modelName + "-mld.txt", model.mld, "text/plain")
                    // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                    }
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  >
                    MLD texte
                  {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                  </button>
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                </div>
                {/* @ligne Construit ou ferme l’élément « pre{model.mld}pre » de l’interface ; son contenu reste géré par React. */}
                <pre>{model.mld}</pre>
                {/* @ligne Construit ou ferme l’élément « h3Associations » de l’interface ; son contenu reste géré par React. */}
                <h3>Associations déduites des clés étrangères</h3>
                {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                {model.relations?.map((r: any, i: number) => (
                  // @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React.
                  <p key={i}>
                    {/* @ligne Construit ou ferme l’élément « code{r.from}code » de l’interface ; son contenu reste géré par React. */}
                    <code>{r.from}</code> ({r.source_cardinality}) →{" "}
                    {/* @ligne Construit ou ferme l’élément « code{r.to}code » de l’interface ; son contenu reste géré par React. */}
                    <code>{r.to}</code> ({r.target_cardinality}) ·{" "}
                    {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
                    {r.columns.join(", ")}
                  {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
                  </p>
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                ))}
                {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                <button
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  onClick={() =>
                    // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                    download(modelName + "-mcd.mmd", model.mcd, "text/plain")
                  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                  }
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                >
                  MCD Mermaid
                {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
                </button>
              {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
              </div>
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            )}
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            <div className="panel">
              {/* @ligne Construit ou ferme l’élément « h3Quelques » de l’interface ; son contenu reste géré par React. */}
              <h3>Quelques repères DBA, sans connexion</h3>
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
              {[
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                "pg_dump -Fc {{database}} > sauvegarde.dump",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                "pg_restore -d {{database}} sauvegarde.dump",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                "mysqldump -u {{user}} -p {{database}} > sauvegarde.sql",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                "sqlite3 {{database}} '.backup sauvegarde.sqlite3'",
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                "GRANT SELECT ON {{table}} TO {{user}};",
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              ].map((c) => (
                // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
                <div className="cheatsheet" key={c}>
                  {/* @ligne Construit ou ferme l’élément « code{c}code » de l’interface ; son contenu reste géré par React. */}
                  <code>{c}</code>
                  {/* @ligne Construit ou ferme l’élément « CopyButton » de l’interface ; son contenu reste géré par React. */}
                  <CopyButton text={c} onError={setError} />
                {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
                </div>
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              ))}
            {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
            </div>
          {/* @ligne Construit ou ferme l’élément «  » de l’interface ; son contenu reste géré par React. */}
          </>
        // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
        )}
      {/* @ligne Construit ou ferme l’élément « main » de l’interface ; son contenu reste géré par React. */}
      </main>
      {/* L'inspecteur donne les métadonnées connues et signale les informations absentes. */}
      {selected && module === "vault" && (
        // @ligne Construit ou ferme l’élément « aside » de l’interface ; son contenu reste géré par React.
        <aside className="inspector">
          {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
          <div className="inspector-title">
            {/* @ligne Construit ou ferme l’élément « spanINSPECTEUR » de l’interface ; son contenu reste géré par React. */}
            <span>INSPECTEUR DE COMMANDE</span>
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            <button
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              aria-label="Fermer l’inspecteur"
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              onClick={() => setSelected(null)}
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            >
              {/* @ligne Construit ou ferme l’élément « PanelRightClose » de l’interface ; son contenu reste géré par React. */}
              <PanelRightClose size={18} />
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            </button>
          {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
          </div>
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          <span className="inspector-icon">
            {/* @ligne Construit ou ferme l’élément « Terminal » de l’interface ; son contenu reste géré par React. */}
            <Terminal size={28} />
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          </span>
          {/* @ligne Construit ou ferme l’élément « h2{selected.binary}h2 » de l’interface ; son contenu reste géré par React. */}
          <h2>{selected.binary}</h2>
          {/* @ligne Construit ou ferme l’élément « p{categories[selected.category]}p » de l’interface ; son contenu reste géré par React. */}
          <p>{categories[selected.category]}</p>
          {/* @ligne Construit ou ferme l’élément « pre{selected.raw}pre » de l’interface ; son contenu reste géré par React. */}
          <pre>{selected.raw}</pre>
          {/* @ligne Construit ou ferme l’élément « CopyButton » de l’interface ; son contenu reste géré par React. */}
          <CopyButton text={selected.raw} onError={setError} />
          {/* @ligne Construit ou ferme l’élément « h3Comprendre » de l’interface ; son contenu reste géré par React. */}
          <h3>Comprendre cette capture</h3>
          {/* @ligne Construit ou ferme l’élément « dl » de l’interface ; son contenu reste géré par React. */}
          <dl>
            {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
            {Object.entries({
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              "Binaire réel": selected.binary,
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              Élévation: selected.privileged
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                ? "Oui · sudo / doas / pkexec"
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                : "Non détectée",
              // @ligne Renseigne le champ « Shell » de la structure utilisée ici du bureau et des parcours utilisateur.
              Shell: selected.shell || "Inconnu",
              // @ligne Renseigne le champ « Terminal » de la structure utilisée ici du bureau et des parcours utilisateur.
              Terminal: selected.terminal || "Non fourni",
              // @ligne Renseigne le champ « Dossier » de la structure utilisée ici du bureau et des parcours utilisateur.
              Dossier: selected.cwd || "Non fourni",
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              "Code de sortie": selected.exit_code ?? "Non fourni",
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              Durée:
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                selected.duration_ms === null
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  ? "Non mesurée"
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  : selected.duration_ms + " ms",
              // @ligne Renseigne le champ « Captures » de la structure utilisée ici du bureau et des parcours utilisateur.
              Captures: selected.count,
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            }).map(([k, v]) => (
              // @ligne Construit ou ferme l’élément « React.Fragment » de l’interface ; son contenu reste géré par React.
              <React.Fragment key={k}>
                {/* @ligne Construit ou ferme l’élément « dt{k}dt » de l’interface ; son contenu reste géré par React. */}
                <dt>{k}</dt>
                {/* @ligne Construit ou ferme l’élément « dd{v}dd » de l’interface ; son contenu reste géré par React. */}
                <dd>{v}</dd>
              {/* @ligne Construit ou ferme l’élément « React.Fragment » de l’interface ; son contenu reste géré par React. */}
              </React.Fragment>
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            ))}
          {/* @ligne Construit ou ferme l’élément « dl » de l’interface ; son contenu reste géré par React. */}
          </dl>
          {/* @ligne Construit ou ferme l’élément « CommandDetails » de l’interface ; son contenu reste géré par React. */}
          <CommandDetails command={selected} />
          {/* @ligne Construit ou ferme l’élément « h3Version » de l’interface ; son contenu reste géré par React. */}
          <h3>Version sans enveloppe</h3>
          {/* @ligne Construit ou ferme l’élément « pre{selected.clean}pre » de l’interface ; son contenu reste géré par React. */}
          <pre>{selected.clean}</pre>
          {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
          <button
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            onClick={() => {
              // @ligne Prépare « template » pour la prochaine étape du bureau et des parcours utilisateur.
              const template = selected.raw.replace(
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                /\b(?:\d{1,3}\.){3}\d{1,3}\b/g,
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                "{{target_ip}}",
              // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
              );
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              navigator.clipboard
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                .writeText(template)
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                .then(() => notify("Template copié."))
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                .catch(() => setError("Copie impossible."));
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            }}
          // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
          >
            Créer un template avec l’IP
          {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
          </button>
          {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
          <div className="help">
            Les valeurs [REDACTED] doivent être remplacées avant utilisation.
            Les commandes ne sont jamais lancées par OmniShell.
          {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
          </div>
          {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
          <button
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            className="danger"
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            onClick={() => {
              // @ligne Teste la condition de cette ligne et conserve un traitement explicite des deux issues. Contexte : du bureau et des parcours utilisateur.
              if (window.confirm("Supprimer cette commande du coffre ?"))
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                run(async () => {
                  // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                  await api("delete", { id: selected.id });
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  setSelected(null);
                  // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                  await loadCommands();
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                });
            // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
            }}
          // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
          >
            {/* @ligne Construit ou ferme l’élément « Trash2 » de l’interface ; son contenu reste géré par React. */}
            <Trash2 size={15} /> Supprimer du coffre
          {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
          </button>
        {/* @ligne Construit ou ferme l’élément « aside » de l’interface ; son contenu reste géré par React. */}
        </aside>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      )}
      {/* Le statut ne prétend pas qu'un daemon ou des hooks sont actifs sans preuve. */}
      <footer>
        {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
        <span>
          {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
          <span className="status-dot" /> Moteur local · capture par hook
          optionnel
        {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
        </span>
        {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
        <span>
          SQLite · {stats.total} commandes ·{" "}
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
          {((stats.bytes || 0) / 1024).toFixed(0)} Kio
        {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
        </span>
        {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
        <span>
          {/* @ligne Construit ou ferme l’élément « Wifi » de l’interface ; son contenu reste géré par React. */}
          <Wifi size={13} /> Aucun cloud requis
        {/* @ligne Construit ou ferme l’élément « span » de l’interface ; son contenu reste géré par React. */}
        </span>
      {/* @ligne Construit ou ferme l’élément « footer » de l’interface ; son contenu reste géré par React. */}
      </footer>
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
      {notice && (
        // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
        <div className="toast" role="status">
          {/* @ligne Construit ou ferme l’élément « Check » de l’interface ; son contenu reste géré par React. */}
          <Check size={18} />
          {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
          {notice}
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        </div>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      )}
      {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
      {settings && (
        // @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React.
        <div className="overlay" onClick={() => setSettings(false)}>
          {/* @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React. */}
          <section className="modal" onClick={(e) => e.stopPropagation()}>
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            <button
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              className="close"
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              aria-label="Fermer les réglages"
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              onClick={() => setSettings(false)}
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            >
              {/* @ligne Construit ou ferme l’élément « X » de l’interface ; son contenu reste géré par React. */}
              <X size={20} />
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            </button>
            {/* @ligne Construit ou ferme l’élément « Settings » de l’interface ; son contenu reste géré par React. */}
            <Settings size={28} />
            {/* @ligne Construit ou ferme l’élément « h2Un » de l’interface ; son contenu reste géré par React. */}
            <h2>Un atelier qui vous appartient.</h2>
            {/* @ligne Construit ou ferme l’élément « pVos » de l’interface ; son contenu reste géré par React. */}
            <p>Vos données sont stockées ici :</p>
            {/* @ligne Construit ou ferme l’élément « pre » de l’interface ; son contenu reste géré par React. */}
            <pre>
              {/* @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur. */}
              {stats.database || "Répertoire de données local du système"}
            {/* @ligne Construit ou ferme l’élément « pre » de l’interface ; son contenu reste géré par React. */}
            </pre>
            {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
            <p>
              La capture continue nécessite l’ajout volontaire d’un hook à votre
              profil. Les instructions et les limites sont détaillées dans le
              README.
            {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
            </p>
            {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
            <p>
              Aucune télémétrie. Aucune synchronisation cloud. Aucun privilège
              administrateur demandé automatiquement.
            {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
            </p>
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            <button
              // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
              onClick={() =>
                // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                run(async () => {
                  // @ligne Prépare « report » pour la prochaine étape du bureau et des parcours utilisateur.
                  const report = await api("import");
                  // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
                  notify(JSON.stringify(report));
                  // @ligne Effectue l’opération indiquée et transmet son résultat ou son erreur dans le parcours du bureau et des parcours utilisateur.
                  await loadCommands();
                // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
                })
              // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
              }
            // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
            >
              {/* @ligne Construit ou ferme l’élément « RefreshCw » de l’interface ; son contenu reste géré par React. */}
              <RefreshCw size={16} /> Relire les historiques
            {/* @ligne Construit ou ferme l’élément « button » de l’interface ; son contenu reste géré par React. */}
            </button>
            {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
            <p className="help">
              OmniShell Studio · Kossi Bernard Etonam ADJOGBLE · version 1.0 RC1
            {/* @ligne Construit ou ferme l’élément « p » de l’interface ; son contenu reste géré par React. */}
            </p>
          {/* @ligne Construit ou ferme l’élément « section » de l’interface ; son contenu reste géré par React. */}
          </section>
        {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
        </div>
      // @ligne Complète l’expression en cours sans changer le contexte parent. Cette étape participe à la mise en œuvre du bureau et des parcours utilisateur.
      )}
    {/* @ligne Construit ou ferme l’élément « div » de l’interface ; son contenu reste géré par React. */}
    </div>
  // @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
  );
// @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
}
// StrictMode repère les effets non nettoyés pendant le développement.
createRoot(document.getElementById("root")!).render(
  // @ligne Construit ou ferme l’élément « React.StrictMode » de l’interface ; son contenu reste géré par React.
  <React.StrictMode>
    {/* @ligne Construit ou ferme l’élément « ErrorBoundary » de l’interface ; son contenu reste géré par React. */}
    <ErrorBoundary>
      {/* @ligne Construit ou ferme l’élément « App » de l’interface ; son contenu reste géré par React. */}
      <App />
    {/* @ligne Construit ou ferme l’élément « ErrorBoundary » de l’interface ; son contenu reste géré par React. */}
    </ErrorBoundary>
  {/* @ligne Construit ou ferme l’élément « React.StrictMode, » de l’interface ; son contenu reste géré par React. */}
  </React.StrictMode>,
// @ligne Termine cette construction et revient au contexte parent du bureau et des parcours utilisateur.
);
