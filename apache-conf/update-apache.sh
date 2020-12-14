#!/bin/bash

SOURCE="${BASH_SOURCE[0]}"
while [ -h "$SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
  DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"
  SOURCE="$(readlink "$SOURCE")"
  [[ $SOURCE != /* ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
DIR="$( cd -P "$( dirname "$SOURCE" )" >/dev/null 2>&1 && pwd )"

APPACHEDIR="/etc/apache2/sites-available"


rm $APPACHEDIR/mpri.puyral.ml.conf &&\
    cp $DIR/mpri.puyral.ml.conf $APPACHEDIR/mpri.puyral.ml.conf && \
    a2dissite mpri.puyral.ml.conf && a2ensite mpri.puyral.ml.conf && \
    systemctl reload apache2