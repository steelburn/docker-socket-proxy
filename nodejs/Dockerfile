FROM node:latest

WORKDIR /app

COPY package.json ./
RUN npm install

COPY src/ ./

CMD ["node", "src/index.js"]