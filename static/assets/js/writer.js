let support_warning = document.getElementById("support_warning");
let item_index = document.getElementById("item_index");
let button = document.getElementById("flash_btn");

function flash() {
    button.disabled = true;

    show_banner("Flashing...");

    let url = window.location.origin + "/#" + item_index.value;

    const ndef = new NDEFReader();
    ndef.write({
        records: [{recordType: "url", data: url}]
    }).then(() => {
        item_index.value -= -1;
        show_banner("Message written.");
        button.disabled = false;
    }).catch(error => {
        hide_banner(`Write failed :-( try again: ${error}.`);
        button.disabled = false;
    });

}

if ('NDEFReader' in window) {
    support_warning.style.display = 'none';
    button.onclick = function () {
        flash()
    }
} else {
    item_index.disabled = true;
    button.disabled = true;
}