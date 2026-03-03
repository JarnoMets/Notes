# Notes – Self-Hosted Notes & Planning Server

A self-hosted productivity suite combining a rich-text note editor, Trello-style kanban boards, a calendar, and visual graph diagrams — all with per-user data isolation and Google SSO.

## Features

### Notes
- Rich-text editor powered by [Tiptap](https://tiptap.dev/) (bold, italic, underline, headings, lists, tables, code blocks, links, images)
- Folder organisation with nested folders
- Full-text search across all notes
- Revision history with diff view
- Attachments (file upload per note)
- Print / export to new tab

### Boards (Kanban)
- Multiple boards with custom colours
- Lists (columns) with drag-and-drop reordering
- Cards with title, description, due dates, labels, and checklists
- Drag-and-drop cards between lists

### Calendar
- Monthly calendar view
- Events linked to board cards or standalone
- Recurring event automations (interval-based triggers)

### Graphs
- Visual node-and-edge diagrams
- Nodes can reference notes, boards, or cards
- Free-form positioning

### Authentication
- Email/password registration and login
- Google OAuth SSO
- JWT session management
- Per-user data isolation

## Tech Stack

| Layer | Technology |
|---|---|
| Backend | Rust · Actix-web 4 · SQLx |
| Database | PostgreSQL 15 |
| Frontend | Vue 3 · TypeScript · Vite · Tiptap · Pinia · VueDraggable |
| Deployment | Docker Compose / Kubernetes (k3s) |

## Quick Start

### Local development

```bash
./dev.sh
```

- Backend: http://localhost:8080
- Frontend: http://localhost:3000

### Docker Compose

```bash
docker-compose up --build
```

### Kubernetes (k3s)

```bash
make deploy
```

## Project Structure

```
Notes/
├── backend/                  # Rust Actix-web API
│   ├── src/
│   │   ├── db/               # Database layer
│   │   ├── models/           # Data models (notes, boards, cards, graphs, …)
│   │   ├── routes/           # HTTP handlers
│   │   │   ├── notes.rs
│   │   │   ├── boards.rs
│   │   │   ├── cards.rs
│   │   │   ├── lists.rs
│   │   │   ├── folders.rs
│   │   │   ├── graphs.rs
│   │   │   ├── automations.rs
│   │   │   ├── reminders.rs
│   │   │   ├── attachments.rs
│   │   │   ├── auth.rs
│   │   │   └── settings.rs
│   │   └── main.rs
│   └── Cargo.toml
├── frontend/                 # Vue 3 SPA
│   ├── src/
│   │   ├── views/            # Top-level page views
│   │   ├── components/       # Feature components (editor, boards, calendar, graphs, …)
│   │   ├── layouts/          # App shell layout
│   │   ├── api/              # Typed API client
│   │   ├── stores/           # Pinia state stores
│   │   └── router/           # Vue Router
│   └── package.json
├── k8s/                      # Kubernetes manifests
├── docker-compose.yaml
├── Makefile
├── dev.sh
└── deploy.sh
```

## API Overview

### Auth
| Method | Path | Description |
|---|---|---|
| POST | `/api/auth/register` | Register with email/password |
| POST | `/api/auth/login` | Login |
| GET | `/api/auth/google/url` | Get Google OAuth URL |
| POST | `/api/auth/google` | Exchange OAuth code for JWT |
| GET | `/api/auth/me` | Current user |

### Notes
| Method | Path | Description |
|---|---|---|
| GET | `/api/notes` | List notes |
| POST | `/api/notes` | Create note |
| GET/PUT/DELETE | `/api/notes/:id` | Get / update / delete note |

### Boards & Cards
| Method | Path | Description |
|---|---|---|
| GET/POST | `/api/boards` | List / create boards |
| GET/PUT/DELETE | `/api/boards/:id` | Board CRUD |
| POST | `/api/boards/:id/lists` | Create list |
| POST | `/api/lists/:id/cards` | Create card |
| POST | `/api/cards/move` | Move card between lists |

## Environment Variables

### Backend

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | local postgres | PostgreSQL connection string |
| `JWT_SECRET` | *(insecure default)* | JWT signing secret — **change in production** |
| `GOOGLE_CLIENT_ID` | – | Google OAuth client ID (optional) |
| `GOOGLE_CLIENT_SECRET` | – | Google OAuth client secret (optional) |
| `APP_URL` | `http://localhost:3000` | Base URL for OAuth callbacks |
| `RUST_LOG` | `info` | Log level |

### Frontend

| Variable | Default | Description |
|---|---|---|
| `VITE_API_URL` | `/api` | Backend API base URL |

## Google OAuth Setup (Optional)

1. Go to [Google Cloud Console](https://console.cloud.google.com/) → Credentials → Create OAuth 2.0 Client
2. Application type: **Web application**
3. Authorised redirect URI: `https://your-domain/auth/google/callback`
4. Set `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET` in your environment / secrets

## Kubernetes Secrets

```bash
kubectl apply -f k8s/secrets.yaml   # edit values first
```

## License

MIT
