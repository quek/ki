FROM nginx:stable

COPY site.conf etc/nginx/conf.d/default.conf
COPY mime.types /etc/nginx/mime.types
COPY dist /usr/share/nginx/html
