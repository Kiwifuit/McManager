FROM eclipse-temurin:�-jre-alpine

WORKDIR /srv/minecraft
COPY dockerfs .

EXPOSE 25565 25575
ENTRYPOINT [ "./run.sh" ]