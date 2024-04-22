add () {
    [[ $# -eq 0 ]] && echo "The sum is 0" && exit 0
    [[ ! $# -eq 1 ]] && echo "Usage: $0 <string>" && exit 1

    numbers=( ${1//[,'\n']/ } )

    sum=0
    for number in ${numbers[@]}; do
        sum=$(( sum + number ))
    done
    echo "The sum is $sum"
}

add $@