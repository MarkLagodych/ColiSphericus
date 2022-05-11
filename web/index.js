import { default as init_wasm, CircleDrawer } from './wasm/wasm.js';

let param_gen_St
let param_gen_Nt
let param_gen_Tt
let param_iter_per_sec
let speed
let time
let param_bounded
let param_wait
let param_hungry
let param_neighbour_limit
let param_freq
let param_dim
let param_zalpha
let param_gen_size_distrib

function isChecked(checkboxId) {
    return document.getElementById(checkboxId).checked;
}

function getNum(inputId) {
    return Number.parseFloat(document.getElementById(inputId).value);
}

function readParams() {
    param_gen_St = isChecked('param_gen_St')
    param_gen_Nt = isChecked('param_gen_Nt')
    param_gen_Tt = isChecked('param_gen_Tt')
    param_gen_size_distrib = isChecked('param_gen_size_distrib')
    param_iter_per_sec = getNum('param_iter_per_sec')
    speed = getNum('speed')
    time = getNum('time')
    param_bounded = isChecked('param_bounded')
    param_wait = isChecked('param_wait')
    param_hungry = isChecked('param_hungry')
    param_neighbour_limit = getNum('param_neighbour_limit')
    param_freq = getNum('param_freq')
    param_dim = getNum('param_dim')
    param_zalpha = isChecked('param_zalpha')

    console.log('Mode: ' + param_dim + 'D')
}

window.onload = () => {
    readParams();

    let popbtn = document.getElementById('popbtn')

    new bootstrap.Popover(
        popbtn,
        {
            html: true,
            content: document.getElementById('popover')
        }
    );

    popbtn.addEventListener('inserted.bs.popover', () => {
        readParams();
    });

    popbtn.addEventListener('hide.bs.popover', () => {
        readParams();
    });

    let form = document.querySelectorAll("form input").forEach((x) => {
        x.oninput = readParams
    })

    let speed_input = document.getElementById('speed');
    speed_input.oninput = () => { speed = Number(speed_input.value) }

    let time_input = document.getElementById('time');
    time_input.oninput = () => { time = Number(time_input.value) }
}

var drawer;

async function load_wasm() {
    await init_wasm();
    drawer = new CircleDrawer();
}
load_wasm();

function step() {
    drawer.draw();
    if (drawer.is_finished())
        window.stopDrawing();
}

function changeStartStopButton(has_started) {
    let startbtn = document.getElementById('startbtn');
    let stopbtn = document.getElementById('stopbtn');
    startbtn.remove();
    stopbtn.remove();

    let widget_cache = document.getElementById('widget_cache');
    let control_panel = document.getElementById('control_panel');

    if (has_started) {
        widget_cache.prepend(startbtn);
        control_panel.prepend(stopbtn);
    } else {
        widget_cache.prepend(stopbtn);
        control_panel.prepend(startbtn);
    }
}

window.drawingIntervalObject = null;
window.startDrawing = function () {
    if (param_iter_per_sec == 0) {
        alert('Кількість ітерацій за секунду не має бути 0!!!');
        return;
    }

    if (param_freq == 0) {
        alert('Частота не має бути 0!!!');
        return;
    }

    if (param_dim < 1 || param_dim > 3) {
        alert('Кількість вимірів не має бути 1, 2 або 3!!!');
        return;
    }

    drawer.set_iter_per_sec(param_iter_per_sec);
    drawer.set_speed(speed);
    drawer.set_time(time * 60); // Minutes -> seconds
    drawer.set_bounded(param_bounded);
    drawer.set_should_wait_until_end(param_wait);
    drawer.set_hungry(param_hungry);
    drawer.set_neighbour_limit(param_neighbour_limit);

    drawer.set_gen_S(param_gen_St);
    drawer.set_gen_N(param_gen_Nt);
    drawer.set_gen_T(param_gen_Tt);

    drawer.set_dimensions(param_dim);
    drawer.set_use_z_alpha(param_zalpha);

    if (window.drawingIntervalObject === null) {
        window.drawingIntervalObject = setInterval(step, 1000. / param_freq);
        changeStartStopButton(true);
    }
};

window.stopDrawing = function () {
    if (window.drawingIntervalObject !== null) {
        clearInterval(window.drawingIntervalObject);
        window.drawingIntervalObject = null;
        changeStartStopButton(false);
    }
};

window.clearDrawing = function () {
    if (confirm("Точно очистити? Все видалиться!"))
        drawer.clear();
};

window.downloadData = function () {
    let need_S = param_gen_St;
    let need_N = param_gen_Nt;
    let need_T = param_gen_Tt;
    let need_D = param_gen_size_distrib;
    let S = drawer.get_data_S();
    let N = drawer.get_data_N();
    let T = drawer.get_data_T();
    let D = drawer.get_data_size_distrib();

    let csv = '';

    csv += '"t"';

    let size_unit = 'м';
    if (param_dim == 2) size_unit += '²';
    if (param_dim >= 3) size_unit += '³';
    size_unit = ' [' + size_unit + ']';

    let size_letter = 'l';
    if (param_dim == 2) size_letter = 'S';
    if (param_dim >= 3) size_letter = 'V';

    if (need_S) csv += ',"' + size_letter + size_unit + '"';
    if (need_N) csv += ',"N активних"';
    if (need_T) csv += ',"T життя [с]"';
    if (need_D) csv += ',"' + size_letter + ' вкінці' + size_unit + '"';
    csv += '\n';

    let dataLength = Math.max(S.length, N.length, T.length);
    for (let t = 0; t < dataLength; t++) {
        csv += t;
        if (need_S) csv += ',' + S[t];
        if (need_N) csv += ',' + N[t];
        if (need_T) csv += ',' + T[t];
        if (need_D && t < D.length) csv += ',' + D[t];
        csv += '\n';
    }

    let downloader = document.getElementById('downloader');
    downloader.href = 'data:attachment/text,' + encodeURI(csv);
    downloader.click();

};