// React pilote la fenêtre ; aucune ressource distante n'est chargée au démarrage.
import React, { useState, useEffect, useCallback, useRef } from "react";
import { createRoot } from "react-dom/client";
import {
  Terminal,
  Search,
  Command as CommandIcon,
  Container,
  Network,
  FolderOpen,
  Database,
  Settings,
  Plus,
  ArrowUpRight,
  Copy,
  Star,
  Check,
  RefreshCw,
  ChevronRight,
  Shield,
  Trash2,
  Download,
  X,
  Wifi,
  Sun,
  PanelRightClose,
  Play,
  FileCode,
  Link,
  Users,
  StopCircle,
} from "lucide-react";
import { api, download, type Command, type Snapshot } from "./api";
import "./style.css";
// Les catégories sont partagées visuellement par navigation et filtres.
const categories: Record<string, string> = {
  containers: "Conteneurs",
  network: "Réseau & sécurité",
  system: "Système & paquets",
  files: "Fichiers & navigation",
  git: "Git & versions",
  database: "Bases de données",
  ai: "IA & Python",
  development: "Développement",
  other: "Autres commandes",
};
// Chaque module possède une identité courte et une aide contextualisée.
const modules = [
  {
    id: "vault",
    name: "Command Vault",
    icon: Terminal,
    tag: "Votre mémoire de travail",
    title: "Les bonnes commandes. À portée de main.",
    desc: "Retrouvez, comprenez et réutilisez ce qui fonctionne.",
  },
  {
    id: "docker",
    name: "Docker & Infra",
    icon: Container,
    tag: "Infrastructure locale",
    title: "Votre infrastructure, même hors ligne.",
    desc: "Captures persistantes et raccourcis prêts à copier.",
  },
  {
    id: "netmap",
    name: "NetMap Studio",
    icon: Network,
    tag: "Explorer le réseau",
    title: "Donnez un visage à votre réseau.",
    desc: "Observez les services accessibles, sans inventer les liens physiques.",
  },
  {
    id: "share",
    name: "ClassShare LAN",
    icon: FolderOpen,
    tag: "Travailler ensemble",
    title: "Une salle. Un réseau. Vos fichiers.",
    desc: "Partagez sur votre LAN et gardez la main sur les permissions.",
  },
  {
    id: "dba",
    name: "DBA & Merise",
    icon: Database,
    tag: "Comprendre les données",
    title: "Du SQL à une structure lisible.",
    desc: "Analysez votre DDL localement et conservez vos modèles.",
  },
];
// Un composant unique garantit le même retour de copie partout dans l'atelier.
function CopyButton({
  text,
  label = "Copier",
  onError,
}: {
  text: string;
  label?: string;
  onError: (s: string) => void;
}) {
  const [copied, setCopied] = useState(false); // État bref de confirmation, propre à chaque bouton.
  return (
    <button
      className="subtle"
      onClick={async (e) => {
        e.stopPropagation();
        try {
          let value = text;
          for (const match of text.matchAll(/\{\{(\w+)\}\}/g)) {
            const answer = window.prompt("Valeur pour " + match[1]);
            if (answer === null) return;
            value = value.replaceAll(match[0], answer);
          }
          await navigator.clipboard.writeText(value);
          setCopied(true);
          setTimeout(() => setCopied(false), 2200);
        } catch {
          onError(
            "Presse-papier inaccessible. Sélectionnez le texte pour le copier.",
          );
        }
      }}
    >
      {copied ? <Check size={15} /> : <Copy size={15} />}{" "}
      {copied ? "Copié !" : label}
    </button>
  ); // La substitution ne déclenche jamais une exécution.
}
// Une erreur de rendu garde un message lisible au lieu d'une fenêtre blanche.
class ErrorBoundary extends React.Component<
  { children: React.ReactNode },
  { error: boolean }
