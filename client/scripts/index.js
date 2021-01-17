let animation = false;
let start = null;

function getActiveCard(){
return document.querySelector(".card-container.active");
}

function touchUp() {
    getActiveCard().style.transform = "translateX(0px) rotate(0deg)";
    start = null;
}

function cardRemove() {
    start = null;
    getActiveCard().remove();
    document.querySelectorAll(".card-container")[0].setAttribute("class", "card-container active");
    attachEventListeners();
}

function touchMove(e) {
    if (start === null) {
        start = e.touches[0].clientX;
    }
    getActiveCard().style.transform = `translateX(${Math.ceil(e.touches[0].clientX - start)}px) rotate(${Math.ceil(e.touches[0].clientX - start)*0.1}deg)`;

    if ((`${Math.ceil(e.touches[0].clientX - start)}`) > 260) {
        console.log("right");

    } else if ((`${Math.ceil(e.touches[0].clientX - start)}`) < -260) {
        cardRemove();
    }
}

function attachEventListeners() {
    getActiveCard().addEventListener("touchmove", touchMove);
    getActiveCard().addEventListener("touchend", touchUp);
}
attachEventListeners();