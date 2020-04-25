FROM  rust:1.42.0-slim-buster

RUN apt update && apt install -y locales

RUN localedef -i ja_JP -c -f UTF-8 -A /usr/share/locale/locale.alias ja_JP.UTF-8
ENV LANG ja_JP.UTF-8

RUN apt update && apt -y install gnupg2 wget
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -
RUN echo "deb http://apt.postgresql.org/pub/repos/apt/ buster-pgdg main" | \
         tee /etc/apt/sources.list.d/pgdg.list
RUN apt update && apt install -y libpq-dev postgresql-client-12

RUN cargo install systemfd cargo-watch && \
    cargo install diesel_cli --no-default-features --features postgres


# see https://docs.docker.com/compose/startup-order/
ADD wait-for-it.sh /

WORKDIR /app/server

CMD ["/wait-for-it.sh", "db:5432", "--", \
     "systemfd", "--no-pid", "-s", "0.0.0.0:7778", "--", \
     "cargo", "watch", "--watch", "../common", "--watch", ".", \
     "-x", "run"]
