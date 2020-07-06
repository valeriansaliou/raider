FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.0

RUN apt-get update && \
    apt-get install -y libmysqlclient20 libmysqlclient-dev && \
    rm -rf /var/lib/apt/lists/*
