# Stage 0, "build-stage", based on Node.js, to build and compile the frontend
FROM node:16.13.0 as build-stage
WORKDIR /workspace
COPY . .

# RUN npm install --registry https://registry.npm.taobao.org
RUN npm install

RUN npm run build:h5

FROM nginx:1.23.0-alpine


COPY --from=build-stage /workspace/dist/build/h5  /usr/share/nginx/html

#Copy default nginx configuration
COPY ./nginx-custom.conf /etc/nginx/conf.d/default.conf
