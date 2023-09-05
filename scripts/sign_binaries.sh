#!/bin/bash

##
#  Raider
#
#  Affiliates dashboard
#  Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
#  License: Mozilla Public License v2.0 (MPL v2.0)
##

# Read arguments
while [ "$1" != "" ]; do
    argument_key=`echo $1 | awk -F= '{print $1}'`
    argument_value=`echo $1 | awk -F= '{print $2}'`

    case $argument_key in
        -v | --version)
            # Notice: strip any leading 'v' to the version number
            RAIDER_VERSION="${argument_value/v}"
            ;;
        *)
            echo "Unknown argument received: '$argument_key'"
            exit 1
            ;;
    esac

    shift
done

# Ensure release version is provided
if [ -z "$RAIDER_VERSION" ]; then
  echo "No Raider release version was provided, please provide it using '--version'"

  exit 1
fi

# Define sign pipeline
function sign_for_architecture {
    final_tar="v$RAIDER_VERSION-$1-$2.tar.gz"
    gpg_signer="valerian@valeriansaliou.name"

    gpg -u "$gpg_signer" --armor --detach-sign "$final_tar"
    sign_result=$?

    if [ $sign_result -eq 0 ]; then
        echo "Result: Signed architecture: $1 ($2) for file: $final_tar"
    fi

    return $sign_result
}

# Run sign tasks
ABSPATH=$(cd "$(dirname "$0")"; pwd)
BASE_DIR="$ABSPATH/../"

rc=0

pushd "$BASE_DIR" > /dev/null
    echo "Executing sign steps for Raider v$RAIDER_VERSION..."

    sign_for_architecture "x86_64" "gnu"
    rc=$?

    if [ $rc -eq 0 ]; then
        echo "Success: Done executing sign steps for Raider v$RAIDER_VERSION"
    else
        echo "Error: Failed executing sign steps for Raider v$RAIDER_VERSION"
    fi
popd > /dev/null

exit $rc
