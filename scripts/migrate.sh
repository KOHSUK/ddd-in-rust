#!/bin/sh

sqlx migrate add user --source ./sql

sqlx migrate add extensions --source ./sql