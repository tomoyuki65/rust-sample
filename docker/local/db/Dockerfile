FROM postgres:17.3

ENV LANG ja_JP.utf8

# PostgreSQLの日本語化で「ja_JP.utf8」を使うために必要
RUN apt-get update && \
    apt-get install -y locales && \
    rm -rf /var/lib/apt/lists/* && \
    localedef -i ja_JP -c -f UTF-8 -A /usr/share/locale/locale.alias ja_JP.UTF-8
