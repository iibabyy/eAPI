# Étape 1 : Construction de l'application
FROM node:18

# Définir le répertoire de travail
WORKDIR /app

# Copier les fichiers de package et installer les dépendances
COPY srcs/package*.json ./

RUN npm install

# Copier le reste des fichiers de l'application
COPY srcs/ .

EXPOSE 5173

# Construire l'application avec Vite
# CMD ["npm", "run", "dev"]