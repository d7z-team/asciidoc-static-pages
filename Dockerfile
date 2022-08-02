FROM asciidoctor/docker-asciidoctor:1.27
COPY . /opt/toolchain
RUN echo 'export PATH=$PATH:/opt/toolchain' >> /etc/profile && . /etc/profile && \
    doc.sh toolchain
