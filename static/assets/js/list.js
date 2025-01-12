function delete_item(id) {
    const xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        get_items();
    };
    xhttp.open("DELETE", "/api/item/" + id, true);
    xhttp.send();
}

function get_items() {
    const xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 200) {
            const responseJsonObj = JSON.parse(this.responseText);
            let table = document.getElementById('all_items');
            table.innerHTML = '';
            responseJsonObj.forEach(item => {
                let row = table.insertRow(-1);
                let id_cell = row.insertCell(-1);
                id_cell.innerText = item.id;
                let date_cell =  row.insertCell(-1);
                date_cell.innerText = item.date;
                let desc_cell =  row.insertCell(-1);
                desc_cell.innerText = item.description;
                let del_cell =  row.insertCell(-1);
                let del_button = document.createElement("a");
                del_button.onclick = function() {
                    delete_item(item.id);
                };
                del_button.innerText = "delete";
                del_cell.appendChild(del_button);
            });
        }
    };
    xhttp.open("GET", "/api/all_items", true);
    xhttp.send();
}

get_items()
