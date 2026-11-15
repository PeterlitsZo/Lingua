function newfilename() {
    perl -e '$f=shift; $l=shift; $f =~ s{\.md\.[^.]+$}{.$l.md}; print $f' \
        -- $1 $2
}

for LANG in zh_CN en_US; do
    for FILENAME in $(ls docs); do
        if [[ $FILENAME == *".lingua" ]]; then
            cargo run -- \
                -v lang=:$LANG \
                -o docs/$(newfilename $FILENAME $LANG) \
                docs/$FILENAME
        fi
    done
done
