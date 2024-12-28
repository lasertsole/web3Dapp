const {
    Connection,
    PublicKey,
    clusterApiUrl,
    Keypair,
    LAMPORTS_PER_SOL,
    Transaction,
    Account,
} = require("@solana/web3.js");

const wallet = new Keypair();

const publicKey = wallet._kepair.publicKey;
const secretKey = wallet._kepair.secretKey;

console.log(publicKey);
console.log(secretKey);