FROM rust:1.63.0-buster

RUN apt-get update
RUN apt-get upgrade -y
RUN rustup component add clippy

RUN sh -c "$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.1.2/zsh-in-docker.sh)" -- \
    -t robbyrussell \
    -p git \
    -p https://github.com/zsh-users/zsh-autosuggestions \
    -p https://github.com/zsh-users/zsh-completions

WORKDIR /app/

COPY . .

ENTRYPOINT ["tail"]
CMD [ "-f", "/dev/null" ]