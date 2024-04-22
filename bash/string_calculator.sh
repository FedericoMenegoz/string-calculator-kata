negative_err () {
    (( $# > 0 )) && echo "negatives not allowed: $@" && exit 1
}

add () {
    local delimiter=,
    local numbers=
    local sum=0
    local negatives=()

    # If empty string return 0
    [[ $# -eq 0 ]] && echo "Empty string = 0" && exit 0
    
    # Check args
    [[ ! $# -eq 1 ]] && echo "Usage: $0 <string>" && exit 1
 
    length=${#1}
    numbers=$1;
    # Length of the string without the optional first line
    actual_length=$(( length - 4 ))

    # Get optional delimiter and shift to second line
    [[ $1 =~ ^//* ]] && delimiter=${1:2:1} && numbers=${1:5}

    # If delimiter is a dash check for negative numbers first
    [[ $delimiter == '-' ]] && negatives=$(echo "$numbers" | grep -oE -- '--[0-9]+|^-[0-9]+' ) && negatives=( ${negatives//--/-} )
    negative_err ${negatives[@]}

    # Replace delimiter with space
    numbers=${numbers//[$delimiter]/ }
    sum=0
    
    # Compute the sum and save the negative numbers if any
    for number in ${numbers[@]}; do
        (( number <= 1000 )) && sum=$(( sum + number ))
        (( number < 0  )) && negatives+=($number)
    done

    negative_err $negatives || echo "The sum is $sum"
}

add $@