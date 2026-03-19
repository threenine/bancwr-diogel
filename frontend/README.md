# Nuxt Minimal Starter

Look at the [Nuxt documentation](https://nuxt.com/docs/getting-started/introduction) to learn more.

## Quick Start

```bash
# 1. Start backend (port 3000)
cd ../bancwr-diogel-backend
cargo run

# 2. Start frontend (port 3001)
npm run dev

# 3. Open http://localhost:3001
```

## Architecture

- Frontend (Nuxt): http://localhost:3001
- Backend (Rust): http://localhost:3000
- Frontend proxies `/api/*` to backend

## Development

```bash
# Terminal 1: Start backend
cd ../backend
cargo run

# Terminal 2: Start frontend
npm run dev
# Frontend available at http://localhost:3001
```

## Setup

Make sure to install dependencies:

```bash
# npm
npm install

# pnpm
pnpm install

# yarn
yarn install

# bun
bun install
```

## Development Server

Start the development server on `http://localhost:3001`:

```bash
# npm
npm run dev

# pnpm
pnpm dev

# yarn
yarn dev

# bun
bun run dev
```

## Testing

Run unit tests with Vitest:

```bash
# pnpm
pnpm test
```

## Linting

Check code quality with ESLint:

```bash
# pnpm
pnpm lint
pnpm lint:fix
```

## Production

Build the application for production:

```bash
# npm
npm run build

# pnpm
pnpm build

# yarn
yarn build

# bun
bun run build
```

Locally preview production build:

```bash
# npm
npm run preview

# pnpm
pnpm preview

# yarn
yarn preview

# bun
bun run preview
```

Check out the [deployment documentation](https://nuxt.com/docs/getting-started/deployment) for more information.
