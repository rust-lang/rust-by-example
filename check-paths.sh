grep -qR ': /' ./examples
if [ "$?" -eq 0 ]; then
    echo "Some links are absolute.."
    grep -R ': /' ./examples
    exit 1
else
    exit 0
fi
