FROM node:22-alpine AS build
WORKDIR /usr/src/web/dev
USER web

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY ./web/package.json .

RUN npm install

COPY ./web .

RUN npm run build
RUN npm prune --omit=dev

FROM node:22-alpine
WORKDIR /usr/src/web/prod

COPY --from=build /usr/src/web/dev/build /usr/src/web/prod/build
COPY --from=build /usr/src/web/dev/node_modules /usr/src/web/prod/node_modules

ENV BODY_SIZE_LIMIT="Infinity"

# * Healthcheck
RUN apk add --no-cache curl
HEALTHCHECK CMD curl --fail http://localhost:3000 || exit 1

# run the app
EXPOSE 3000
ENTRYPOINT [ "node", "build" ]