#! /bin/sh

url=$(curl -s https://api.github.com/repos/jmeischner/todoco/releases/latest \
| grep "browser_download_url.*.ubuntu" \
| cut -d : -f 2,3 \
| tr -d \")

curl $url -Lo "/usr/local/bin/todoco"
chmod +x /usr/local/bin/todoco
