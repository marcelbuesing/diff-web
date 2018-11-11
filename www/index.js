import * as wasm from "diff-web";
//import * as fastdiff from "fast-diff";
import * as JsDiff from "diff";

const benchRunCount = 1;

let in1 = document.getElementById("in-1");
let in2 = document.getElementById("in-2");
let elapsedElem = document.getElementById("elapsed");
let elapsedFastDiffElem = document.getElementById("elapsedFastDiff");
let showDiff = document.getElementById("show-diff");
let runJSDiffCheckbox = document.getElementById("runJSDiff");

runJSDiffCheckbox.addEventListener('change', (event) => runDiff());

const benchDiff = (diffF) => {
    let diff = Array(benchRunCount).fill(1).reduce((acc, i) => {
        const start = performance.now();
        const diff = diffF();
        const end = performance.now();
        const elapsed = end - start;
        return {
            total: acc.total + elapsed,
            diff: diff
        };
    }, { total: 0, diff: "" });

    return {
        average: diff.total / benchRunCount,
        diff: diff.diff
    }
}

const wasmDiff = () => {
    const colors =
        {
            added: { fg: "#34312C", bg: "#daf4cd" },
            added_highlighted: { fg: "#34312C", bg: "#bed8b1" },
            removed: { fg: "#34312C", bg: "#fedfda" },
            removed_highlighted: { fg: "#34312C", bg: "#fdb7b1" },
            same: { fg: "#34312C", bg: "white" }
        };

    const diff = benchDiff(() => wasm.diff(in1.value, in2.value, colors));

    showDiff.innerHTML = diff.diff.replace(/\r?\n/g, '<br />');
    elapsedElem.innerHTML = "⏱ WASM <a href=\"https://crates.io/crates/difference\">difference</a> elapsed ~" + diff.average.toFixed(3) + "ms";
};

const runJSDiff = () => {
    const diff = benchDiff(() => JsDiff.diffLines(in1.value, in2.value));
    elapsedFastDiffElem.innerHTML = "⏱ JS <a href=\"https://www.npmjs.com/package/diff\">diff</a>  elapsed ~" + diff.average.toFixed(3) + "ms";

    const fragment = document.createDocumentFragment();

    diff.diff.forEach(function (part) {
        const color = part.added ? '#daf4cd' :
            part.removed ? '#fedfda' : 'white';
        const span = document.createElement('span');
        span.style.backgroundColor = color;
        span.appendChild(document
            .createTextNode(part.value));
        fragment.appendChild(span);
        fragment.appendChild(document.createElement('br'));
    });

    showDiff.innerHTML = "";
    showDiff.appendChild(fragment);
}


const runDiff = () => {
    wasmDiff();

    if (runJSDiffCheckbox.checked) {
        runJSDiff();
    } else {
        elapsedFastDiffElem.innerText = "⏱ JS   elapsed ~?ms";
    }
}

in1.onkeyup = runDiff;
in2.onkeyup = runDiff;

runDiff();

fetch('https://raw.githubusercontent.com/docker/docker-install/46dc063425ba40e29da650389f99930bea21abab/install.sh')
    .then(in1_response => in1_response.text())
    .then(in1_text => {
        in1.value = in1_text;
        runDiff();
    });
fetch('https://raw.githubusercontent.com/docker/docker-install/5273f654845070b6bb5b2529080f48e7599e4b09/install.sh')
    .then(in2_response => in2_response.text())
    .then(in2_text => {
        in2.value = in2_text;
        runDiff();
    });