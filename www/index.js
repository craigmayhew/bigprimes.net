import * as wasm from "BigPrimes";

window.onload = function() {
    
    var elBody = document.getElementById("app");
    //todo lets do our routing in JS

    //home page
    elBody.innerHTML = wasm.page_index();
}