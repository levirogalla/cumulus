FROM node:22-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN rm -rf node_modules package-lock.json
RUN npm i
COPY . .
ARG API
ENV PUBLIC_API_URL=$API
RUN npm run build
RUN npm prune --production

FROM node:22-alpine
WORKDIR /app
COPY --from=builder /app/build build/
COPY --from=builder /app/node_modules node_modules/
COPY package.json .
EXPOSE 3001
ENV HOST=0.0.0.0
ENV PORT=3001
ENV NODE_ENV=production
CMD [ "node", "build" ]