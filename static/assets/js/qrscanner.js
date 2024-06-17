function onScanSuccess(decodedText, decodedResult) {
    // handle the scanned code as you like, for example:
    console.log(`Code matched = ${decodedText}`, decodedResult);

    let url = document.getElementById('url')
    url.href = decodedText;
    url.innerText = decodedText;

    const rex = /([0-9]*)$/g;
    document.getElementById('id').value = decodedText.match(rex)[0];
    get_item();
}

function get_item() {
    const id = document.getElementById('id').value;
    const xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 200) {
            const responseJsonObj = JSON.parse(this.responseText);

            document.getElementById('description').value = responseJsonObj.description;
        }
    };
    xhttp.open("GET", "api/item/" + id, true);
    xhttp.send();
}

document.getElementById('id').onchange = function () {
    get_item();
}

document.getElementById('save').onclick = function () {
    const id = document.getElementById('id').value;
    const description = document.getElementById('description').value;
    const xhttp = new XMLHttpRequest();
    xhttp.open("POST", "api/item/" + id, true);
    const jsonData = {"id": id, "description": description};

    xhttp.send(JSON.stringify( jsonData ) );
}

function onScanFailure(error) {}

let html5QrcodeScanner = new Html5QrcodeScanner(
    "reader",
    { fps: 10 },
    /* verbose= */ false);
html5QrcodeScanner.render(onScanSuccess, onScanFailure);

document.getElementById('id').value = window.location.hash.substring(1)
get_item()