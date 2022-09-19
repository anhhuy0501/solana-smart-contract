#!/bin/bash

# Init solana
solana config set --url localhost
# First acc
solana-keygen new
#Wrote new keypair to /home/huy/.config/solana/id.json
#============================================================================
#pubkey: B2bXuMLuvc6Fi25NdiCEKNy23KYbs5sAXFRAd5QB3f9x
#============================================================================
#Save this seed phrase and your BIP39 passphrase to recover your new keypair:
#panda number oil prepare merge loop limit mandate cabbage soup antenna crisp
#============================================================================
# Airdrop local acc
solana airdrop 1000

# Second acc
solana-keygen new --no-outfile
#==============================================================================
#pubkey: 4dA8o8YkHahRMvELv7hDnHmbdZtN3CM3caFsR3VFLdYT
#==============================================================================
#Save this seed phrase and your BIP39 passphrase to recover your new keypair:
#actual nose hotel frame offer excuse pattern garage portion tide detect damage
#==============================================================================
# Airdrop second acc
solana airdrop 1000 4dA8o8YkHahRMvELv7hDnHmbdZtN3CM3caFsR3VFLdYT

# Run solana local
solana-test-validator



# Deploy
bash ./run.sh deploy

# Run client
bash ./run.sh client

# Create token
spl-token create-token
#Address:  HNyojvtShydRPLuCugGvUU9X1TiNtwBanDWwf2AfZeq7
#Decimals:  9
#Signature: oXVsDEGhneGRmL1Dhotg1enPvdSdHULuy8khJpc4owQpkQtzkutSqXySpMqQJvvDfJDCegLuJfUN2hR8k8hKPTP


spl-token create-account HNyojvtShydRPLuCugGvUU9X1TiNtwBanDWwf2AfZeq7
#Creating account AyDCxBRHwSgT8CCa3vgzwpA4iCXqHwk1GvCBTrGR4C17
#Signature: 2yvssT1qRshkqKaTfojKpmT4xoFEpKuSLUNSmT9YbTnTf4hUwvApFcfU44NSF1NLokiWuQud53UmHZsAhQErFNge

spl-token mint HNyojvtShydRPLuCugGvUU9X1TiNtwBanDWwf2AfZeq7 1000000
#Minting 1000000 tokens
#  Token: HNyojvtShydRPLuCugGvUU9X1TiNtwBanDWwf2AfZeq7
#  Recipient: AyDCxBRHwSgT8CCa3vgzwpA4iCXqHwk1GvCBTrGR4C17
#Signature: 2jR7b8NMpwAFQZom5rWLnG4ZAoqbsqqbutMWhUntz4Sd7XmGMRgm5mf7uKn8G34mCQqbiuGcH1RoSdagCU45GmTx

spl-token balance HNyojvtShydRPLuCugGvUU9X1TiNtwBanDWwf2AfZeq7
