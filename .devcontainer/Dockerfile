FROM ubuntu:20.04

RUN apt update

# Install pkg-config, prevent user interaction for select timezone
RUN echo Europe/Amsterdam > /etc/timezone
RUN ln -s /usr/share/zoneinfo/Europe/Amsterdam
RUN export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y pkg-config \
    && dpkg-reconfigure --frontend noninteractive tzdata

RUN apt install -y bash-completion sudo nano curl build-essential libncurses5\
     iputils-ping git libudev-dev openssl libssl-dev wget iproute2 telnet gdb-multiarch

RUN echo "devuser ALL=(ALL) NOPASSWD:ALL">>/etc/sudoers

# for convenient file exchange with the host, use your own host UID here
RUN adduser --uid 1000 --system --group --shell /bin/bash devuser \
    && mkdir /home/devuser/bin \
    && usermod devuser -G root,sudo
USER devuser

RUN pwd
RUN echo "alias ll='ls -lap'" >/home/devuser/.bashrc

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y

RUN export PATH=$HOME/.cargo/bin:$PATH && rustup component add rls rust-src rust-analysis
RUN export PATH=$HOME/.cargo/bin:$PATH && rustup target add thumbv8m.main-none-eabihf
RUN export PATH=$HOME/.cargo/bin:$PATH && rustup target add thumbv8m.main-none-eabi

## Next items only needed when creating a new project using cargo-generate
#RUN export PATH=$HOME/.cargo/bin:$PATH && cargo install flip-link 
#RUN export PATH=$HOME/.cargo/bin:$PATH && cargo install probe-run 
#RUN export PATH=$HOME/.cargo/bin:$PATH && cargo install cargo-generate

