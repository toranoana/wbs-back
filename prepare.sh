#!/bin/sh

cp .env.sample .env
cp .env.sample .env.local
docker network create wbs_network
