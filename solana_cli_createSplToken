命令行操作，查看spl-token命令的帮助文档

## 示例：

设置环境为开发环境：

solana-cli  默认是mainnet，这里设置为devnet

```
solana config set --url https://api.devnet.solana.com
```



创建账号：

```
solana-keygen new --force
```

如果是已经有的账号，可以使用:

```
solana config get  // 检查当前配置
solana config set --keypair <Filepath or URL to a keypair>  // 重新设置当前账号
```



申请水龙头：

```
solana airdrop 1
```





创建Token：

decimals表示精度，enable-metadata表示后续可以设置元数据

```
spl-token create-token --decimals 6 ----enable-metadata
```





设置metadata：

 	<TOKEN_MINT_ADDRESS>      The token address with no metadata present
	<TOKEN_NAME>            			The name of the token to set in metadata
	<TOKEN_SYMBOL>         		   The symbol of the token to set in metadata
	<TOKEN_URI>  						    The URI of the token to set in metadata  （一般是去中心化存储在ar、ipfs等里）

```
spl-token initialize-metadata <TOKEN_MINT_ADDRESS> <TOKEN_NAME> <TOKEN_SYMBOL> <TOKEN_URI>
	
```



创建Token Account:

```
// 为当前账号创建Token（上面新建的）的token account，这个token account可以理解为存储代币信息
// 这里实际上调用了ATA合约，并创建了ATA账号
spl-token create-account <Token address>
```



给自己的这个Token Account发送（mint）

```
spl-token mint  <Token> <amount> <Token Account> 
```



之后就能进行转账了
