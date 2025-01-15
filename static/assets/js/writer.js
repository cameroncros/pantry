let support_warning = document.getElementById("support_warning");
let item_index = document.getElementById("item_index");
let button = document.getElementById("flash_btn");

function flash() {
    button.disabled = true;

    let url = window.location.origin + "/#" + item_index.value;

    const ndef = new NDEFReader();
    ndef.write({
        records: [{recordType: "url", data: url}]
    }).then(() => {
        console.log("Message written.");
    }).catch(error => {
        console.log(`Write failed :-( try again: ${error}.`);
    });

    item_index.value -= -1;

    button.disabled = false;
}

if ('NDEFReader' in window) {
    support_warning.style.display = 'none';
    button.onclick = flash()
} else {
    item_index.disabled = true;
    button.disabled = true;
}