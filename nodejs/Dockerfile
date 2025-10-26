FROM node:latest

WORKDIR /app

COPY nodejs/package.json ./
RUN npm install

COPY nodejs/src/ ./

CMD ["node", "src/index.js"]