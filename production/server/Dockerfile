FROM debian:stable

RUN apt update && apt install -y locales
RUN localedef -i ja_JP -c -f UTF-8 -A /usr/share/locale/locale.alias ja_JP.UTF-8
ENV LANG ja_JP.UTF-8

RUN apt update && apt install -y libpq-dev

COPY app /app

WORKDIR /app

CMD ["/app/wait-for-it.sh", "db:5432", "--", "/app/start.sh"]
