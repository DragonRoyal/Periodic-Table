#!/bin/bash

count=1
filename="test2.html.tera"

while grep -q "redirectToElement('hydrogen')" "$filename"; do
    gsed -i "0,/redirectToElement('hydrogen')/s//redirectToElement('$count')/" "$filename"
    count=$((count + 1))
done

