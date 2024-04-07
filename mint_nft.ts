import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { Metaplex, keypairIdentity, bundlrStorage, toMetaplexFile, toBigNumber } from "@metaplex-foundation/js";
import * as fs from 'fs';
// import secret from './wallet-keypair.json';

// const DECNET_RPC = 'https://example.solana-devnet.quiknode.pro/0123456/';
// const SOLANA_CONNECTION = new Connection(DECNET_RPC);
const SOLANA_CONNECTION = new Connection(clusterApiUrl("devnet"));

let secretKey = Uint8Array.from([213,250,198,250,57,113,231,219,112,203,145,138,170,144,202,186,72,149,38,158,81,86,179,46,232,182,225,140,164,121,121,115,114,56,244,34,140,178,16,202,233,117,242,145,50,191,207,71,167,171,168,136,216,211,150,226,197,231,25,160,150,220,4,37]);

const WALLET = Keypair.fromSecretKey(secretKey);
const METAPLEX = Metaplex.make(SOLANA_CONNECTION)
    .use(keypairIdentity(WALLET))
    .use(bundlrStorage({
        address: 'https://devnet.bundlr.network',
        providerUrl: 'https://api.devnet.solana.com',
        timeout: 60000,
    }));

const CONFIG = {
    uploadPath: 'uploads/',
    imgFileName: 'dog.png',
    imgType: 'image/png',
    imgName: 'Dog Test',
    description: 'a dog!!',
    attributes: [],
    sellerFeeBasisPoints: 500,//500 bp = 5%
    symbol: 'DT2',
    creators: [
        {address: WALLET.publicKey, share: 100}
    ]
};
    

async function uploadImage(filePath: string,fileName: string): Promise<string> {
    console.log(`Step 1 - Uploading Image`);
    const imgBuffer = fs.readFileSync(filePath+fileName);
    const imgMetaplexFile = toMetaplexFile(imgBuffer,fileName);
    const imgUri = await METAPLEX.storage().upload(imgMetaplexFile);
    console.log(`   Image URI:`,imgUri);
    return imgUri;
}

async function uploadMetadata(imgUri: string, imgType: string, nftName: string, description: string, attributes: {trait_type: string, value: string}[]) {
    console.log(`Step 2 - Uploading Metadata`);
    const { uri } = await METAPLEX
    .nfts()
    .uploadMetadata({
        name: nftName,
        description: description,
        image: imgUri,
        attributes: attributes,
        properties: {
            files: [
                {
                    type: imgType,
                    uri: imgUri,
                },
            ]
        }
    });
    console.log('   Metadata URI:',uri);
    return uri;    
}

async function mintNft(metadataUri: string, name: string, sellerFee: number, symbol: string, creators: {address: PublicKey, share: number}[]) {
    console.log(`Step 3 - Minting NFT`);
    // const { nft } = await METAPLEX
    // .nfts()
    // .create({
    //     uri: metadataUri,
    //     name: name,
    //     sellerFeeBasisPoints: sellerFee,
    //     symbol: symbol,
    //     // creators: creators,
    //     isMutable: false,
    //     confirmOptions: {
    //         commitment: 'confirmed'
    //       }
    //     // maxSupply: toBigNumber(1)
    // });

    const transactionBuilder = await METAPLEX.nfts().builders().create({
        uri: metadataUri,
        name: name,
        sellerFeeBasisPoints: sellerFee,
        symbol: symbol,
        // creators: creators,
        isMutable: false,
        // maxSupply: toBigNumber(1)
    });
    const { mintAddress } = transactionBuilder.getContext();
    await METAPLEX.rpc().sendAndConfirmTransaction(transactionBuilder);

    // Then, optionally fetch the NFT afterwards after sleeping for 2 seconds. 🤢
    await new Promise(resolve => setTimeout(resolve, 2000))
    const nft = await METAPLEX.nfts().findByMint({ mintAddress });


    console.log(`   Success!🎉`);
    console.log(`   Minted NFT: https://explorer.solana.com/address/${nft.address}?cluster=devnet`);
}

async function main() {
    console.log(`Minting ${CONFIG.imgName} to an NFT in Wallet ${WALLET.publicKey.toBase58()}`);
    //Step 1 - Upload Image
    const imgUri = await uploadImage(CONFIG.uploadPath,CONFIG.imgFileName);
    //Step 2 - Upload Metadata
    const metadataUri = await uploadMetadata(imgUri,CONFIG.imgType,CONFIG.imgName, CONFIG.description, CONFIG.attributes); 
    //Step 3 - Mint NFT
    mintNft(metadataUri,CONFIG.imgName,CONFIG.sellerFeeBasisPoints,CONFIG.symbol,CONFIG.creators);
}

main();


// await metaplex.nfts().mint({
//     nftOrSft: sft,
//     toOwner,
//     amount: token(1),
// });