> {
  state = { error: false };
  static getDerivedStateFromError() {
    return { error: true };
  }
  render() {
    return this.state.error ? (
      <main className="fatal">
        <h1>L’atelier a rencontré une erreur.</h1>
        <p>Vos données sont conservées sur disque.</p>
        <button onClick={() => location.reload()}>Rouvrir l’atelier</button>
      </main>
    ) : (
      this.props.children
    );
  }
}
// L'application utilise des requêtes courtes et des états de chargement explicites.
function App() {
  const [module, setModule] = useState("vault"),
    [query, setQuery] = useState(""),
    [category, setCategory] = useState(""),
    [favorites, setFavorites] = useState(false),
    [commands, setCommands] = useState<Command[]>([]),
    [total, setTotal] = useState(0),
    [offset, setOffset] = useState(0),
    [stats, setStats] = useState<any>({ total: 0, categories: [] }),
    [selected, setSelected] = useState<Command | null>(null),
    [error, setError] = useState(""),
    [notice, setNotice] = useState(""),
    [busy, setBusy] = useState(false),
    [settings, setSettings] = useState(false),
    [interfaces, setInterfaces] = useState<any[]>([]),
    [ip, setIp] = useState(""),
    [docker, setDocker] = useState<Snapshot[]>([]),
    [graph, setGraph] = useState<any>(null),
    [cidr, setCidr] = useState("192.168.1.0/24"),
    [room, setRoom] = useState<any>(null),
    [peers, setPeers] = useState<any[]>([]),
    [paths, setPaths] = useState(""),
    [ddl, setDdl] = useState(
      "CREATE TABLE utilisateurs (\n  id INTEGER PRIMARY KEY,\n  nom VARCHAR(120) NOT NULL\n);\n\nCREATE TABLE projets (\n  id INTEGER PRIMARY KEY,\n  titre VARCHAR(200),\n  auteur_id INTEGER REFERENCES utilisateurs(id)\n);",
    ),
    [model, setModel] = useState<any>(null),
    [modelName, setModelName] = useState("Mon modèle"),
    [models, setModels] = useState<any[]>([]); // Chaque domaine conserve son état lors d'un changement d'onglet.
  const searchRef = useRef<HTMLInputElement>(null),
    requestId = useRef(0); // Permet le focus clavier et écarte les recherches obsolètes.
  const current = modules.find((m) => m.id === module)!; // Les identifiants proviennent uniquement de la navigation.
  const notify = (message: string) => {
    setNotice(message);
    setTimeout(() => setNotice(""), 5000);
  }; // Retour d'action distinct des erreurs persistantes.
  const run = async <T,>(job: () => Promise<T>): Promise<T | undefined> => {
    setBusy(true);
    setError("");
    try {
      return await job();
    } catch (e) {
      setError(String(e));
      return undefined;
    } finally {
      setBusy(false);
    }
  }; // Les échecs de service n'effacent pas les caches affichés.
  const loadCommands = useCallback(async () => {
    const id = ++requestId.current;
    try {
      const data = await api<{ items: Command[]; total: number }>("commands", {
        query,
        category,
        favorites,
        offset,
      });
      if (id === requestId.current) {
        setCommands(data.items);
        setTotal(data.total);
      }
      setStats(await api("stats"));
    } catch (e) {
      setError(String(e));
    }
  }, [query, category, favorites, offset]); // La dernière requête seule remplace la liste.
  useEffect(() => {
    const timer = setTimeout(loadCommands, 150);
    return () => clearTimeout(timer);
  }, [loadCommands]); // Réduit les accès disque durant la frappe.
  useEffect(() => {
    setOffset(0);
  }, [query, category, favorites]); // Un nouveau filtre revient à la première page.
  useEffect(() => {
    const key = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "k") {
        e.preventDefault();
        setModule("vault");
        searchRef.current?.focus();
      }
      if (e.key === "Escape") {
        setSelected(null);
        setSettings(false);
      }
    };
    window.addEventListener("keydown", key);
    api<any[]>("interfaces")
      .then((data) => {
        setInterfaces(data);
        const first = data.find((i) => /^\d+\./.test(i.ip));
        if (first) {
          setIp(first.ip);
          setCidr(first.ip.split(".").slice(0, 3).join(".") + ".0/24");
        }
      })
      .catch(() => {});
    return () => window.removeEventListener("keydown", key);
  }, []); // Aucun scan réseau au démarrage.
  useEffect(() => {
    if (module === "docker")
      api<Snapshot[]>("docker_cache")
        .then(setDocker)
        .catch((e) => setError(String(e)));
    if (module === "netmap")
      api<Snapshot[]>("net_cache")
        .then((data) => {
          if (data[0]) setGraph(data[0].data);
        })
        .catch(() => {});
    if (module === "dba")
      api<any[]>("sql_models")
        .then(setModels)
        .catch(() => {});
  }, [module]); // Les vues utilisent d'abord les données hors ligne.
  useEffect(() => {
    if (module !== "share") return;
    const refresh = () =>
      api("share_status")
        .then((data) => {
          setRoom(data.room);
          setPeers(data.peers);
        })
        .catch(() => {});
    refresh();
    const timer = setInterval(refresh, 2000);
    return () => clearInterval(timer);
  }, [module]); // Le polling administrateur ne sort pas de l'IPC local.
  const favorite = async (c: Command) => {
    await run(() => api("favorite", { id: c.id }));
    await loadCommands();
    if (selected?.id === c.id) setSelected({ ...c, favorite: !c.favorite });
  }; // Met à jour aussi l'inspecteur ouvert.
  const exportJson = (name: string, value: unknown) =>
    download(name, JSON.stringify(value, null, 2)); // Exports lisibles et indépendants du service.
  return (
    <div className="app">
      {/* L'en-tête reste fixe et accessible depuis chaque module. */}
      <header>
        <div className="brand">
          <span className="logo">
            <CommandIcon size={22} />
          </span>
          <strong>
            OmniShell<span> Studio</span>
          </strong>
          <small>LOCAL</small>
        </div>
        <div className="search">
          <Search size={17} />
          <input
            ref={searchRef}
            aria-label="Rechercher une commande"
            placeholder="Retrouver une commande, un outil…"
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setModule("vault");
            }}
          />
          <kbd>⌃ K</kbd>
        </div>
        <button className="primary" onClick={() => setModule("share")}>
          <Plus size={16} /> Quick Share
        </button>
        <button
          className="icon-button"
          aria-label="Réglages"
          onClick={() => setSettings(true)}
        >
          <Settings size={19} />
        </button>
      </header>
      {/* La barre latérale présente les modules, puis la taxonomie du coffre. */}
      <aside className="sidebar">
        <div className="workspace">
          <span className="avatar">OS</span>
          <div>
            <strong>Mon espace local</strong>
            <small>Personnel · hors ligne</small>
          </div>
          <ChevronRight size={14} />
        </div>
        <p className="eyebrow">ATELIER</p>
        <nav>
          {modules.map((m) => (
            <button
              key={m.id}
              className={module === m.id ? "nav active" : "nav"}
              onClick={() => {
                setModule(m.id);
                setSelected(null);
                setError("");
              }}
            >
              <m.icon size={18} />
              {m.name}
              {module === m.id && <span className="nav-dot" />}
            </button>
          ))}
        </nav>
        <p className="eyebrow category-heading">EXPLORER LES COMMANDES</p>
        <button
          className={"category " + (!category ? "chosen" : "")}
          onClick={() => {
            setCategory("");
            setModule("vault");
          }}
        >
          <span>Toutes les catégories</span>
          <small>{stats.total}</small>
        </button>
        {Object.entries(categories).map(([key, name]) => (
          <button
            key={key}
            className={"category " + (category === key ? "chosen" : "")}
            onClick={() => {
              setCategory(key);
              setModule("vault");
            }}
          >
            <span>
              <span className={"dot " + key} />
              {name}
            </span>
            <small>
              {stats.categories.find((c: any) => c[0] === key)?.[1] || 0}
            </small>
          </button>
        ))}
        <div className="local-card">
          <Shield size={19} />
          <strong>Votre machine. Vos données.</strong>
          <p>
            Aucun cloud, aucune télémétrie.
            <br />
            Un atelier qui reste avec vous.
          </p>
          <span>
            <span className="status-dot" /> Local-first, par conception
          </span>
        </div>
        <div className="sidebar-footer">
          <Sun size={16} /> Daylight workspace <small>v0.1.0</small>
        </div>
      </aside>
      {/* Le canevas central explique l'état réel de chaque service. */}
      <main
        className={
          selected && module === "vault" ? "content with-inspector" : "content"
        }
      >
        <div className="breadcrumb">
          Mon espace <ChevronRight size={12} /> {current.name}
          <span>
            <span className="status-dot" /> Stockage local
          </span>
        </div>
        <div className="page-heading">
          <div>
            <p className="overline">{current.tag}</p>
            <h1>{current.title}</h1>
            <p>{current.desc}</p>
          </div>
          <span className="heading-icon">
            <current.icon size={31} />
          </span>
        </div>
        {error && (
          <div className="alert" role="alert">
            <Shield size={17} />
            <span>{error}</span>
            <button aria-label="Fermer le message" onClick={() => setError("")}>
              <X size={15} />
            </button>
          </div>
        )}
        {module === "vault" && (
          <>
            <div className="metrics">
              <div>
                <span>Commandes mémorisées</span>
                <strong>
                  {stats.total.toLocaleString("fr-FR")}
                  <small>dans votre coffre</small>
                </strong>
              </div>
              <div>
                <span>Organisation automatique</span>
                <strong>
                  {stats.categories.length}
                  <small>catégories utilisées</small>
                </strong>
              </div>
              <div>
                <span>Conçu pour durer</span>
                <strong>
                  100 %<small>sur votre machine</small>
                </strong>
              </div>
            </div>
            <div className="toolbar">
              <div className="tabs">
                <button
                  className={!favorites ? "tab selected" : "tab"}
                  onClick={() => setFavorites(false)}
                >
                  Toutes les commandes <small>{total}</small>
                </button>
                <button
                  className={favorites ? "tab selected" : "tab"}
                  onClick={() => setFavorites(true)}
                >
                  <Star size={14} /> Favoris
                </button>
              </div>
              <button
                disabled={busy}
                onClick={() =>
                  run(async () => {
                    const r = await api("import");
                    notify(JSON.stringify(r.sources));
                    await loadCommands();
                  })
                }
              >
                <RefreshCw size={15} className={busy ? "spin" : ""} /> Importer
                les historiques
              </button>
              <button
                aria-label="Exporter la page de commandes"
                onClick={() => exportJson("omnishell-commandes.json", commands)}
              >
                <Download size={16} />
              </button>
            </div>
            <div className="list-label">
              <span>
                {category ? categories[category] : "VOTRE MÉMOIRE DE TERMINAL"}
              </span>
              <span>Les plus récentes d’abord</span>
            </div>
            <div className="command-list">
              {commands.map((c) => (
                <article
                  tabIndex={0}
                  onKeyDown={(e) => {
                    if (e.key === "Enter") setSelected(c);
                  }}
                  onClick={() => setSelected(c)}
                  key={c.id}
                  className={
                    "command-card " + (selected?.id === c.id ? "focused" : "")
                  }
                >
                  <span className={"binary-icon " + c.category}>
                    <Terminal size={19} />
                  </span>
                  <div className="command-body">
                    <div className="command-title">
                      <span className="badge">{c.binary || "shell"}</span>
                      {c.privileged && <span className="badge sudo">SUDO</span>}
                      {c.raw.includes("[REDACTED]") && (
                        <span className="badge safe">Secrets masqués</span>
                      )}
                      <small>
                        {new Date(c.last_seen * 1000).toLocaleDateString(
                          "fr-FR",
                        )}
                      </small>
                    </div>
                    <code>{c.raw}</code>
                    <div className="metadata">
                      <span>{categories[c.category]}</span>
                      <i /> {c.count} capture{c.count > 1 ? "s" : ""} <i />{" "}
                      {c.shell || "Shell inconnu"}
                      {c.cwd && (
                        <>
                          <i />
                          <span className="path">{c.cwd}</span>
                        </>
                      )}
                    </div>
                  </div>
                  <div className="card-actions">
                    <button
                      aria-label="Basculer favori"
                      className={c.favorite ? "star favorite" : "star"}
                      onClick={(e) => {
                        e.stopPropagation();
                        favorite(c);
                      }}
                    >
                      <Star
                        size={17}
                        fill={c.favorite ? "currentColor" : "none"}
                      />
                    </button>
                    <CopyButton text={c.raw} onError={setError} />
                    <ChevronRight size={15} />
                  </div>
                </article>
              ))}
              {commands.length === 0 && (
                <div className="empty">
                  <Terminal size={38} />
                  <h2>
                    {query
                      ? "Aucune commande correspondante"
                      : "Votre mémoire de terminal commence ici."}
                  </h2>
                  <p>
                    Importez vos historiques existants ou ajoutez le hook
                    documenté pour capturer vos prochaines commandes.
                  </p>
                  <button
                    className="primary"
                    onClick={() =>
                      run(async () => {
                        await api("import");
                        await loadCommands();
                        notify("Import terminé.");
                      })
                    }
                  >
                    <Plus size={16} /> Importer mon historique
                  </button>
                </div>
              )}
            </div>
            <div className="pagination">
              <span>
                {total ? offset + 1 : 0}–{Math.min(offset + 100, total)} sur{" "}
                {total}
              </span>
              <button
                disabled={offset === 0}
                onClick={() => setOffset(Math.max(0, offset - 100))}
              >
                Précédent
              </button>
              <button
                disabled={offset + 100 >= total}
                onClick={() => setOffset(offset + 100)}
              >
                Suivant
              </button>
            </div>
            <div className="tip">
              <span>✦</span>
              <p>
                <strong>Le petit réflexe qui change tout.</strong> Ajoutez vos
                commandes utiles aux favoris. Votre prochain dépannage vous dira
                merci.
              </p>
              <kbd>Ctrl K</kbd>
            </div>
          </>
        )}
        {module === "docker" && (
          <>
            <div className="toolbar">
              <span className="badge sudo">
                Cache persistant · actualisation manuelle
              </span>
              <button
                disabled={busy}
                className="primary"
                onClick={() =>
                  run(async () => {
                    const r = await api("docker_sync");
                    setDocker(r.snapshots);
                    notify(`${r.updated} conteneur(s) actualisé(s).`);
                  })
                }
              >
                <RefreshCw size={16} /> Synchroniser Docker
              </button>
            </div>
            <div className="grid">
              {docker.map((s) => (
                <article className="panel" key={s.data.id}>
                  <span className="badge">{s.data.state}</span>
                  <h2>
                    <Container size={22} /> {s.data.name}
                  </h2>
                  <code>{s.data.image}</code>
                  <p className="muted">
                    Capture du{" "}
                    {new Date(s.captured_at * 1000).toLocaleString("fr-FR")}
                  </p>
                  <details>
                    <summary>Ports, volumes & environnement masqué</summary>
                    <pre>
                      {JSON.stringify(
                        {
                          ports: s.data.ports,
                          mounts: s.data.mounts,
                          environment: s.data.environment,
                        },
                        null,
                        2,
                      )}
                    </pre>
                  </details>
                  <div className="action-grid">
                    {Object.entries(s.data.actions || {}).map(
                      ([name, value]) => (
                        <CopyButton
                          key={name}
                          text={String(value)}
                          label={name}
                          onError={setError}
                        />
                      ),
                    )}
                  </div>
                  <details>
                    <summary>Commande reconstruite (partielle)</summary>
                    <pre>{s.data.reconstructed}</pre>
                    <CopyButton
                      text={s.data.reconstructed}
                      onError={setError}
                    />
                  </details>
                  <details>
                    <summary>Derniers logs masqués</summary>
                    <pre>{s.data.logs}</pre>
                  </details>
                </article>
              ))}
            </div>
            {docker.length === 0 && (
              <div className="empty">
                <Container size={40} />
                <h2>Vos conteneurs ont aussi une mémoire.</h2>
                <p>
                  Démarrez Docker, puis synchronisez. Les captures resteront
                  disponibles après son arrêt.
                </p>
              </div>
            )}
          </>
        )}
        {module === "netmap" && (
          <>
            <div className="toolbar">
              <input
                aria-label="Sous-réseau privé à scanner"
                value={cidr}
                onChange={(e) => setCidr(e.target.value)}
              />
              <button
                disabled={busy}
                className="primary"
                onClick={() =>
                  run(async () => {
                    setGraph(await api("net_scan", { cidr }));
                    notify("Sondes terminées.");
                  })
                }
              >
                <Network size={16} />{" "}
                {busy ? "Observation en cours…" : "Observer ce /24"}
              </button>
              <button
                disabled={!graph}
                onClick={() => exportJson("omnishell-reseau.json", graph)}
              >
                <Download size={16} /> JSON
              </button>
              <button disabled={!graph} onClick={() => window.print()}>
                Imprimer / PDF
              </button>
            </div>
            <div className="topology">
              <div className="network-root">
                <Network size={26} />
                <strong>{graph?.cidr || cidr}</strong>
                <small>Réseau observé · TCP connect</small>
              </div>
              <div className="nodes">
                {graph?.nodes?.map((node: any) => (
                  <div className="network-node" key={node.id}>
                    <span className="status-dot" />
                    <strong>{node.ip}</strong>
                    <span>{node.kind}</span>
                    <small>Ports : {node.ports.join(", ")}</small>
                    <CopyButton
                      text={"ssh utilisateur@" + node.ip}
                      label="SSH"
                      onError={setError}
                    />
                    <CopyButton
                      text={"nmap -sV " + node.ip}
                      label="Nmap"
                      onError={setError}
                    />
                  </div>
                ))}
              </div>
              {!graph?.nodes?.length && (
                <p className="muted">
                  Aucun service observé. Un appareil sans port ouvert peut
                  rester invisible.
                </p>
              )}
            </div>
            <p className="help">
              {graph?.notice ||
                "Scan non privilégié limité à un réseau privé /24. Aucun accès Internet ni collecte FAI. Les liens physiques ne sont pas déduits."}
            </p>
            <div className="panel">
              <h3>Interfaces de cette machine</h3>
              {interfaces.map((i) => (
                <p key={i.name + i.ip}>
                  <code>{i.name}</code> · {i.ip} / {i.netmask}
                </p>
              ))}
            </div>
          </>
        )}
        {module === "share" && (
          <>
            <div className="panel">
              <h2>
                <FolderOpen size={22} /> Préparer une salle
              </h2>
              <p>
                Un chemin absolu par ligne. Pour un dossier, créez d’abord une
                archive.
              </p>
              <label>
                Fichiers à partager
                <textarea
                  value={paths}
                  onChange={(e) => setPaths(e.target.value)}
                  placeholder={
                    "/home/moi/Documents/TP-reseau.zip\n/home/moi/Documents/cours.pdf"
                  }
                />
              </label>
              <label>
                Interface LAN
                <select value={ip} onChange={(e) => setIp(e.target.value)}>
                  {interfaces
                    .filter((i) => /^\d+\./.test(i.ip))
                    .map((i) => (
                      <option value={i.ip} key={i.name + i.ip}>
                        {i.name} · {i.ip}
                      </option>
                    ))}
                </select>
              </label>
              <div className="actions">
                <button
                  className="primary"
                  disabled={busy || !paths.trim() || !ip}
                  onClick={() =>
                    run(async () => {
                      setRoom(
                        await api("share_start", {
                          ip,
                          paths: paths
                            .split("\n")
                            .map((p) => p.trim())
                            .filter(Boolean),
                        }),
                      );
                      notify("Salle ouverte sur votre interface LAN.");
                    })
                  }
                >
                  <Plus size={16} /> Ouvrir la salle
                </button>
                {room && (
                  <button
                    className="danger"
                    onClick={() =>
                      run(async () => {
                        await api("share_stop");
                        setRoom(null);
                        setPeers([]);
                      })
                    }
                  >
                    <StopCircle size={16} /> Fermer la salle
                  </button>
                )}
              </div>
              <p className="help">
                Transport TLS avec certificat local auto-signé. Communiquez le
                certificat aux participants pour valider la première connexion.
                Aucune règle de pare-feu n’est modifiée automatiquement.
              </p>
            </div>
            {room && (
              <>
                <div className="room-card">
                  <div>
                    <span className="overline">VOTRE CODE DE SALLE</span>
                    <div className="pin">
                      {room.pin.slice(0, 3)} {room.pin.slice(3)}
                    </div>
                    <code>{room.url}</code>
                    <CopyButton
                      text={room.url}
                      label="Copier le lien"
                      onError={setError}
                    />
                    <p>
                      Le code accompagne le lien ; il ne remplace pas une
                      adresse réseau.
                    </p>
                    <details>
                      <summary>Certificat et dossier de réception</summary>
                      <pre>{room.certificate}</pre>
                      <small>
                        Empreinte BLAKE3 du PEM : {room.fingerprint}
                      </small>
                      <pre>{room.upload_dir}</pre>
                    </details>
                  </div>
                  <img
                    alt="QR code du lien vers la salle"
                    width="180"
                    src={
                      "data:image/svg+xml;charset=utf-8," +
                      encodeURIComponent(room.qr)
                    }
                  />
                </div>
                <div className="panel">
                  <h2>
                    <Users size={20} /> Participants · {peers.length}
                  </h2>
                  {peers.length === 0 && (
                    <p className="muted">
                      Votre salle attend ses premiers participants.
                    </p>
                  )}
                  {peers.map((p) => (
                    <div className="peer" key={p.id}>
                      <div>
                        <strong>{p.name || "Invité"}</strong>
                        <small>
                          Dernière activité :{" "}
                          {new Date(p.last_seen * 1000).toLocaleTimeString(
                            "fr-FR",
                          )}
                        </small>
                      </div>
                      <label>
                        <input
                          type="checkbox"
                          checked={p.upload}
                          disabled={p.banned}
                          onChange={(e) =>
                            run(() =>
                              api("share_permission", {
                                id: p.id,
                                upload: e.target.checked,
                                banned: false,
                              }),
                            )
                          }
                        />{" "}
                        Autoriser les blocs d’upload
                      </label>
                      <button
                        disabled={p.banned}
                        className="danger"
                        onClick={() =>
                          run(() =>
                            api("share_permission", {
                              id: p.id,
                              upload: false,
                              banned: true,
                            }),
                          )
                        }
                      >
                        {p.banned ? "Expulsé" : "Expulser"}
                      </button>
                    </div>
                  ))}
                </div>
              </>
            )}
          </>
        )}
        {module === "dba" && (
          <>
            <div className="sql-layout">
              <div className="panel">
                <h2>
                  <FileCode size={20} /> Votre schéma SQL
                </h2>
                <label>
                  Nom du modèle
                  <input
                    value={modelName}
                    onChange={(e) => setModelName(e.target.value)}
                  />
                </label>
                <textarea
                  aria-label="Script DDL"
                  className="sql-editor"
                  spellCheck={false}
                  value={ddl}
                  onChange={(e) => setDdl(e.target.value)}
                />
                <div className="actions">
                  <button
                    disabled={busy}
                    className="primary"
                    onClick={() =>
                      run(async () => setModel(await api("sql_parse", { ddl })))
                    }
                  >
                    <Play size={15} /> Analyser
                  </button>
                  <button
                    disabled={busy || !modelName.trim()}
                    onClick={() =>
                      run(async () => {
                        setModel(
                          await api("sql_save", { ddl, name: modelName }),
                        );
                        setModels(await api("sql_models"));
                        notify("Modèle enregistré.");
                      })
                    }
                  >
                    Enregistrer
                  </button>
                </div>
                {models.length > 0 && (
                  <label>
                    Reprendre un modèle
                    <select
                      value=""
                      onChange={(e) => {
                        const m = models.find((m) => m.name === e.target.value);
                        if (m) {
                          setDdl(m.ddl);
                          setModelName(m.name);
                        }
                      }}
                    >
                      <option value="">Choisir…</option>
                      {models.map((m) => (
                        <option key={m.name}>{m.name}</option>
                      ))}
                    </select>
                  </label>
                )}
              </div>
              <div className="schema-canvas">
                {model ? (
                  model.tables.map((t: any) => (
                    <article className="entity" key={t.name}>
                      <h3>
                        <Database size={16} /> {t.name}
                      </h3>
                      {t.columns.map((c: any) => (
                        <div key={c.name}>
                          <strong>{c.name}</strong>
                          <code>{c.type}</code>
                          <small>{c.constraints.join(" · ")}</small>
                        </div>
                      ))}
                      {t.constraints.map((c: string) => (
                        <p key={c}>{c}</p>
                      ))}
                    </article>
                  ))
                ) : (
                  <div className="empty">
                    <Database size={32} />
                    <p>
                      Vos tables apparaîtront ici.
                      <br />
                      Le SQL est analysé, jamais exécuté.
                    </p>
                  </div>
                )}
              </div>
            </div>
            {model && (
              <div className="panel">
                <p className="help">{model.notice}</p>
                <div className="actions">
                  <button
                    onClick={() => exportJson(modelName + ".json", model)}
                  >
                    <Download size={15} /> Modèle JSON
                  </button>
                  <button
                    onClick={() =>
                      download(modelName + ".mmd", model.uml, "text/plain")
                    }
                  >
                    UML Mermaid
                  </button>
                  <button
                    onClick={() =>
                      download(modelName + "-mld.txt", model.mld, "text/plain")
                    }
                  >
                    MLD texte
                  </button>
                </div>
                <pre>{model.mld}</pre>
                <h3>Associations déduites des clés étrangères</h3>
                {model.relations?.map((r: any, i: number) => (
                  <p key={i}>
                    <code>{r.from}</code> ({r.source_cardinality}) →{" "}
                    <code>{r.to}</code> ({r.target_cardinality}) ·{" "}
                    {r.columns.join(", ")}
                  </p>
                ))}
                <button
                  onClick={() =>
                    download(modelName + "-mcd.mmd", model.mcd, "text/plain")
                  }
                >
                  MCD Mermaid
                </button>
              </div>
            )}
            <div className="panel">
              <h3>Quelques repères DBA, sans connexion</h3>
              {[
                "pg_dump -Fc {{database}} > sauvegarde.dump",
                "pg_restore -d {{database}} sauvegarde.dump",
                "mysqldump -u {{user}} -p {{database}} > sauvegarde.sql",
                "sqlite3 {{database}} '.backup sauvegarde.sqlite3'",
                "GRANT SELECT ON {{table}} TO {{user}};",
              ].map((c) => (
                <div className="cheatsheet" key={c}>
                  <code>{c}</code>
                  <CopyButton text={c} onError={setError} />
                </div>
              ))}
            </div>
          </>
        )}
      </main>
      {/* L'inspecteur donne les métadonnées connues et signale les informations absentes. */}
      {selected && module === "vault" && (
        <aside className="inspector">
          <div className="inspector-title">
            <span>INSPECTEUR DE COMMANDE</span>
            <button
              aria-label="Fermer l’inspecteur"
              onClick={() => setSelected(null)}
            >
              <PanelRightClose size={18} />
            </button>
          </div>
          <span className="inspector-icon">
            <Terminal size={28} />
          </span>
          <h2>{selected.binary}</h2>
          <p>{categories[selected.category]}</p>
          <pre>{selected.raw}</pre>
          <CopyButton text={selected.raw} onError={setError} />
          <h3>Comprendre cette capture</h3>
          <dl>
            {Object.entries({
              "Binaire réel": selected.binary,
              Élévation: selected.privileged
                ? "Oui · sudo / doas / pkexec"
                : "Non détectée",
              Shell: selected.shell || "Inconnu",
              Terminal: selected.terminal || "Non fourni",
              Dossier: selected.cwd || "Non fourni",
              "Code de sortie": selected.exit_code ?? "Non fourni",
              Durée:
                selected.duration_ms === null
                  ? "Non mesurée"
                  : selected.duration_ms + " ms",
              Captures: selected.count,
            }).map(([k, v]) => (
              <React.Fragment key={k}>
                <dt>{k}</dt>
                <dd>{v}</dd>
              </React.Fragment>
            ))}
          </dl>
          <h3>Version sans enveloppe</h3>
          <pre>{selected.clean}</pre>
          <button
            onClick={() => {
              const template = selected.raw.replace(
                /\b(?:\d{1,3}\.){3}\d{1,3}\b/g,
                "{{target_ip}}",
              );
              navigator.clipboard
                .writeText(template)
                .then(() => notify("Template copié."))
                .catch(() => setError("Copie impossible."));
            }}
          >
            Créer un template avec l’IP
          </button>
          <div className="help">
            Les valeurs [REDACTED] doivent être remplacées avant utilisation.
            Les commandes ne sont jamais lancées par OmniShell.
          </div>
          <button
            className="danger"
            onClick={() => {
              if (window.confirm("Supprimer cette commande du coffre ?"))
                run(async () => {
                  await api("delete", { id: selected.id });
                  setSelected(null);
                  await loadCommands();
                });
            }}
          >
            <Trash2 size={15} /> Supprimer du coffre
          </button>
        </aside>
      )}
      {/* Le statut ne prétend pas qu'un daemon ou des hooks sont actifs sans preuve. */}
      <footer>
        <span>
          <span className="status-dot" /> Moteur local · capture par hook
          optionnel
        </span>
        <span>
          SQLite · {stats.total} commandes ·{" "}
          {((stats.bytes || 0) / 1024).toFixed(0)} Kio
        </span>
        <span>
          <Wifi size={13} /> Aucun cloud requis
        </span>
      </footer>
      {notice && (
        <div className="toast" role="status">
          <Check size={18} />
          {notice}
        </div>
      )}
      {settings && (
        <div className="overlay" onClick={() => setSettings(false)}>
          <section className="modal" onClick={(e) => e.stopPropagation()}>
            <button
              className="close"
              aria-label="Fermer les réglages"
              onClick={() => setSettings(false)}
            >
              <X size={20} />
            </button>
            <Settings size={28} />
            <h2>Un atelier qui vous appartient.</h2>
            <p>Vos données sont stockées ici :</p>
            <pre>
              {stats.database || "Répertoire de données local du système"}
            </pre>
            <p>
              La capture continue nécessite l’ajout volontaire d’un hook à votre
              profil. Les instructions et les limites sont détaillées dans le
              README.
            </p>
            <p>
              Aucune télémétrie. Aucune synchronisation cloud. Aucun privilège
              administrateur demandé automatiquement.
            </p>
            <button
              onClick={() =>
                run(async () => {
                  const report = await api("import");
                  notify(JSON.stringify(report));
                  await loadCommands();
                })
              }
            >
              <RefreshCw size={16} /> Relire les historiques
            </button>
            <p className="help">
              OmniShell Studio · Kossi Bernard Etonam ADJOGBLE · version 0.1.0
            </p>
          </section>
        </div>
      )}
    </div>
  );
}
// StrictMode repère les effets non nettoyés pendant le développement.
createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ErrorBoundary>
      <App />
    </ErrorBoundary>
  </React.StrictMode>,
);
