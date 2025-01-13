FROM node:18-slim

WORKDIR /app

COPY requirements/package*.json /app/

RUN npm install --force

RUN npm install react-hook-form && npm install react-router-dom

COPY requirements /app

EXPOSE 3000

CMD [ "npm", "run", "dev" ]