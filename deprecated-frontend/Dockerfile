FROM node:23

WORKDIR /app

COPY ./requirements/package*.json ./

RUN npm install

# COPY requirements ./

CMD [ "npm", "run", "start" ]