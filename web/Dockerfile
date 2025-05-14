FROM node:18-bookworm-slim AS builder

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build


FROM nginx:1.26-bookworm

RUN rm -rf /usr/share/nginx/html/*

COPY nginx/web.conf /etc/nginx/conf.d/web.conf
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 3001

CMD ["nginx", "-g", "daemon off;"]
