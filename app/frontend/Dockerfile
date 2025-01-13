FROM node:18-slim

WORKDIR /app

COPY requirements/package*.json /app/

RUN npm install --force

COPY requirements /app

EXPOSE 3000

CMD [ "npm", "run", "dev" ]