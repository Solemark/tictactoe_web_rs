/**
 * @param {number} num
 * @returns {void}
 */
const send_button = num => {
    let button = document.getElementById(`button_${num}`)
    if (check_space(button.textContent)) {
        console.log(`updating space: ${num}`)

    }
}

/**
 * @param {string} text 
 * @returns {boolean}
 */
const check_space = text => {
    switch(text) {
        case "_":
            return true
        case "X":
            return false
        case "O":
            return false
        default:
            return true
    }
}
