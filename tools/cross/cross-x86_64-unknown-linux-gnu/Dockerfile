FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.0

RUN apt-get update && \
    apt-get install -y libssl1.0.0 libssl-dev libmysqlclient-dev && \
    rm -rf /var/lib/apt/lists/*
