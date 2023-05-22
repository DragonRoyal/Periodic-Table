const cursor = document.querySelector(".cursor");
const links = document.querySelectorAll("nav ul li a");
const navlinks = document.querySelectorAll("nav ul li");

document.addEventListener("mousemove", (e) => {
    let leftPosition = e.pageX + 4;
    let topPosition = e.pageY + 4;
    
    // subtract the current scroll position from the mouse position
    leftPosition -= window.pageXOffset;
    topPosition -= window.pageYOffset;

    cursor.style.left = leftPosition + "px";
    cursor.style.top = topPosition + "px";
});

// Add event listeners for links
links.forEach(link => {
    link.addEventListener("mouseenter", () => {
        cursor.classList.add("large");
    });

    link.addEventListener("mouseleave", () => {
        cursor.classList.remove("large");
    });
});

// Set animation delay for nav links
navlinks.forEach((li, i) => {
    li.style.animationDelay = 0 + i * 140 + "ms";
});

