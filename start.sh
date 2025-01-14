if [ -d "_public" ] && [ -d "public" ]; then
    rm public -rf
    mv _public public
fi

npm ci ; chmod u+x ./samambaia ; ./samambaia