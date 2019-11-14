function getLink(codename) {
    for (k=0;k<codename.length;k++) {
        linkFetch(codename[k]);
    }
}
function linkFetch(device) {
    var deviceCodename = device, directLink;
    var link = "https://raw.githubusercontent.com/Cosmic-OS/platform_vendor_ota/pulsar-release/" + device + ".xml";
    $.ajax({
        url: link,
        type: 'get',
        success: function (response) {
            var $doc = $.parseXML(response);
            $data = ($($doc).find('DirectUrl').text());
            directLink = $data;
            $('#' + deviceCodename).attr('href', $data);
        },
        error: function () {
            alert('Server error');
        }
    });
}
var isDevicePage;
function checkThemeStatus() {
    var themeStatus = localStorage.getItem("theme");
    if (themeStatus === "light") {
        applyTheme('light');
    } else {
        applyTheme('dark');
    }
}
function changeAcc(acTheme) {
    if (acTheme === "light"){
        athemeName = "css/accordion_light";
    } else {
        athemeName = "css/accordion_dark";
    }
    document.getElementById("acc_theme").href = athemeName + ".css";
}
function changeTheme() {
    if (themeName === "css/dark"){
        applyTheme('light');
        if (isDevicePage === 'yes') {
            changeAcc('light');
        }
        localStorage.setItem("theme", "light");
    } else {
        applyTheme('dark');
        if (isDevicePage === 'yes') {
            changeAcc('dark');
        }
        localStorage.removeItem("theme");
    }
}
function applyTheme(theme){
    console.log(theme);
    if (theme === 'light') {
        console.log("light, comin' right up");
        themeName = "css/light";
        if (isDevicePage === 'yes') {
            changeAcc('light');
        }
    } else {
        console.log("Dark it is!");
        themeName = "css/dark";
        if (isDevicePage === 'yes') {
            changeAcc('dark');
        }
    }
    document.getElementById("styleSheet").href = themeName + ".css";
}