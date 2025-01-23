function get_item() {
    const id = document.getElementById('id').value;
    const xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function () {
        if (this.readyState === 4 && this.status === 200) {
            const responseJsonObj = JSON.parse(this.responseText);

            document.getElementById('description').value = responseJsonObj.description;
            document.getElementById('date').value = responseJsonObj.date;
        }
    };
    xhttp.open("GET", "/api/item/" + id, true);
    xhttp.send();
}

document.getElementById('id').onchange = function () {
    get_item();
}

document.getElementById("today").onclick = function () {
    document.getElementById("date").valueAsDate = new Date();
}

document.getElementById('save').onclick = function () {
    const id = document.getElementById('id').value;
    const description = document.getElementById('description').value;
    show_banner("Saving...");
    setTimeout(() => {
            let date = document.getElementById('date').value;
            if (date === "") {
                date = null;
            }
            const xhttp = new XMLHttpRequest();
            xhttp.open("PUT", "/api/item/" + id, true);
            xhttp.setRequestHeader("Content-Type", "application/json");
            xhttp.onreadystatechange = function () {
                if (this.readyState === 4 && this.status === 202) {
                    const responseJsonObj = JSON.parse(this.responseText);

                    document.getElementById('description').value = responseJsonObj.description;
                    document.getElementById('date').value = responseJsonObj.date;

                    hide_banner("Saved");
                } else {
                    show_banner("Not Saved");
                }
            };
            const jsonData = {"id": parseInt(id), "description": description, "date": date};

            xhttp.send(JSON.stringify(jsonData));
        },
        0.5)
}

document.getElementById('delete').onclick = function () {
    const id = document.getElementById('id').value;
    delete_item(id, () => {
        window.location.href = '/list';
    });
}

if (window.location.hash === "") {
    window.location.href = "/list";
}
document.getElementById('id').value = window.location.hash.substring(1)
get_item()
