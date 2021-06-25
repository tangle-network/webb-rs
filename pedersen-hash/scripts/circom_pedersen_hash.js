const circomlib = require('circomlib');

function pedersenHash(data) {
  return circomlib.babyJub.unpackPoint(circomlib.pedersenHash.hash(data))[0];
}

const fromHexString = hexString =>
  new Uint8Array(hexString.match(/.{1,2}/g).map(byte => parseInt(byte, 16)));

const inputHex = process.argv[2];
const input = fromHexString(inputHex);
const result = pedersenHash(input);
const output = result.toString(16);
if (output.length % 2 == 0) {
  process.stdout.write(output);
} else {
  process.stdout.write('0' + output);
}

