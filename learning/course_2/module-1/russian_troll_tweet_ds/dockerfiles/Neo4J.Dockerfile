ARG NEO4J_VERSION=5.15.0
ARG NEO4J_PLUGINS=apoc,graph-data-science
ARG NEO4J_PASSWORD=neo4j
FROM neo4j:${NEO4J_VERSION} AS builder

ENV NEO$J_AUTH=neo4j/${NEO4J_PASSWORD:-neo4j}

# enable plugins 
ENV NEO4J_PLUGINS="[${NEO4J_PLUGINS}]"

# default config env vars, set up physical memory and page cache size
ENV NEO4J_server_memory_pagecache_size=512M
ENV NEO4J_server_memory_heap_max__size=1G
ENV NEO4J_server_memory_heap_initial__size=512M
ENV NEO4J_dbms_security_procedures_unrestricted=apoc.*

# allow import/export file 
ENV NEO4J_apoc_export_file_enabled=true
ENV NEO4J_apoc_import_file_enabled=true


# custom configuration
COPY ../conf/neo4j.conf /var/lib/conf/neo4j.conf

# extension script to inititalze database with custom configuration and plugins
COPY ./neo4j_script_starting.sh /extension_script.sh
RUN chmod +x /extension_script.sh
RUN ./extension_script.sh
ENV EXTENSION_SCRIPT=/extension_script.sh

# expose ports for neo4j
EXPOSE 7474 7473 7678
