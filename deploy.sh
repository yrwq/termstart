#!/bin/sh

rsync -av --delete dist/ /tmp/dist_backup/

git checkout pages

rsync -av --delete --exclude='.git' /tmp/dist_backup/ ./
