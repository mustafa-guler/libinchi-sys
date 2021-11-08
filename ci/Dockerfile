FROM centos:8

RUN yum install -y gcc gcc-c++ make cmake clang-devel
# RUN yum install cmake
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > install.sh
RUN sh install.sh -y
ENV PATH=/root/.cargo/bin:$PATH
WORKDIR app
COPY . .
CMD cargo test
