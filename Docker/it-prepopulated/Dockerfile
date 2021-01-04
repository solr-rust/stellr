FROM openjdk:11.0.8-jre-buster

WORKDIR /opt
RUN wget https://archive.apache.org/dist/lucene/solr/8.6.2/solr-8.6.2.tgz \
	&& tar -xzvf solr-8.6.2.tgz \
	&& mv solr-8.6.2 solr \
	&& rm solr-8.6.2.tgz

RUN apt-get update && apt-get install -y lsof procps

RUN mkdir -p /home/solr \
	&& mv /opt/solr/* /home/solr \
	&& useradd -r -u 999 -U solr \
	&& chown -R solr /home/solr
USER solr

RUN /home/solr/bin/solr start -cloud -noprompt \
    && cd /home/solr \
    && bin/solr create_collection -c films \
    && curl http://localhost:8983/solr/films/schema -X POST -H 'Content-type:application/json' --data-binary '{ "add-field" : { "name":"name", "type":"text_general", "multiValued":false, "stored":true }, "add-field" : { "name":"initial_release_date", "type":"pdate", "stored":true } }' \
    && bin/post -c films example/films/films.json \
    && bin/post -c films example/films/films.xml \
    && /home/solr/bin/solr stop -all

WORKDIR /home/solr
EXPOSE 8983
EXPOSE 9983
ENTRYPOINT /home/solr/bin/solr start -c -f
