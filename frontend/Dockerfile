# Build stage
FROM node:20-slim AS build

WORKDIR /app

# Install pnpm
RUN npm install -g pnpm

# Copy package files
COPY package.json pnpm-lock.yaml* ./

# Install dependencies
RUN pnpm install

# Copy source
COPY . .

# Build app
RUN pnpm build

# Final stage
FROM node:20-slim

WORKDIR /app

# Copy built files from Nitro
COPY --from=build /app/.output ./.output

# Expose port
EXPOSE 3000

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=3000

# Start the server
CMD ["node", ".output/server/index.mjs"]
