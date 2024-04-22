add () {

    [[ ! $# -eq 1 ]] && echo "Usage: $0 <string>" && exit 1

    numbers=( ${1//,/ } )

    sum=0
    for number in ${numbers[@]}; do
        sum=$(( sum + number ))
    done
    echo "The sum is $sum"
}

add $@