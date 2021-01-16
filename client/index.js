function swipeRight(){
    document.getElementById(".card-container").classList.add("rotate-left");//fade ve delay verilecek css yazarken 
    if(document.querySelectorAll(".card-container").lastChild){
        document.getElementById('.card-container').childNodes[length-1].removeClass ('rotate-left rotate-right');
    } else {
        document.querySelectorAll(".card-container").nextElementSibling.remove('rotate-left rotate-right');
    }
}

const activeCard = document.querySelectorAll(".card-container.active")[0];
activeCard.addEventListener("mousedown",swipeRight);
