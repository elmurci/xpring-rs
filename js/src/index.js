const net = require('net');
const wallet = require('./wallet');
const utils = require('./utils');
const signer = require('./signer');

function main() {
  // The first argument will be the port used for ipc.
  const port = parseInt(process.argv[2], 10);
  const client = new net.Socket();
  let buffers = [];
  let bufferLength = 0;
  client.connect(port, '127.0.0.1');
  client.on('data', data => {
    buffers.push(data);
    bufferLength += data.length;
    // Data arrives in chunks. Only process if this was the last chunk of the message.
    if (data[data.length - 1] === 0x04) {
      const buffer = Buffer.concat(buffers, bufferLength - 1);
      const message = JSON.parse(buffer);
      buffers = [];
      bufferLength = 0;
      let response = null;
      const funcargs = message.func.split('.').map(item => item.trim());
      try {
        response = [ipcExports[funcargs[0]][funcargs[1]].apply(null, message.arguments), null];
      } catch (e) {
        response = [null, e.message];
      }
      // \0x04 is used as a separator between messages it marks the end of a message.
      client.write(JSON.stringify(response) + '\x04', 'utf-8');
    }
  });
  client.on('close', () => { process.exit(0); });
}

// Export the functions that should be available to rust.
 const ipcExports = {
  wallet,
  utils,
  signer
 };

main();
