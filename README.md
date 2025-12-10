# Notes Server

A self-hosted notes and planning server with two main features:
1. **Notes** - Create, edit, and manage text notes
2. **Boards** - Trello-like kanban boards for task planning

## Tech Stack

- **Backend**: Rust (Actix-web) with PostgreSQL
- **Frontend**: Vue 3 + TypeScript + Vite
- **Deployment**: Kubernetes (k3s)

## Features

### Authentication
- Email/password registration and login
- Google OAuth SSO integration
- JWT-based session management
- Per-user data isolation (notes and boards are private to each user)

### Notes
- Create and edit notes with title and content
- Auto-save on blur
- Notes list with search
- Markdown support (planned)

### Boards (Trello-like)
- Create multiple boards with custom colors
- Add lists (columns) to boards
- Create cards with:
  - Title and description
  - Due dates
  - Labels
- Drag and drop cards between lists (planned)

## Quick Start

### Development

```bash
# Start development servers
./dev.sh
```

This starts:
- Backend at http://localhost:8080
- Frontend at http://localhost:3000

### Docker Compose

```bash
# Run with Docker Compose
docker-compose up
```

### Deploy to Kubernetes

```bash
# Build and deploy
make deploy
```

## Project Structure

```
Notes/
├── backend/               # Rust Actix-web API
│   ├── src/
│   │   ├── main.rs       # Entry point
│   │   ├── models.rs     # Data models
│   │   ├── db/           # Database layer
│   │   └── routes/       # API routes
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/              # Vue 3 SPA
│   ├── src/
│   │   ├── main.ts
│   │   ├── App.vue
│   │   ├── api/          # API client
│   │   ├── router/       # Vue Router
│   │   ├── types/        # TypeScript types
│   │   └── views/        # Page components
│   ├── package.json
│   └── Dockerfile
├── k8s/                   # Kubernetes manifests
│   ├── namespace.yaml
│   ├── backend-deployment.yaml
│   ├── frontend-deployment.yaml
│   └── frontend-ingressroute.yaml
├── docker-compose.yaml
├── Makefile
├── dev.sh
└── deploy.sh
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register with email/password
- `POST /api/auth/login` - Login with email/password
- `GET /api/auth/google/url` - Get Google OAuth URL
- `POST /api/auth/google` - Exchange Google OAuth code for token
- `GET /api/auth/me` - Get current user (requires auth)

### Notes
- `GET /api/notes` - List all notes
- `GET /api/notes/:id` - Get a note
- `POST /api/notes` - Create a note
- `PUT /api/notes/:id` - Update a note
- `DELETE /api/notes/:id` - Delete a note

### Boards
- `GET /api/boards` - List all boards
- `GET /api/boards/:id` - Get a board with lists and cards
- `POST /api/boards` - Create a board
- `PUT /api/boards/:id` - Update a board
- `DELETE /api/boards/:id` - Delete a board

### Lists
- `GET /api/boards/:board_id/lists` - Get lists for a board
- `POST /api/boards/:board_id/lists` - Create a list
- `PUT /api/lists/:id` - Update a list
- `DELETE /api/lists/:id` - Delete a list
- `POST /api/lists/reorder` - Reorder lists

### Cards
- `GET /api/lists/:list_id/cards` - Get cards for a list
- `GET /api/cards/:id` - Get a card
- `POST /api/lists/:list_id/cards` - Create a card
- `PUT /api/cards/:id` - Update a card
- `DELETE /api/cards/:id` - Delete a card
- `POST /api/cards/move` - Move card to another list
- `POST /api/cards/reorder` - Reorder cards in a list

## Environment Variables

### Backend
- `DATABASE_URL` - PostgreSQL connection string
- `RUST_LOG` - Log level (default: info)
- `JWT_SECRET` - Secret key for JWT signing (required)
- `GOOGLE_CLIENT_ID` - Google OAuth client ID (optional, for Google SSO)
- `GOOGLE_CLIENT_SECRET` - Google OAuth client secret (optional, for Google SSO)
- `APP_URL` - Application URL for OAuth callbacks (e.g., https://notes.example.com)

### Frontend
- `VITE_API_URL` - Backend API URL (default: /api)

## Google OAuth Setup (Optional)

To enable Google OAuth:

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select existing
3. Enable the Google+ API
4. Go to Credentials → Create Credentials → OAuth Client ID
5. Application type: Web application
6. Add authorized redirect URI: `https://your-domain/auth/google/callback`
7. Copy Client ID and Client Secret to your secrets

## Kubernetes Secrets

Before deploying to Kubernetes, create secrets:

```bash
# Edit k8s/secrets.yaml with your values
kubectl apply -f k8s/secrets.yaml
```

## Development

### Backend
```bash
cd backend
cargo run
```

### Frontend
```bash
cd frontend
npm install
npm run dev
```

## License

MIT
