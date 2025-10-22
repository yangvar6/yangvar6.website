#!/bin/bash

tailwindcss -i style/input.css -o style/output.css --optimize -m --watch &
trunk serve --open &

wait