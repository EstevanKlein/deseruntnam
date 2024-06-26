---
id: dev-txn-signing
title: Signing
keywords:
  - signing
  - transitions
  - zilliqa
description: Zilliqa Transaction Signing
---

---

Before sending a transaction, one must first sign it with a **valid private key**. This can be done with one of the numerous SDKs provided by the Zilliqa team and community.

Signing is done against the Protobuf-serialised version of the transaction's contents. This is the reason why all SDKs depend on Protobuf to function. This step is **transparent** to you as a developer.

Example of providing a private key:

=== "JavaScript"

    ```js
    const { Zilliqa } = require("@zilliqa-js/zilliqa");
    const { getAddressFromPrivateKey } = require("@zilliqa-js/crypto");
    const zilliqa = new Zilliqa("https://dev-api.zilliqa.com");

    const PRIVATE_KEY =
      "9afc1a1dab96127e902daaaec1a56c30346f007523c787c3bb62371c0e5a1be7";

    zilliqa.wallet.addByPrivateKey(PRIVATE_KEY);
    ```

=== "Go"

    ```go
    import (
        "github.com/Zilliqa/gozilliqa-sdk/account"
    )

    wallet := account.NewWallet()
    wallet.AddByPrivateKey("e19d05c5452598e24caad4a0d85a49146f7be089515c905ae6a19e8a578a6930")
    ```

=== "Java"

    ```java
    package com.firestack.example;

    import com.firestack.laksaj.account.Wallet;

    public class TransactionOperation {
        public static void main(String[] args) throws IOException, NoSuchAlgorithmException {
            Wallet wallet = new Wallet();
            String ptivateKey = "e19d05c5452598e24caad4a0d85a49146f7be089515c905ae6a19e8a578a6930";
            String address = wallet.addByPrivateKey(ptivateKey);
        }
    }
    ```
