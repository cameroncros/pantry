function delete_item(id, callback) {
    show_banner("Deleting...")

    setTimeout(() => {
        const xhttp = new XMLHttpRequest();
        xhttp.onreadystatechange = () => {
            if (xhttp.readyState === 4 && xhttp.status === 200) {
                hide_banner("Deleted");
                callback()
            } else {
                show_banner("Error");
            }
        }
        xhttp.open("DELETE", "/api/item/" + id, true);
        xhttp.send();
    }, 0.5)

}

function show_banner(text) {
    const banner = document.getElementById("banner");
    banner.innerHTML = `<h2>${text}</h2>`;
    banner.classList.remove('hideme');
    banner.classList.remove('hidden');
    banner.hidden = false;
}

function hide_banner(text) {
    const banner = document.getElementById("banner");
    banner.innerHTML = `<h2>${text}</h2>`;
    banner.classList.add('hideme');
}
