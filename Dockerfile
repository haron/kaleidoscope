FROM rust:latest
RUN apt-get -qq update
RUN apt-get -y install librust-clang-sys-dev x11-apps
COPY ./ ./
RUN cargo build
ENV DISPLAY :0
CMD /target/debug/webcam
