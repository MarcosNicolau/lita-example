import * as valida from "valida-basic-api-wasm";
import programBase64 from "./program.base64";

const asciiToBytesArray = (ascii) => {
    const byteArray = [];
    for (var i = 0; i < ascii.length; i++) {
        byteArray.push(ascii.charCodeAt(i));
    }
    return new Uint8Array(byteArray);
};

const base64ToBytesArray = (base64) => {
    const ascii = atob(base64);
    return asciiToBytesArray(ascii);
};

const bytesToString = (bytes) => {
    var string = "";
    for (var i = 0; i < bytes.length; i++) {
        string += String.fromCharCode(bytes[i]);
    }
    return string;
};

const programBytes = base64ToBytesArray(programBase64);
const inputBytes = asciiToBytesArray("3\n");

const resultBytes = valida.run(programBytes, inputBytes);
const resultString = bytesToString(resultBytes);
console.log(resultString);

const proof = valida.prove(programBytes, inputBytes);
valida.verify(programBytes, resultBytes, proof);
console.log("verified proof");

try {
    valida.verify(programBytes, resultBytes, new Uint8Array(0));
} catch (e) {
    console.log("Failed to verify non-proof: ", e);
}
