# Use an Alpine-based Node.js image for a smaller footprint
FROM node:18-alpine

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy only package.json and package-lock.json first to leverage Docker caching
COPY package*.json ./

# Install dependencies
RUN npm install --production

# Copy the application code
COPY . .

# Expose the port the app runs on
EXPOSE 3277

# Command to run the application
CMD ["node", "index.js"]