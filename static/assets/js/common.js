function delete_item(id, callback) {
    const xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = callback
    xhttp.open("DELETE", "/api/item/" + id, true);
    xhttp.send();
}
