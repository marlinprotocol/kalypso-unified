curl -X POST http://localhost:3000/getPrivInput \
  -H "Content-Type: application/json" \
  -d '{
  "ask_id": "1",
  "ivs_pubkey": "04dea308a0d2b8f80f2304818df0eaea9da11d082e58c5018e04d37662b7db2b051d9475505986ed42d099cecbe6f5e5d824ee272d979fae9803190340420dd399",
  "signature": "eee8c507bdf5609746c1ffed667862e94c0e5c9017ef9f059e99c822d2fc295a05c8c77de461d66e8a4877b7c7a73989ca1e191caba9e56e094ec03ffb02e1ac28"
  }'


  # it.only("Generate some signature", async() => {
  #   const askId = "1";
  #   const signature = await ivsEnclave.signMessage(askId);
  #   const ivs_pubkey = ivsEnclave.getUncompressedPubkey();

  #   console.log({askId, ivs_pubkey, signature})
  # })