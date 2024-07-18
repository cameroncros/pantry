let scanner = new Html5Qrcode("reader");
config = {
    qrbox: { width: 250, height: 250 },
    aspectRatio: 1,
    rememberLastUsedCamera: true,
    // Only support camera scan type.
    supportedScanTypes: [Html5QrcodeScanType.SCAN_TYPE_CAMERA]
}

function onScanSuccess(decodedText, decodedResult) {
    // handle the scanned code as you like, for example:
    console.log(`Code matched = ${decodedText}`, decodedResult);

    let url = document.getElementById('url')
    url.href = decodedText;
    url.innerText = decodedText;

    const rex = /([0-9]*)$/g;
    document.getElementById('id').value = decodedText.match(rex)[0];
    get_item();
    stopScanner();
}

function stopScanner() {
    scanner.stop();
    let button = document.getElementById("button");
    button.innerText = "Start Scan";
    button.onclick = startScanner
}

function get_item() {
    const id = document.getElementById('id').value;
    const xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 200) {
            const responseJsonObj = JSON.parse(this.responseText);

            document.getElementById('description').value = responseJsonObj.description;
            document.getElementById('date').value = responseJsonObj.date;
        }
    };
    xhttp.open("GET", "/api/item/" + id, true);
    xhttp.send();
}

document.getElementById("button").onclick = function startScanner() {
    scanner.start(
        { facingMode: "environment" }, config, onScanSuccess
    )
    let button = document.getElementById("button");
    button.innerText = "Stop Scan";
    button.onclick = stopScanner
}

document.getElementById('id').onchange = function () {
    get_item();
}

document.getElementById("updatedate").onclick = function () {
    document.getElementById("date").valueAsDate = new Date();
}

document.getElementById('save').onclick = function () {
    const id = document.getElementById('id').value;
    const description = document.getElementById('description').value;
    const date = document.getElementById('date').value;
    const xhttp = new XMLHttpRequest();
    xhttp.open("PUT", "/api/item/" + id, true);
    xhttp.setRequestHeader("Content-Type", "application/json")
    const jsonData = {"id": parseInt(id), "description": description, "date": date};

    xhttp.send(JSON.stringify( jsonData ) );
}

document.getElementById('id').value = window.location.hash.substring(1)
get_item()
