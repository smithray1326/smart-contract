const fs = require('fs');
const path = require('path');
const bs58 = require('bs58').default;

// Path to the id.json file
const keypairPath = path.join('/Users/mac/.config/solana/id.json');

// Read the JSON file
fs.readFile(keypairPath, 'utf8', (err, data) => {
  if (err) {
    console.error('Error reading the keypair file:', err);
    return;
  }

  try {
    // Parse the JSON data to get the secret key array
    const secretKeyArray = JSON.parse(data);

    // Create a Uint8Array from the secret key array
    const secretKey = Uint8Array.from(secretKeyArray);

    // Convert the secret key to Base58
    const base58PrivateKey = bs58.encode(secretKey);

    // Output the Base58 encoded private key
    console.log('Base58 Private Key:', base58PrivateKey);
  } catch (parseError) {
    console.error('Error parsing the keypair data:', parseError);
  }
});
